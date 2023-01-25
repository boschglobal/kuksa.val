/********************************************************************************
* Copyright (c) 2022 Contributors to the Eclipse Foundation
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

use std::{future::Future, time::Duration};

use tonic::transport::Server;
use tracing::info;

use databroker_proto::{kuksa, sdv};

use crate::{broker, grpc::chariott_v1};

async fn shutdown<F>(databroker: broker::DataBroker, signal: F)
where
    F: Future<Output = ()>,
{
    // Wait for signal
    signal.await;

    info!("Shutting down");
    databroker.shutdown().await;
}

pub async fn serve_with_shutdown<F>(
    addr: &str,
    broker: broker::DataBroker,
    signal: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = ()>,
{
    let addr = addr.parse()?;

    use chariott::proto::runtime_api::{
        intent_registration::Intent, intent_service_registration::ExecutionLocality,
    };
    use chariott_client::chariott;
    let (url, _socket_address) = chariott::provider::register(
        "sdv.kuksa",
        "0.0.1",
        "sdv.kuksa",
        [Intent::Discover, Intent::Read],
        "CHARIOTT_KUKSA_URL",
        format!("http://{addr}").as_str(),
        ExecutionLocality::Local,
    )
    .await?;

    broker.start_housekeeping_task();
    let chariott_provider = chariott_v1::provider::Provider::new(url.clone());

    info!("Listening on {}", addr);
    info!("Chariott service listening on {}", url);
    Server::builder()
        .http2_keepalive_interval(Some(Duration::from_secs(10)))
        .http2_keepalive_timeout(Some(Duration::from_secs(20)))
        .add_service(sdv::databroker::v1::broker_server::BrokerServer::new(
            broker.clone(),
        ))
        .add_service(sdv::databroker::v1::collector_server::CollectorServer::new(
            broker.clone(),
        ))
        .add_service(kuksa::val::v1::val_server::ValServer::new(broker.clone()))
        .add_service(chariott_client::chariott::proto::provider::provider_service_server::ProviderServiceServer::new(
            chariott_provider,
        ))
        .serve_with_shutdown(addr, shutdown(broker, signal))
        .await?;

    Ok(())
}
