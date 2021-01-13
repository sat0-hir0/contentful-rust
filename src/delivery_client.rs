use reqwest::{Error, Response};
use std::future::Future;

pub struct DeliveryClient {
    delivery_api_access_token: String,
    space_id: String,
    base_url: String,
    environment: String,
}

impl DeliveryClient {
    pub fn new(
        environment: &str,
        space_id: &str,
        delivery_api_access_token: &str,
    ) -> DeliveryClient {
        DeliveryClient {
            environment: environment.to_string(),
            space_id: space_id.to_string(),
            base_url: "https://cdn.contentful.com".to_string(),
            delivery_api_access_token: delivery_api_access_token.to_string(),
        }
    }

    // https://www.contentful.com/developers/docs/references/content-delivery-api/#/reference/spaces/space/get-a-space/console/curl
    pub async fn get_space(&self) -> impl Future<Output = Result<Response, Error>> {
        let url = format!(
            "{base_url}/spaces/{space_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
        );

        self.fetch(&url)
    }

    // https://www.contentful.com/developers/docs/references/content-delivery-api/#/reference/content-types/content-model/get-the-content-model-of-a-space/console/curl
    pub async fn get_content_model(&self) -> impl Future<Output = Result<Response, Error>> {
        let url = format!(
            "{base_url}/spaces/{space_id}/environments/{environment}/content_types",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &self.environment,
        );

        self.fetch(&url)
    }

    fn fetch(&self, url: &str) -> impl Future<Output = Result<Response, Error>> {
        let client = reqwest::Client::new();
        client
            .get(url)
            .bearer_auth(&self.delivery_api_access_token)
            .send()
    }
}
