use color_eyre::Result;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use futures_util::{StreamExt, SinkExt};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, time::Duration,
};
use tokio::{net::{TcpListener, TcpStream}, time};
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{info, error, warn, debug};
use uuid::Uuid;
use crate::backapis::{DrawTask, TaskMessage};

use super::handler::Handler as InnerHandler;

type PeerMap = Arc<Mutex<DashMap<SocketAddr, (Uuid, Sender<Message>)>>>;

pub struct Handler {
    peer_map: PeerMap,
    task_sender: Sender<TaskMessage>
}

impl Handler {
    pub fn new(tx: Sender<TaskMessage>) -> Self {
        Self {
            peer_map: Arc::new(Mutex::new(DashMap::new())),
            task_sender: tx,
        }
    }

    pub async fn start_service(&self, port: u16) -> Result<()> {
        let addr = format!("127.0.0.1:{}", port);
        info!("Starting websocket service at {}", addr);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(
                self.peer_map.clone(),
                stream,
                addr,
                self.task_sender.clone()
            ));
        }

        Ok(())
    }


}

async fn handle_connection (
    peer_map: PeerMap,
    stream: TcpStream,
    addr: SocketAddr,
    task_sender: Sender<TaskMessage>
) -> Result<()> {
    info!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("WebSocket connection established: {}", addr);
    let (tx, rx) = crossbeam::channel::unbounded();

    let id = uuid::Uuid::new_v4();
    peer_map.lock().unwrap().insert(addr, (id, tx.clone()));

    let (mut outgoing, mut incoming) = ws_stream.split();

    let timeout_duration = Duration::from_secs(30);
    let mut interval = time::interval(timeout_duration);

    let last_time = Arc::new(Mutex::new(time::Instant::now()));

    let inner_handler = InnerHandler::new(task_sender);

    tokio::spawn(async move {
        loop{
            if let Ok(msg) = rx.recv() {
                outgoing.send(msg).await.unwrap();
            }
        }
    });
    
    loop {
        tokio::select! {
            income_packet = incoming.next() => {
                match income_packet {
                    Some(msg) => match msg {
                        Ok(packet) => {
                            match packet {
                                Message::Close(_) => {
                                    warn!("Closing connection with {}", addr);
                                    break;
                                }
                                _ =>{
                                    debug!("get message {}", addr);
                                    *last_time.lock().unwrap() = time::Instant::now();
                                    inner_handler.handle_packet(packet, tx.clone()).await?
                                }
                            }
                        },
                        Err(e) => {
                            error!("Error while receiving a message: {:?}", e);
                            break;
                        }
                    },
                    None => {}
                }
            }
            _ = interval.tick() => {
                if last_time.lock().unwrap().elapsed() > timeout_duration {
                    //peer_map.lock().unwrap().remove(&addr);
                    //warn!("Client {} is not responding, closing connection", addr);
                    //break;
                }
            }
        }
    }

    info!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
    Ok(())
}
