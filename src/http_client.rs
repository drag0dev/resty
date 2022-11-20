use crate::{http_config::Config, http_config::Test};
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
        let mut client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_millis(config.timeout.unwrap_or(5000))); // default to 5s
        if config.keep_session{
            client = client.cookie_store(true);
        }

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
        let method = test.request_method.value();
        let mut url = self.base_url.clone();
        url.set_path(&test.request_end_point);

        let mut request = self.reqwest_client.request(method, url.as_str());
        if let Some(req_hed) = &test.request_headers{
            for h in req_hed.iter(){
                request = request.header(&h.header, &h.value);
            }
        }

        if let Some(params) = &test.request_params{
            for p in params.iter(){
                request = request.query(&[(&p.key, &p.value)])
            }
        }


        if test.request_body.is_some(){
            let payload = test.request_body.clone().unwrap();
            request = request.body(payload);
        }

        let res = request.send().await?;
        Ok(res)
    }
}
