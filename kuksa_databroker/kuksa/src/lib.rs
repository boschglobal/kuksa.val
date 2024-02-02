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

use http::Uri;
use std::collections::HashMap;
use tokio_stream::wrappers::BroadcastStream;

use kuksa_proto::{self as proto, v1::DataEntry};

use kuksa_grpc::{Client as GrpcClient, Error as GrpcError};

pub use kuksa_grpc::{ConnectionState, TokenError};

#[derive(Debug)]
pub struct Client {
    grpc_client: GrpcClient,
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("connection failed: {0}")]
    Connection(String),
    #[error("failed with GRPC status code: {0}")]
    Status(tonic::Status),
    #[error("function returned error")]
    Function(Vec<proto::v1::Error>),
}

impl Client {
    pub fn new(uri: Uri) -> Self {
        Client {
            grpc_client: GrpcClient::new(uri),
        }
    }

    pub fn set_access_token(&mut self, token: impl AsRef<str>) -> Result<(), TokenError> {
        self.grpc_client.set_access_token(token)
    }

    #[cfg(feature = "tls")]
    pub fn set_tls_config(&mut self, tls_config: tonic::transport::ClientTlsConfig) {
        self.grpc_client.set_tls_config(tls_config)
    }

    pub fn subscribe_to_connection_state(&mut self) -> BroadcastStream<ConnectionState> {
        self.grpc_client.subscribe_to_connection_state()
    }

    pub async fn try_connect(&mut self) -> Result<(), Error> {
        Ok(self.grpc_client.try_connect().await?)
    }

    pub fn get_uri(&self) -> String {
        self.grpc_client.get_uri()
    }

    pub fn is_connected(&self) -> bool {
        self.grpc_client.is_connected()
    }

    pub async fn try_connect_to(&mut self, uri: tonic::transport::Uri) -> Result<(), Error> {
        Ok(self.grpc_client.try_connect_to(uri).await?)
    }

