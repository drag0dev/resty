use crate::{types::Config, types::Test};
use reqwest::{Response, Client as rClient, Method};

pub struct Client{
    // TODO: keep the session if its configured that way
    pub config: Config,
    pub reqwest_client: rClient,
}

impl Client {
    pub fn new(config: Config) -> Self{
        // TODO: check the default options
        let client = reqwest::Client::new();
        Client{
            config,
            reqwest_client: client,
        }
    }

    pub fn exec_test(self: &Self, test: &Test) -> Response{
        let method = Method::from_bytes(test.method.as_bytes());
        let method = method.unwrap();
        // TODO: form a url using url library with port and endpoints
        let res = self.reqwest_client.request(method, &self.config.base_url)
            .send();


        todo!();
    }
}

