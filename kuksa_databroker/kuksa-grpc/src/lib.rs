/********************************************************************************
* Copyright (c) 2023 Contributors to the Eclipse Foundation
*
* See the NOTICE file(s) distributed with this work for additional
* information regarding copyright ownership.
*
* This program and the accompanying materials are made available under the
* terms of the Apache License 2.0 which is available at
* http://www.apache.org/licenses/LICENSE-2.0
*
* SPDX-License-Identifier: Apache-2.0
********************************************************************************/

use std::convert::TryFrom;

use http::Uri;
use tokio_stream::wrappers::BroadcastStream;
use tonic::transport::Channel;

#[derive(Debug)]
pub struct Client {
    uri: Uri,
    token: Option<tonic::metadata::AsciiMetadataValue>,
    #[cfg(feature = "tls")]
    tls_config: Option<tonic::transport::ClientTlsConfig>,
    channel: Option<tonic::transport::Channel>,
    connection_state_subs: Option<tokio::sync::broadcast::Sender<ConnectionState>>,
}

#[derive(Clone)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
pub enum Error {
    Connection(String),
    Status(tonic::Status),
}

#[derive(Debug)]
pub enum TokenError {
    MalformedTokenError(String),
}

pub fn to_uri(uri: impl AsRef<str>) -> Result<Uri, String> {
    let uri = uri
        .as_ref()
        .parse::<tonic::transport::Uri>()
        .map_err(|err| format!("{err}"))?;
    let mut parts = uri.into_parts();

    if parts.scheme.is_none() {
        parts.scheme = Some("http".parse().expect("http should be valid scheme"));
    }

    match &parts.authority {
        Some(_authority) => {
            // match (authority.port_u16(), port) {
            //     (Some(uri_port), Some(port)) => {
            //         if uri_port != port {
            //             parts.authority = format!("{}:{}", authority.host(), port)
            //                 .parse::<Authority>()
            //                 .map_err(|err| format!("{}", err))
            //                 .ok();
            //         }
            //     }
            //     (_, _) => {}
            // }
        }
        None => return Err("No server uri specified".to_owned()),
    }
    parts.path_and_query = Some("".parse().expect("uri path should be empty string"));
    tonic::transport::Uri::from_parts(parts).map_err(|err| format!("{err}"))
}

impl Client {
    pub fn new(uri: Uri) -> Self {
        Client {
            uri,
            token: None,
            #[cfg(feature = "tls")]
            tls_config: None,
            channel: None,
            connection_state_subs: None,
        }
    }

    pub fn get_uri(&self) -> String {
        self.uri.to_string()
    }

    #[cfg(feature = "tls")]
    pub fn set_tls_config(&mut self, tls_config: tonic::transport::ClientTlsConfig) {
        self.tls_config = Some(tls_config);
    }

    pub fn set_access_token(&mut self, token: impl AsRef<str>) -> Result<(), TokenError> {
        match tonic::metadata::AsciiMetadataValue::try_from(&format!("Bearer {}", token.as_ref())) {
            Ok(token) => {
                self.token = Some(token);
                Ok(())
            }
            Err(err) => Err(TokenError::MalformedTokenError(format!("{err}"))),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.channel.is_some()
    }

    pub fn subscribe_to_connection_state(&mut self) -> BroadcastStream<ConnectionState> {
        match &self.connection_state_subs {
            Some(stream) => BroadcastStream::new(stream.subscribe()),
            None => {
                let (tx, rx) = tokio::sync::broadcast::channel(1);
                self.connection_state_subs = Some(tx);
                BroadcastStream::new(rx)
            }
        }
    }

    async fn try_create_channel(&mut self) -> Result<&Channel, Error> {
        #[cfg(feature = "tls")]
        let mut builder = tonic::transport::Channel::builder(self.uri.clone());
        #[cfg(not(feature = "tls"))]
        let builder = tonic::transport::Channel::builder(self.uri.clone());

        #[cfg(feature = "tls")]
        if let Some(tls_config) = &self.tls_config {
            match builder.tls_config(tls_config.clone()) {
                Ok(new_builder) => {
                    builder = new_builder;
                }
                Err(err) => {
                    return Err(Error::Connection(format!("Failed to configure TLS: {err}")));
                }
            }
        }

        match builder.connect().await {
            Ok(channel) => {
                if let Some(subs) = &self.connection_state_subs {
                    subs.send(ConnectionState::Connected).map_err(|err| {
                        Error::Connection(format!(
                            "Failed to notify connection state change: {err}"
                        ))
                    })?;
                }
                self.channel = Some(channel);
                Ok(self.channel.as_ref().expect("Channel should exist"))
            }
            Err(err) => {
                if let Some(subs) = &self.connection_state_subs {
                    subs.send(ConnectionState::Disconnected).unwrap_or_default();
                }
                Err(Error::Connection(format!(
                    "Failed to connect to {}: {}",
                    self.uri, err
                )))
            }
        }
    }

    pub async fn try_connect(&mut self) -> Result<(), Error> {
        self.try_create_channel().await?;
        Ok(())
    }

    pub async fn try_connect_to(&mut self, uri: tonic::transport::Uri) -> Result<(), Error> {
        self.uri = uri;
        self.try_create_channel().await?;
        Ok(())
    }

    pub async fn get_channel(&mut self) -> Result<&Channel, Error> {
        match self.channel {
            None => self.try_create_channel().await,
            Some(ref channel) => Ok(channel),
        }
    }

    pub fn get_auth_interceptor(
        &self,
    ) -> impl FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> + '_ {
        move |mut req: tonic::Request<()>| {
            if let Some(token) = &self.token {
                // debug!("Inserting auth token: {:?}", token);
                req.metadata_mut().insert("authorization", token.clone());
            }
            Ok(req)
        }
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connection(con) => f.pad(con),
            Error::Status(status) => f.pad(&format!("{status}")),
        }
    }
}

impl std::error::Error for TokenError {}
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::MalformedTokenError(msg) => f.pad(msg),
        }
    }
}
