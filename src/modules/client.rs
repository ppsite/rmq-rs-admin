use std::{collections::HashMap, error::Error, time};

use reqwest::{header::HeaderMap, Response};

#[derive(Clone)]
pub struct Client {
    pub host: String,
    pub port: u16,
    pub auth_token: String,
    pub timeout: u8,
}

impl Client {
    pub fn new(host: String, port: u16, auth_token: String, timeout: u8) -> Box<Client> {
        Box::new(Client {
            host,
            port,
            auth_token,
            timeout,
        })
    }

    pub async fn get(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }
        _headers.insert(
            "Authorization",
            format!("Basic {}", self.auth_token).parse()?,
        );

        let url: String = format!("{}:{}/{}", self.host, self.port, uri).parse()?;

        let client = client
            .get(url)
            .headers(_headers)
            .timeout(time::Duration::from_secs(self.timeout.into()))
            .send()
            .await?;
        Ok(client)
    }

    pub async fn post(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
        body: Option<HashMap<String, String>>,
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }
        _headers.insert(
            "Authorization",
            format!("Basic {}", self.auth_token).parse()?,
        );

        let url: String = format!("{}:{}/{}", self.host, self.port, uri).parse()?;

        let mut _body = HashMap::new();
        match body {
            Some(b) => _body.extend(b),
            None => {}
        }
        let response = client
            .post(url)
            .headers(_headers)
            .timeout(time::Duration::from_secs(self.timeout.into()))
            .json(&_body)
            .send()
            .await?;
        Ok(response)
    }
}
