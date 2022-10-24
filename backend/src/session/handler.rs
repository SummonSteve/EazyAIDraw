use color_eyre::Result;
use tokio_tungstenite::tungstenite::Message;
use crossbeam::channel::Sender;
use tracing::{info, error};
use crate::backapis::TaskMessage;
use crate::glob::STATUS;
use crate::session::packets::draw_call::handle_draw_call;


use super::packets::{IncomePacket, OutcomePacketType, OutcomePacket};

pub struct Handler {
    task_sender: Sender<TaskMessage>,
}

impl Handler {
    pub fn new(task_sender: Sender<TaskMessage>) -> Self {
        Self {
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
                        Ok(handle_draw_call(draw_call, message_tx.clone(), self.task_sender.clone()).await?)
                    }
                    IncomePacket::Ping => {
                        Ok(message_tx.send(Message::text("Pong"))?)
                    }

                    IncomePacket::BackendPostProgress(progress) => {
                        self.task_sender.send(TaskMessage::TaskSyncStatus(progress))?;
                        Ok(())
                    }

                    IncomePacket::BackendStatusRequest => {
                        self.task_sender.send(TaskMessage::RequestSchedulerStatus).unwrap();
                        let status = &*STATUS.lock().unwrap();
                        let status_str = serde_json::to_string(status)?;

                        let res = OutcomePacket::new(OutcomePacketType::BackendStatus, serde_json::from_str(&status_str).unwrap());
                        let status_str = serde_json::to_string(&res)?;
                        Ok(message_tx.send(Message::text(status_str))?)
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
