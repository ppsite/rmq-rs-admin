use std::{collections::HashMap, time};

use reqwest::{header::HeaderMap, Error, Response};

pub struct Client {
    pub host: String,
    pub port: u16,
    pub auth_token: String,
    pub timeout: u8,
}

impl Default for Client {
    fn default() -> Self {
        Client::new(
            "http://127.0.0.1".to_string(),
            15672,
            "Z3Vlc3Q6Z3Vlc3Q=".to_string(),
            5,
        )
    }
}

impl Client {
    pub fn new(host: String, port: u16, auth_token: String, timeout: u8) -> Self {
        Client {
            host,
            port,
            auth_token,
            timeout,
        }
    }

    pub async fn get(&self, uri: String, headers: Option<HeaderMap>) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }
        _headers.insert(
            "Authorization",
            format!("Basic {}", self.auth_token).parse().unwrap(),
        );

        let url: String = format!("{}:{}/{}", self.host, self.port, uri)
            .parse()
            .unwrap();

        let response = client
            .get(url)
            .headers(_headers)
            .timeout(time::Duration::from_secs(self.timeout.into()))
            .send()
            .await;
        return response;
    }

    pub async fn post(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
        body: Option<HashMap<String, String>>,
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }
        _headers.insert(
            "Authorization",
            format!("Basic {}", self.auth_token).parse().unwrap(),
        );

        let url: String = format!("{}:{}/{}", self.host, self.port, uri)
            .parse()
            .unwrap();

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
            .await;
        response
    }
}
