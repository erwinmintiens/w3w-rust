use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const W3WHOST: String = "https://api.what3words.com/v3".to_string();

enum ResponseFormat {
    Json,
    Geojson,
}

impl ResponseFormat {
    fn to_string(&self) -> &'static str {
        match self {
            ResponseFormat::Json => "json",
            ResponseFormat::Geojson => "geojson",
        }
    }
}

pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

impl Coordinates {
    pub fn to_string(&self) -> String {
        format!("{},{}", &self.latitude, &self.longitude)
    }
}

pub struct W3WClient {
    api_key: String,
    host: String,
    language: String,
    response_format: ResponseFormat,
}

impl W3WClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            host: W3WHOST,
            language: String::from("en"),
            response_format: ResponseFormat::Json,
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }

    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }

    pub fn set_response_format(&mut self, response_format: ResponseFormat) {
        self.response_format = response_format;
    }

    pub fn convert_to_3wa(
        &self,
        latitude: &str,
        longitude: &str,
    ) -> Result<ThreeWordAddress, reqwest::Error> {
        let url = format!(
            "{}/convert_to_3wa?key={}&coordinates={},{}&language={}&format={}",
            &self.host,
            &self.api_key,
            latitude,
            longitude,
            &self.language,
            &self.response_format.to_string()
        );
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).send()?;
        response.json()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeWordAddress {
    words: String,
    country: String,
}
