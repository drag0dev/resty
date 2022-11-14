use serde_derive::Deserialize;

// TODO: Close specific test

#[derive(Deserialize, Debug)]
pub struct MasterStruct{
    pub config: Config,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config{
    /// url to which requests are sent
    pub url: String,
    /// port to which request are sent
    pub port: Option<u16>,
    /// timeout between each test in ms
    pub timeout: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Test{
    // send
    /// type of the message that is being sent
    pub send_type: MessageType,
    /// parsed to a corresponding type based on message type
    pub send_data: Option<String>,

    // resp
    pub response_type: Option<MessageType>,
    pub response_data: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType{
    Text, Binary, Ping, Pong, Close,
}

impl std::fmt::Display for MessageType{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            MessageType::Text => write!(f, "Text"),
            MessageType::Binary => write!(f, "Binary"),
            MessageType::Ping => write!(f, "Ping"),
            MessageType::Pong => write!(f, "Pong"),
            MessageType::Close => write!(f, "Close"),
        }
    }
}
