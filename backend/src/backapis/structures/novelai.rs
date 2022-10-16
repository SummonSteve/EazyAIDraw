use rand::Rng;
use reqwest::{Client, RequestBuilder};
use serde::{Serialize, Deserialize};

use super::DrawCall;

static UC_DEFAULT: &str = "lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry, lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry";

#[derive(Serialize, Deserialize ,Debug)]
enum StableDiffusionSampler {
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


#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateStream {
    height: i32,
    width: i32,
    n_samples: i32,
    prompt: String,
    sampler: StableDiffusionSampler,
    scale: i32,
    seed: u32,
    steps: i32,
    #[serde(rename = "uc")]
    undesired_content: String,
    #[serde(rename = "ucPreset")]
    uc_preset: i32,
}

impl GenerateStream {
    pub fn new(prompt: String) -> Self {
        Self {
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
    fn into_http_request(&self, client: &Client, api_url: String) -> RequestBuilder {
        client.post(api_url).json(&self)
    }
}