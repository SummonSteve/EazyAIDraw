pub mod draw_result;
pub mod draw_call;

use serde::{Serialize, Deserialize};

use crate::backapis::structures::novelai;
use crate::backapis::structures::stablediffusion;

#[derive(Serialize, Deserialize ,Debug)]
pub enum DrawCallType{
    #[serde(rename = "novel_ai")]
    NovelAi(novelai::GenerateStream),

    #[serde(rename = "sd")]
    Sd(stablediffusion::GenerateStream)
}

#[derive(Serialize, Deserialize ,Debug)]
pub enum IncomePacket {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "draw_call")]
    DrawCall(DrawCallType),
    #[serde(rename = "database_ping")]
    DatabasePing,
    #[serde(rename = "backend_post_progress")]
    BackendPostProgress(BackendProgress),
}


#[derive(Serialize, Deserialize ,Debug)]
pub struct BackendProgress{
    pub id: i64,
    pub current: u16,
    pub total: u16
}


#[derive(Serialize, Deserialize ,Debug)]
pub enum OutcomePacket {
    DrawCallResult,
    DrawProgress,
}
