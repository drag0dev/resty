use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MasterStruct{
    pub config: Config,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Debug)]
pub struct Config{
    /// url to which requests are sent
    pub base_url: String,
    /// port to which request are sent
    /// -1 if no port is required
    pub port: i32,
    /// timeout between each test in ms
    /// 0 if no timeout
    pub timeout: Option<u128>,
    /// keep the session/cookie if the
    /// respone has it
    /// TODO: jwt/session/cookie?
    pub keep_session: bool,
}

#[derive(Deserialize, Debug)]
pub struct Test{ // request
    pub end_point: String,
    pub method: String,
    pub headers: Vec<Header>,
    pub params: Vec<UrlParams>,
    pub payload: Option<String>,

    // response
    pub response_code: u16,
    pub response_body: Option<String>,
    pub response_headers: Option<Vec<String>>,
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
