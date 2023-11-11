extern crate reqwest;
use reqwest::blocking::Response;
use serde_json::Value;

const W3WHOST: &str = "https://api.what3words.com/v3";

#[derive(Debug)]
pub enum ResponseFormat {
    Json,
    Geojson,
}

impl ResponseFormat {
    fn to_string(&self) -> &str {
        match self {
            ResponseFormat::Json => "json",
            ResponseFormat::Geojson => "geojson",
        }
    }
}

#[derive(Debug)]
pub struct W3WClient {
    pub api_key: String,
    pub host: String,
    pub language: String,
    pub response_format: ResponseFormat,
    pub client: reqwest::blocking::Client,
}

impl W3WClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            host: W3WHOST.to_string(),
            language: String::from("en"),
            response_format: ResponseFormat::Json,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_request(&self, url: String) -> Result<Response, Response> {
        let resp = self.client.get(url).send();
        let mut response = resp.unwrap();
        response = check_status_code(response)?;
        Ok(response)
    }

    pub fn convert_to_3wa(&self, latitude: &str, longitude: &str) -> Result<Response, Response> {
        let url = format!(
            "{}/convert-to-3wa?key={}&coordinates={},{}&language={}&format={}",
            self.host,
            self.api_key,
            latitude,
            longitude,
            self.language,
            self.response_format.to_string()
        );
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_3wa_json(&self, latitude: &str, longitude: &str) -> Result<Value, Response> {
        let resp = self.convert_to_3wa(latitude, longitude);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_3wa_string(
        &self,
        latitude: &str,
        longitude: &str,
    ) -> Result<String, Response> {
        let resp = self.convert_to_3wa(latitude, longitude);
        let json = get_json(resp)?;
        let result = json["words"].to_string();
        Ok(result)
    }

    pub fn convert_to_coordinates(&self, three_words: &str) -> Result<Response, Response> {
        let url = format!(
            "{}/convert-to-coordinates?words={}&key={}",
            self.host, three_words, self.api_key
        );
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_coordinates_json(&self, three_words: &str) -> Result<Value, Response> {
        let resp = self.convert_to_coordinates(three_words);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_coordinates_floats(&self, three_words: &str) -> Result<(f64, f64), Response> {
        let three_words_json: Value = self.convert_to_coordinates_json(three_words)?;

        let latitude: f64 = match three_words_json["coordinates"]["lat"].as_f64() {
            Some(value) => value,
            None => {
                panic!("Error: latitude is not an f64");
            }
        };
        let longitude: f64 = match three_words_json["coordinates"]["lng"].as_f64() {
            Some(value) => value,
            None => {
                panic!("Error: longitude is not an f64");
            }
        };
        Ok((latitude, longitude))
    }

    pub fn available_languages_json(&self) -> Result<Value, Response> {
        let resp = self.available_languages();
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn available_languages(&self) -> Result<Response, Response> {
        let url = format!("{}/available-languages?key={}", self.host, self.api_key);
        let resp = self.get_request(url);
        resp
    }

    pub fn autosuggest(&self, input: &str) -> Result<Response, Response> {
        let url = format!(
            "{}/autosuggest?input={}&key={}",
            self.host, input, self.api_key
        );
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn autosuggest_json(&self, input: &str) -> Result<Value, Response> {
        let resp = self.autosuggest(input);
        let json = get_json(resp)?;
        Ok(json)
    }
}

fn get_json(resp: Result<Response, Response>) -> Result<Value, Response> {
    let json: Value = resp?.json().unwrap();
    Ok(json)
}

fn check_status_code(response: Response) -> Result<Response, Response> {
    let status_code = response.status();
    if status_code.is_client_error() || status_code.is_server_error() {
        return Err(response);
    }
    Ok(response)
}
