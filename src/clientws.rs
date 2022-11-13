use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::{Result, Message},
};
use anyhow::{
    Result as aResult,
    Context,
    anyhow,
};

use crate::wsconfig::Config;
use url::Url;

pub struct ClientWS {
    pub client: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl ClientWS{
    pub async fn new(config: &Config) -> aResult<ClientWS>{
        let mut url = Url::parse(&config.url).context("parsing url")?;
        if url.scheme() != "ws"{
                return Err(anyhow!("url schema must be ws"));
        }
        if config.port.is_some(){
            if url.set_port(config.port).is_err(){
                return Err(anyhow!("error setting port"));
            }
        }
        let (socket, _) = connect_async("ws://localhost:8080/ws_mirror")
            .await.context("")?;
        Ok( ClientWS { client: socket })
    }

    pub async fn text(&mut self, msg: String) -> aResult<Option<Result<Message>>>{
        let msg: Message = Message::Text(msg);
        self.client.send(msg).await?;
        let message_res = self.client.next().await;
        Ok(message_res)
    }

    /// send a binary message
    pub async fn binary(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        let msg: Message = Message::Binary(msg);
        self.client.send(msg).await?;
        let message_res = self.client.next().await;
        Ok(message_res)
    }

    /// send a ping message
    /// will panic if the message is longer than 128 bytes
    pub async fn ping(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        if msg.len() > 128{
            panic!("Ping message payload cannot exceed 128bytes!");
        }
        let msg: Message = Message::Ping(msg);
        self.client.send(msg).await?;
        let message_res = self.client.next().await;
        Ok(message_res)
    }

    /// send a pong message with binary payload
    /// will panic if the message is longer than 128bytes
    pub async fn pong(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        if msg.len() > 128{
            panic!("Pong message paylaod cannot exceed 128bytes!");
        }
        let msg: Message = Message::Pong(msg);
        self.client.send(msg).await?;
        let message_res = self.client.next().await;
        Ok(message_res)
    }

    // TODO: raw frame and close frame
}
