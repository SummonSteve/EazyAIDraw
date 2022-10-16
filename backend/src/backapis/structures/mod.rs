pub mod novelai;
use reqwest::{Client, RequestBuilder};

pub trait DrawCall {
    fn into_http_request(&self, client: &Client, api_url: String) -> RequestBuilder;
}