    pub async fn set(&mut self, entry: DataEntry, fields: Vec<i32>) -> Result<(), Error> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        let set_request = proto::v1::SetRequest {
            updates: vec![proto::v1::EntryUpdate {
                entry: Some(entry),
                fields: fields,
            }],
        };
        match client.set(set_request).await {
            Ok(response) => {
                let message = response.into_inner();
                let mut errors: Vec<proto::v1::Error> = Vec::new();
                if let Some(err) = message.error {
                    errors.push(err);
                }
                for error in message.errors {
                    if let Some(err) = error.error {
                        errors.push(err);
                    }
                }
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Error::Function(errors))
                }
            }
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn get(
        &mut self,
        path: &str,
        view: proto::v1::View,
        _fields: Vec<i32>,
    ) -> Result<Vec<DataEntry>, Error> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );

        let get_request = proto::v1::GetRequest {
            entries: vec![proto::v1::EntryRequest {
                path: path.to_string(),
                view: view.into(),
                fields: _fields,
            }],
        };

        match client.get(get_request).await {
            Ok(response) => {
                let message = response.into_inner();
                let mut errors = Vec::new();
                if let Some(err) = message.error {
                    errors.push(err);
                }
                for error in message.errors {
                    if let Some(err) = error.error {
                        errors.push(err);
                    }
                }
                if !errors.is_empty() {
                    Err(Error::Function(errors))
                } else {
                    // since there is only one DataEntry in the vector return only the according DataEntry
                    Ok(message.entries.clone())
                }
            }
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn get_metadata(&mut self, paths: Vec<&str>) -> Result<Vec<DataEntry>, Error> {
        let mut metadata_result = Vec::new();

        for path in paths {
            match self
                .get(
                    path,
                    proto::v1::View::Metadata,
                    vec![proto::v1::Field::Metadata.into()],
                )
                .await
            {
                Ok(mut entry) => metadata_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(metadata_result)
    }

    pub async fn get_current_values(
        &mut self,
        paths: Vec<String>,
    ) -> Result<Vec<DataEntry>, Error> {
        let mut get_result = Vec::new();

        for path in paths {
            match self
                .get(
                    &path,
                    proto::v1::View::CurrentValue,
                    vec![
                        proto::v1::Field::Value.into(),
                        proto::v1::Field::Metadata.into(),
                    ],
                )
                .await
            {
                Ok(mut entry) => get_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(get_result)
    }

    pub async fn get_target_values(&mut self, paths: Vec<&str>) -> Result<Vec<DataEntry>, Error> {
        let mut get_result = Vec::new();

        for path in paths {
            match self
                .get(
                    path,
                    proto::v1::View::TargetValue,
                    vec![
                        proto::v1::Field::ActuatorTarget.into(),
                        proto::v1::Field::Metadata.into(),
                    ],
                )
                .await
            {
                Ok(mut entry) => get_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(get_result)
    }

    pub async fn set_current_values(
        &mut self,
        datapoints: HashMap<String, proto::v1::Datapoint>,
    ) -> Result<(), Error> {
        for (path, datapoint) in datapoints {
            match self
                .set(
                    proto::v1::DataEntry {
                        path: path.clone(),
                        value: Some(datapoint),
                        actuator_target: None,
                        metadata: None,
                    },
                    vec![
                        proto::v1::Field::Value.into(),
                        proto::v1::Field::Path.into(),
                    ],
                )
                .await
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub async fn set_target_values(
        &mut self,
        datapoints: HashMap<String, proto::v1::Datapoint>,
    ) -> Result<(), Error> {
        for (path, datapoint) in datapoints {
            match self
                .set(
                    proto::v1::DataEntry {
                        path: path.clone(),
                        value: None,
                        actuator_target: Some(datapoint),
                        metadata: None,
                    },
                    vec![
                        proto::v1::Field::ActuatorTarget.into(),
                        proto::v1::Field::Path.into(),
                    ],
                )
                .await
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub async fn set_metadata(
        &mut self,
        metadatas: HashMap<String, proto::v1::Metadata>,
    ) -> Result<(), Error> {
        for (path, metadata) in metadatas {
            match self
                .set(
                    proto::v1::DataEntry {
                        path: path.clone(),
                        value: None,
                        actuator_target: None,
                        metadata: Some(metadata),
                    },
                    vec![
                        proto::v1::Field::Metadata.into(),
                        proto::v1::Field::Path.into(),
                    ],
                )
                .await
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub async fn subscribe_current_values(
        &mut self,
        paths: Vec<&str>,
    ) -> Result<tonic::Streaming<proto::v1::SubscribeResponse>, Error> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );

        let mut entries = Vec::new();
        for path in paths {
            entries.push(proto::v1::SubscribeEntry {
                path: path.to_string(),
                view: proto::v1::View::CurrentValue.into(),
                fields: vec![
                    proto::v1::Field::Value.into(),
                    proto::v1::Field::Metadata.into(),
                ],
            })
        }

        let req = proto::v1::SubscribeRequest { entries };

        match client.subscribe(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn subscribe(
        &mut self,
        paths: Vec<&str>,
    ) -> Result<tonic::Streaming<proto::v1::SubscribeResponse>, Error> {
        self.subscribe_current_values(paths).await
    }

    pub async fn subscribe_target_values(
        &mut self,
        paths: Vec<&str>,
    ) -> Result<tonic::Streaming<proto::v1::SubscribeResponse>, Error> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        let mut entries = Vec::new();
        for path in paths {
            entries.push(proto::v1::SubscribeEntry {
                path: path.to_string(),
                view: proto::v1::View::TargetValue.into(),
                fields: vec![proto::v1::Field::ActuatorTarget.into()],
            })
        }

        let req = proto::v1::SubscribeRequest { entries };

        match client.subscribe(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn subscribe_metadata(
        &mut self,
        paths: Vec<String>,
    ) -> Result<tonic::Streaming<proto::v1::SubscribeResponse>, Error> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        let mut entries = Vec::new();
        for path in paths {
            entries.push(proto::v1::SubscribeEntry {
                path: path.to_string(),
                view: proto::v1::View::Metadata.into(),
                fields: vec![proto::v1::Field::Metadata.into()],
            })
        }

        let req = proto::v1::SubscribeRequest { entries };

        match client.subscribe(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(Error::Status(err)),
        }
    }
}

impl From<GrpcError> for Error {
    fn from(err: GrpcError) -> Self {
        match err {
            GrpcError::Connection(msg) => Error::Connection(msg),
            GrpcError::Status(status) => Error::Status(status),
        }
    }
}
