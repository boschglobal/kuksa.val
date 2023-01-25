use std::collections::HashMap;

use chariott_client::chariott::proto::common::{discover_fulfillment, DiscoverFulfillment};
use chariott_client::chariott::proto::common::{fulfillment::Fulfillment, intent::Intent};
use chariott_client::chariott::proto::provider::{
    provider_service_server::ProviderService, FulfillRequest, FulfillResponse,
};
use tonic::async_trait;
use url::Url;

pub struct Provider {
    url: Url,
}

impl Provider {
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}

#[async_trait]
impl ProviderService for Provider {
    async fn fulfill(
        &self,
        request: tonic::Request<FulfillRequest>,
    ) -> Result<tonic::Response<FulfillResponse>, tonic::Status> {
        let response = match request
            .into_inner()
            .intent
            .and_then(|i| i.intent)
            .ok_or_else(|| tonic::Status::invalid_argument("Intent must be specified"))?
        {
            Intent::Discover(_) => Fulfillment::Discover(DiscoverFulfillment {
                services: vec![discover_fulfillment::Service {
                    url: self.url.to_string(),
                    schema_kind: "grpc+proto".to_owned(),
                    schema_reference: "kuksa.val.v1".to_owned(),
                    metadata: HashMap::new(),
                }],
            }),
            /*
            common::intent::Intent::Inspect(inspect) => fulfill(inspect.query, &*VDT_SCHEMA),
            common::intent::Intent::Subscribe(subscribe) => {
                self.streaming_store.subscribe(subscribe)?
            }
            */
            Intent::Read(_read) => unimplemented!(),
            _ => return Err(tonic::Status::unknown("Unknown or unsupported intent!")),
        };

        Ok(tonic::Response::new(FulfillResponse {
            fulfillment: Some(chariott_client::chariott::proto::common::Fulfillment {
                fulfillment: Some(response),
            }),
        }))
    }
}
