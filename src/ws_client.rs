use std::borrow::Cow;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    time::{
        timeout,
        Duration,
    },
};
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::{
        Result,
        Message,
        protocol::frame::CloseFrame,
        protocol::frame::coding::CloseCode,
    },
};
use anyhow::{
    Result as aResult,
    Context,
    anyhow,
};
use colored::Colorize;

use crate::ws_config::Config;
use url::Url;

pub struct ClientWS {
    pub client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub timeout: Duration,
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

        let timeout = Duration::from_millis(config.timeout.unwrap_or(5000)); // default to 5s

        let (socket, _) = connect_async(url)
            .await.context("")?;
        Ok( ClientWS { client: socket, timeout})
    }

    pub async fn text(&mut self, msg: String) -> aResult<Option<Result<Message>>>{
        let msg: Message = Message::Text(msg);
        self.client.send(msg).await?;
        let res = timeout(self.timeout, self.client.next()).await;
        if res.is_err(){
            return Err(anyhow!("timed out waiting for response"));
        }
        Ok(res.unwrap())
    }

    /// send a binary message
    pub async fn binary(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        let msg: Message = Message::Binary(msg);
        self.client.send(msg).await?;
        let res = timeout(self.timeout, self.client.next()).await;
        if res.is_err(){
            return Err(anyhow!("timed out waiting for response"));
        }
        Ok(res.unwrap())
    }

    /// send a ping message
    /// will panic if the message is longer than 128 bytes
    pub async fn ping(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        if msg.len() > 128{
            panic!("{}: ping message payload cannot exceed 128bytes!", "error".bold().red());
        }
        let msg: Message = Message::Ping(msg);
        self.client.send(msg).await?;
        let res = timeout(self.timeout, self.client.next()).await;
        if res.is_err(){
            return Err(anyhow!("timed out waiting for response"));
        }
        Ok(res.unwrap())
    }

    /// send a pong message with binary payload
    /// will panic if the message is longer than 128bytes
    pub async fn pong(&mut self, msg: Vec<u8>) -> aResult<Option<Result<Message>>>{
        if msg.len() > 128{
            panic!("{}: ping message payload cannot exceed 128bytes!", "error".bold().red());
        }
        let msg: Message = Message::Pong(msg);
        self.client.send(msg).await?;
        let res = timeout(self.timeout, self.client.next()).await;
        if res.is_err(){
            return Err(anyhow!("timed out waiting for response"));
        }
        Ok(res.unwrap())
    }

    pub async fn close(&mut self, msg: &str, code: CloseCode) -> aResult<Option<Result<Message>>>{
        let msg = Cow::Borrowed(msg);
        let frame = CloseFrame{code, reason: msg};
        self.client.close(Some(frame)).await?;
        let res = timeout(self.timeout, self.client.next()).await;
        if res.is_err(){
            return Err(anyhow!("timed out waiting for response"));
        }
        Ok(res.unwrap())
    }

    pub async fn close_socket(&mut self) -> Result<()>{
        let frame = CloseFrame{code: CloseCode::Normal, reason: Cow::Borrowed("")};
        self.client.close(Some(frame)).await
    }
}
