use crate::hue::requests::*;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::{header, Certificate};
use std::fs;
use std::path::PathBuf;
use crate::hue::error::Error::CertificateError;

#[derive(Debug, Clone)]
pub struct Client {
    client: ReqwestClient,
    url: String,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub url: String,
    pub token: String,
    pub certificate: Option<PathBuf>,
}

impl Client {
    pub fn new(config: ClientConfig) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let builder = ReqwestClient::builder();

        let mut headers = header::HeaderMap::new();
        headers.insert("hue-application-key", header::HeaderValue::from_str(&config.token)?);

        let builder = if let Some(certificate) = config.certificate {
            let cert = match fs::read(certificate) {
                Ok(cert) => cert,
                Err(e) =>  return Err(CertificateError(e.into()).into()),
            };
            
            let cert = Certificate::from_pem(&cert)?;
            builder
                .add_root_certificate(cert)
                .danger_accept_invalid_hostnames(true)
        } else {
            builder
        };

        let builder = builder.default_headers(headers);

        Ok(Client {
            client: builder.build()?,
            url: config.url,
        })
    }

    pub fn send<R: Request>(&self, request: R) -> Result<R::Response> {
        let endpoint = request.endpoint();
        let endpoint = endpoint.trim_matches('/');
        let url = format!("{}/{}", self.url, endpoint);

        self.client
            .request(R::METHOD, &url)
            .send()?
            .error_for_status()
            .and_then(|resp| resp.json())
            .map_err(From::from)
    }
}
