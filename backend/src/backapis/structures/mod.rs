pub mod novelai;
pub mod stablediffusion;

use reqwest::{Client, RequestBuilder};

use crate::errors::BackendError;

use super::backend::Backends;

pub trait DrawCall {
    fn into_http_request(&self, client: &Client, backends: &mut Backends) -> Result<RequestBuilder, BackendError> ;
}

