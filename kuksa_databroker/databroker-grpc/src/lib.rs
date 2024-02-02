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

use std::collections::HashMap;

use databroker_proto::sdv::databroker as proto;
use http::Uri;
use kuksa_grpc::{Client as GrpcClient, Error as GrpcError};
pub use kuksa_grpc::{ConnectionState, TokenError};
use tokio_stream::wrappers::BroadcastStream;

pub struct Client {
    grpc_client: GrpcClient,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("connection failed with: {0}")]
    Connection(String),
    #[error("failed with status: {0}")]
    Status(tonic::Status),
    #[error("unkown datapoint")]
    UnknownDatapoint,
    #[error("invalid type")]
    InvalidType,
    #[error("access denied")]
    AccessDenied,
    #[error("internal error")]
    InternalError,
    #[error("out of bounds")]
    OutOfBounds,
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

    pub async fn get_metadata(
        &mut self,
        paths: Vec<String>,
    ) -> Result<Vec<proto::v1::Metadata>, Error> {
        let mut client = proto::v1::broker_client::BrokerClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        // Empty vec == all property metadata
        let args = tonic::Request::new(proto::v1::GetMetadataRequest { names: paths });
        match client.get_metadata(args).await {
            Ok(response) => {
                let message = response.into_inner();
                Ok(message.list)
            }
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn get_datapoints(
        &mut self,
        paths: Vec<impl AsRef<str>>,
    ) -> Result<HashMap<std::string::String, databroker_proto::sdv::databroker::v1::Datapoint>, Error>
    {
        let mut client = proto::v1::broker_client::BrokerClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        let args = tonic::Request::new(proto::v1::GetDatapointsRequest {
            datapoints: paths.iter().map(|path| path.as_ref().into()).collect(),
        });
        match client.get_datapoints(args).await {
            Ok(response) => {
                let message = response.into_inner();
                Ok(message.datapoints)
            }
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn set_datapoints(
        &mut self,
        datapoints: HashMap<String, proto::v1::Datapoint>,
    ) -> Result<proto::v1::SetDatapointsReply, Error> {
        let args = tonic::Request::new(proto::v1::SetDatapointsRequest { datapoints });
        let mut client = proto::v1::broker_client::BrokerClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        match client.set_datapoints(args).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn subscribe(
        &mut self,
        query: String,
    ) -> Result<tonic::Streaming<proto::v1::SubscribeReply>, Error> {
        let mut client = proto::v1::broker_client::BrokerClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );
        let args = tonic::Request::new(proto::v1::SubscribeRequest { query });

        match client.subscribe(args).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(Error::Status(err)),
        }
    }

    pub async fn update_datapoints(
        &mut self,
        datapoints: HashMap<i32, proto::v1::Datapoint>,
    ) -> Result<proto::v1::UpdateDatapointsReply, Error> {
        let mut client = proto::v1::collector_client::CollectorClient::with_interceptor(
            self.grpc_client.get_channel().await?.clone(),
            self.grpc_client.get_auth_interceptor(),
        );

        let request = tonic::Request::new(proto::v1::UpdateDatapointsRequest { datapoints });
        match client.update_datapoints(request).await {
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
