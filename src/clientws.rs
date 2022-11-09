use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::{Result, Message},
};
use anyhow::Result as aResult;

// TODO: wait for message, send message for different types

pub struct ClientWS {
    pub client: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl ClientWS{
    pub async fn new() -> ClientWS{
        let (socket, _) = connect_async("ws://localhost:8080/ws_mirror")
            .await.unwrap();
        ClientWS {
            client: socket
        }
    }

    pub async fn send_message(&mut self) -> aResult<Option<Result<Message>>>{
        let floppa: Message = Message::Text("ping".to_string());
        let message_res = self.client.send(floppa).await?;
        let message_res = self.client.next().await;
        Ok(message_res)
    }
}
