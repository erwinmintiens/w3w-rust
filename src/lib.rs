//! # What3Words Rust Client
//!
//! ## Description
//!
//! This is a Rust client for the What3Words API, allowing you to convert coordinates to three-word addresses and vice versa.
//! This client is based on the provided [API documention](https://developer.what3words.com/public-api/docs) on the What3Words website.
//!
//! ## Features
//!
//! The following endpoints have been implemented:
//! - Convert coordinates to 3words addresses;
//! - Convert 3words addresses to coordinates;
//! - Autosuggest 3words addresses based on given parameters;
//! - Retrieve a list of the coordinates of all what3words squares in a given rectangle which is defined by the coordinates of the southwestern and northeastern points;
//! - Retrieve the available languages and locales.

extern crate reqwest;

mod bounding_box;
mod circle;
mod coordinates;
mod options;
mod polygon;
pub use bounding_box::BoundingBox;
pub use circle::Circle;
pub use coordinates::Coordinates;
pub use options::{
    AutoSuggestOptions, ConvertTo3WAOptions, ConvertToCoordinatesOptions, GridSectionOptions,
};
pub use polygon::Polygon;
use reqwest::blocking::Response;
use serde_json::Value;

const W3WHOST: &str = "https://api.what3words.com/v3";

/// The main client for interacting with the What3Words API.
#[derive(Debug)]
pub struct W3WClient {
    /// Your W3W API key
    pub api_key: String,
    /// The W3W host which defaults to the what3words API endpoint. This is changeable should you
    /// run a W3W endpoint locally
    pub host: String,
    /// The API client
    pub client: reqwest::blocking::Client,
}

impl W3WClient {
    /// Creates a new instance of the What3Words client with the provided API key.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = W3WClient::new("your_api_key");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            host: W3WHOST.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Executes a GET request to the given url
    fn get_request(&self, url: String) -> Result<Response, Response> {
        let resp = self.client.get(url).send();
        let mut response = resp.unwrap();
        response = check_status_code(response)?;
        Ok(response)
    }

    pub fn convert_to_3wa(
        &self,
        coordinates: &Coordinates,
        options: ConvertTo3WAOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-3wa?key={}&coordinates={}",
            self.host,
            self.api_key,
            coordinates.to_string(),
        );
        if let Some(language) = options.language {
            url.push_str(&format!("&language={}", language));
        }
        if let Some(format) = options.format {
            url.push_str(&format!("&format={}", format));
        }
        if let Some(locale) = options.locale {
            url.push_str(&format!("&locale={}", locale));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_3wa_json(
        &self,
        coordinates: &Coordinates,
        options: ConvertTo3WAOptions,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_3wa(coordinates, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_3wa_string(
        &self,
        coordinates: &Coordinates,
        options: ConvertTo3WAOptions,
    ) -> Result<String, Response> {
        let json = self.convert_to_3wa_json(coordinates, options)?;
        let result = json["words"].to_string();
        Ok(result)
    }

    pub fn convert_to_coordinates(
        &self,
        three_words: &str,
        options: ConvertToCoordinatesOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-coordinates?words={}&key={}",
            self.host, three_words, self.api_key
        );
        if let Some(format) = options.format {
            url.push_str(&format!("&format={}", format));
        }
        if let Some(locale) = options.locale {
            url.push_str(&format!("&locale={}", locale));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_coordinates_json(
        &self,
        three_words: &str,
        options: ConvertToCoordinatesOptions,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_coordinates(three_words, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_coordinates_floats(
        &self,
        three_words: &str,
        options: ConvertToCoordinatesOptions,
    ) -> Result<(f64, f64), Response> {
        let three_words_json: Value = self.convert_to_coordinates_json(three_words, options)?;

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

    pub fn autosuggest(
        &self,
        input: &str,
        options: AutoSuggestOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/autosuggest?key={}&input={}",
            self.host, self.api_key, input
        );
        if let Some(focus_coordinates) = options.focus_coordinates {
            url.push_str(&format!("&focus={}", focus_coordinates.to_string()));
        }
        if let Some(circle) = options.circle {
            url.push_str(&format!("&clip-to-circle={}", circle.to_string()));
        }
        if let Some(country_value) = options.country {
            url.push_str(&format!("&clip-to-country={}", country_value));
        }
        if let Some(bounding_box) = options.bounding_box {
            url.push_str(&format!(
                "&clip-to-bounding-box={}",
                bounding_box.to_string()
            ));
        }
        if let Some(polygon) = options.polygon {
            url.push_str(&format!("&clip-to-polygon={}", polygon.to_string()));
        }
        if let Some(language) = options.language {
            url.push_str(&format!("&language={}", language));
        }
        if let Some(prefer_land) = options.prefer_land {
            url.push_str(&format!("&prefer-land={}", prefer_land));
        }
        if let Some(locale) = options.locale {
            url.push_str(&format!("&locale={}", locale));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn autosuggest_json(
        &self,
        input: &str,
        options: AutoSuggestOptions,
    ) -> Result<Value, Response> {
        let resp = self.autosuggest(input, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn grid_section(
        &self,
        bounding_box: &BoundingBox,
        options: GridSectionOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/grid-section?bounding-box={}&key={}",
            self.host,
            bounding_box.to_string(),
            self.api_key
        );
        if let Some(format) = options.format {
            url.push_str(&format!("&format={}", format));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn grid_section_json(
        &self,
        bounding_box: &BoundingBox,
        options: GridSectionOptions,
    ) -> Result<Value, Response> {
        let resp = self.grid_section(bounding_box, options);
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
