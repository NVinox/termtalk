use std::net::Ipv4Addr;

use futures_util::{SinkExt, StreamExt};

use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct Websocket {
    host: Ipv4Addr,
    port: u16,
}

impl Websocket {
    pub fn new(host: Ipv4Addr, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn connect(&self) -> (Sender<String>, Receiver<String>) {
        let (ws_stream, _) = connect_async(self.websocket())
            .await
            .expect("Error to connect websocket");
        let (mut ws_tx, mut ws_tr) = ws_stream.split();
        let (tx_to_network, mut tr_from_ui) = mpsc::channel::<String>(100);
        let (tx_to_ui, tr_from_network) = mpsc::channel::<String>(100);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(message) = tr_from_ui.recv() => {
                        let _ = ws_tx.send(Message::Text(message.into())).await;
                    },
                    Some(Ok(message)) = ws_tr.next() => {
                        let _ = tx_to_ui.send(message.to_string()).await;
                    }
                }
            }
        });

        (tx_to_network, tr_from_network)
    }

    fn websocket(&self) -> String {
        format!("ws://{}:{}", self.host, self.port)
    }
}
