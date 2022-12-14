use serde_derive::Deserialize;
use reqwest::Method;

#[derive(Deserialize, Debug)]
pub struct MasterStruct{
    pub config: Config,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config{
    /// url to which requests are sent
    pub base_url: String,
    /// port to which requests are sent
    pub port: Option<u16>,
    /// pause between each test in ms
    pub pause: Option<u32>,
    /// request timeout
    pub timeout: Option<u64>,
    /// keep the session/cookie if the response has it
    pub keep_session: bool,
    /// are extra headers received ignored
    pub suppress_extra_headers: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Test{
    // request
    pub request_endpoint: String,
    pub request_method: HttpMethod,
    pub request_headers: Option<Vec<Header>>,
    pub request_params: Option<Vec<UrlParams>>,
    pub request_body: Option<String>,

    // response
    pub response_code: u16,
    pub response_body: Option<String>,
    pub response_headers: Option<Vec<Header>>,
}

#[derive(Deserialize, Debug)]
pub struct Header{
    pub header: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct UrlParams{
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub enum HttpMethod{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethod{
    // reqwest inner enum for methods is not public thus the need for this function
    pub fn value(&self) -> Method{
        match *self{
            HttpMethod::GET => Method::GET,
            HttpMethod::HEAD => Method::HEAD,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
            HttpMethod::CONNECT => Method::CONNECT,
            HttpMethod::OPTIONS => Method::OPTIONS,
            HttpMethod::TRACE => Method::TRACE,
            HttpMethod::PATCH => Method::PATCH,
        }
    }
}
