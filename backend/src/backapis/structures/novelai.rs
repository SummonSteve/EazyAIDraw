use rand::Rng;
use reqwest::{Client, RequestBuilder};
use serde::{Serialize, Deserialize};
use tracing::log::warn;

use crate::backapis::{backend::Backends, BackendType};
use crate::errors::BackendError;
use super::DrawCall;

static UC_DEFAULT: &str = "lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry,";

#[derive(Serialize, Deserialize ,Debug, Clone)]
pub enum StableDiffusionSampler {
    #[serde(rename = "plms")]
    Plms,
    #[serde(rename = "ddim")]
    Ddim,
    #[serde(rename = "k_euler")]
    KEuler,
    #[serde(rename = "k_euler_ancestral")]
    KEulerAncestral,
    #[serde(rename = "k_heun")]
    KHuen,
    #[serde(rename = "k_dpm_2")]
    KDpm2,
    #[serde(rename = "k_dpm_2_ancestral")]
    KDpm2Ancestral,
    #[serde(rename = "k_lms")]
    KLms,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateStream {
    pub id: i64,
    pub height: i32,
    pub width: i32,
    pub n_samples: i32,
    pub prompt: String,
    pub sampler: StableDiffusionSampler,
    pub scale: i32,
    pub seed: u32,
    pub steps: u16,
    #[serde(rename = "uc")]
    pub undesired_content: String,
    #[serde(rename = "uc_preset")]
    pub uc_preset: i32,
}

impl GenerateStream {
    pub fn new(prompt: String) -> Self {
        Self {
            id: 0,
            height: 768,
            width: 512,
            n_samples: 1,
            prompt,
            sampler: StableDiffusionSampler::KEulerAncestral,
            scale: 12,
            seed: rand::thread_rng().gen_range(0..100000), //todo: need change seed range
            steps: 20,
            undesired_content: UC_DEFAULT.to_string(),
            uc_preset: 0,
        }
    }
}

impl DrawCall for GenerateStream {
    fn into_http_request(&self, client: &Client, backends: &mut Backends) -> Result<RequestBuilder, BackendError> {
        for backend in &mut backends.inner {
            if backend.task == None && backend.backend_type == BackendType::NovelAi {
                backend.task = Some(self.id);
                return Ok(client.post(format!("{}/generate-stream",backend.url.to_owned())).json(&self));
            }
        }
        Err(BackendError::NoAvailableBackend)
    }
}