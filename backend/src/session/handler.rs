use sea_orm::{ConnectionTrait, Statement};
use tokio_tungstenite::tungstenite::Message;
use crossbeam::channel::Sender;
use tracing::{info, warn};
use crate::backapis::{DrawCaller, DrawTask};
use crate::glob::DB;

use super::packets::IncomePacket;


pub struct Handler {
    draw_caller: DrawCaller,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            draw_caller: DrawCaller::new(),
        }
    }

    pub async fn handle_packet(&self,packet: Message, tx: Sender<Message>) {
        let data = packet.into_data();
        if data.len() == 0 {
            return;
        }
        let json_str = String::from_utf8_lossy(&data);
        match serde_json::from_str(&json_str) {
            Ok::<IncomePacket, _>(packet) =>{
                match packet {
                    IncomePacket::DrawCall(draw_call) => {
                        info!("Draw call received");

                        //let task = DrawTask::new();

                        //self.draw_caller.add_task();
                    }
                    IncomePacket::Ping => {
                        info!("Ping");
                        tx.send(Message::text("Pong")).unwrap();
                    }
                    IncomePacket::DatabasePing => {
                        let res = DB.clone();
                        let res = res.execute(
                            Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT 1".to_string()))
                                .await;
                        warn!("Database ping {:?}", res);
                    }

                    _ => {}
                }
            },
            Err(e) => {
                info!("Error while parsing packet: {}; Raw message: {}", e , json_str);
                return;
            }
        }
        
    
    }
}
