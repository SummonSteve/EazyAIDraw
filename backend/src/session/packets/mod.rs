pub mod draw_result;
pub mod draw_call;

use serde::{Serialize, Deserialize};

use crate::backapis::structures::novelai::GenerateStream;

#[derive(Serialize, Deserialize ,Debug)]
pub enum IncomePacket {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "draw_call")]
    DrawCall(GenerateStream),
    #[serde(rename = "database_ping")]
    DatabasePing,
}




#[derive(Serialize, Deserialize ,Debug)]
pub enum OutcomePacket {
    DrawCallResult,
    DrawProgress,
}
