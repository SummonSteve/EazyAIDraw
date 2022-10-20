use rand::Rng;
use reqwest::{Client, RequestBuilder};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateStream {
    
}