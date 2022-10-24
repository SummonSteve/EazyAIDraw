pub mod draw_result;
pub mod draw_call;

use std::str;
use serde::{Serialize, Deserialize};
use serde_json::Value;
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
    #[serde(rename = "backend_status_request")]
    BackendStatusRequest,
}


#[derive(Serialize, Deserialize ,Debug)]
pub struct BackendProgress{
    pub id: i64,
    pub current: u16,
    pub total: u16
}


#[derive(Serialize, Deserialize ,Debug)]
pub enum OutcomePacketType {
    #[serde(rename = "draw_call_result")]
    DrawCallResult,
    #[serde(rename = "draw_call_progress")]
    DrawProgress,
    #[serde(rename = "backend_status")]
    BackendStatus,
}


#[derive(Serialize, Deserialize ,Debug)]
pub struct OutcomePacket {
    pub packet_type: OutcomePacketType,
    pub body: Value,
}

impl OutcomePacket {
    pub fn new(packet_type: OutcomePacketType, body: Value) -> Self {
        Self {
            packet_type,
            body
        }
    }
}