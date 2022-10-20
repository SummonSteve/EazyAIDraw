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
}




#[derive(Serialize, Deserialize ,Debug)]
pub enum OutcomePacket {
    DrawCallResult,
    DrawProgress,
}
