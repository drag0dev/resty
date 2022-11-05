use crate::{types::Config, types::Test};
use reqwest::{Response, Client as rClient};
use url::Url;
use anyhow::{Result, Context, anyhow};
use std::time::Duration;

pub struct Client{
    pub config: Config,
    pub reqwest_client: rClient,
    base_url: Url,
}

impl Client{
    pub fn new(config: &Config) -> Result<Self>{
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .cookie_store(true);

        let client = client.build()?;
        let mut base_url = Url::parse(&config.base_url).context("Parsing base URL")?;
        if base_url.set_port(config.port).is_err(){
            return Err(anyhow!("Error parsing port")).context("Setting up base url");
        }
        Ok(
            Client{
                config: config.clone(),
                reqwest_client: client,
                base_url,
            })
    }

    pub async fn exec_test(self: &Self, test: &Test) -> Result<Response>{
        let method = test.method.value();
        let mut url = self.base_url.clone();
        url.set_path(&test.end_point);

        let mut request = self.reqwest_client.request(method, url.as_str());
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
