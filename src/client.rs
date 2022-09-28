use crate::{types::Config, types::Test};
use reqwest::{Response, Client as rClient, Method};
use url::Url;
use anyhow::{Result, Context, anyhow};

pub struct Client{
    // TODO: keep the session if its configured that way
    pub config: Config,
    pub reqwest_client: rClient,
    base_url: Url,
}

impl Client{
    pub fn new(config: &Config) -> Result<Self>{
        // TODO: check the default options
        let client = reqwest::Client::new();
        let mut base_url = Url::parse(&config.base_url).context("Parsing base URL")?;
        if config.port > 0 {
            if base_url.set_port(Some(config.port as u16)).is_err(){
                return Err(anyhow!("Error parsing port")).context("Setting up base url");
            }
        }
        Ok(
            Client{
                config: config.clone(),
                reqwest_client: client,
                base_url,
            })
    }

    pub async fn exec_test(self: &Self, test: &Test) -> Result<Response>{
        let method = Method::from_bytes(test.method.as_bytes()).context("Parsing method for a test")?;
        let mut url = self.base_url.clone();
        url.set_path(&test.end_point);

        let mut request = self.reqwest_client.request(method, self.base_url.as_str());
        for h in test.headers.iter(){
            request = request.header(&h.header, &h.value);
        }
        if test.payload.is_some(){
            let payload = test.payload.clone().unwrap();
            request = request.body(payload);
        }
        let res = request.send().await?;
        Ok(res)
    }
}
