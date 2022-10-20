use std::sync::Arc;
use color_eyre::Result;
use sea_orm::{ConnectionTrait, Statement, EntityTrait, DatabaseConnection};
use tokio_tungstenite::tungstenite::Message;
use crossbeam::channel::Sender;
use tracing::{info, warn, error};
use crate::backapis::{scheduler::TaskScheduler, DrawTask};
use crate::glob::DB;
use crate::session::packets::draw_call::handle_draw_call;
use crate::session::packets::draw_result::DrawResult;
use crate::session::packets::draw_result::TaskStatus;

use super::packets::IncomePacket;

pub struct Handler {
    db: Arc<DatabaseConnection>,
    task_sender: Sender<DrawTask>,
}

impl Handler {
    pub fn new(task_sender: Sender<DrawTask>) -> Self {
        Self {
            db: DB.clone(),
            task_sender,
        }
    }

    pub async fn handle_packet(&self,packet: Message, message_tx: Sender<Message>) -> Result<()>{
        let data = packet.into_data();
        let json_str = String::from_utf8_lossy(&data);
        match serde_json::from_str(&json_str) {
            Ok::<IncomePacket, _>(packet) =>{
                match packet {
                    IncomePacket::DrawCall(draw_call) => {
                        info!("Draw call received");
                        let task_id = handle_draw_call(draw_call, message_tx.clone(), self.task_sender.clone()).await?;
                        let result = DrawResult::new(task_id, data.len() as i32, TaskStatus::Pending);
                        Ok(message_tx.send(Message::Text(serde_json::to_string(&result)?))?)
                    }
                    IncomePacket::Ping => {
                        Ok(message_tx.send(Message::text("Pong"))?)
                    }
                    IncomePacket::DatabasePing => {
                        let res = self.db.execute(
                            Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT 1".to_string()))
                                .await;
                        warn!("Database ping {:?}", res);
                        Ok(())
                    }

                    _ => {Ok(())}
                }
            },
            Err(e) => {
                error!("Error while parsing packet: {}; Raw message: {}", e , json_str);
                Ok(())
            }
        }
        
    
    }
}
