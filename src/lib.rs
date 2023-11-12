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

// #![warn(missing_docs)]
extern crate reqwest;
use std::fmt::format;

use reqwest::blocking::Response;
use serde_json::Value;

const W3WHOST: &str = "https://api.what3words.com/v3";

/// Represents geographical coordinates with latitude and longitude.
pub struct Coordinates {
    /// The latitude value
    pub latitude: f64,
    /// The longitude value
    pub longitude: f64,
}

impl Coordinates {
    fn to_string(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }
}

/// A circle constructed of a centerpoint which has a latitude and longitude and a radius in
/// kilometers.
pub struct Circle {
    /// The latitude value of the centerpoint
    pub latitude: f64,
    /// The longitude value of the centerpoint
    pub longitude: f64,
    /// The radius in kilometers
    pub radius: f64,
}

impl Circle {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.latitude, self.longitude, self.radius)
    }
}

/// A rectangle which is defined by the coordinates of the southwestern point and the coordinates
/// of the northeastern point.
pub struct BoundingBox {
    /// Coordinates of the southwestern point
    pub south_west: Coordinates,
    /// Coordinates of the northeastern point
    pub north_east: Coordinates,
}

impl BoundingBox {
    fn to_string(&self) -> String {
        format!(
            "{},{}",
            self.south_west.to_string(),
            self.north_east.to_string()
        )
    }
}

pub struct Polygon {
    pub coordinates: Vec<Coordinates>,
}

impl Polygon {
    pub fn to_string(&self) -> String {
        let mut url: String = String::new();
        for item in self.coordinates.iter() {
            url.push_str(&format!("{},", &item.to_string()));
        }
        url.push_str(&self.coordinates[0].to_string());
        url
    }
}

#[derive(Debug)]
pub struct ConvertTo3WAOptions<'a> {
    pub language: Option<&'a str>,
    pub format: Option<&'a str>,
    pub locale: Option<&'a str>,
}

impl Default for ConvertTo3WAOptions<'_> {
    fn default() -> Self {
        ConvertTo3WAOptions {
            language: None,
            format: None,
            locale: None,
        }
    }
}

#[derive(Debug)]
pub struct ConvertToCoordinatesOptions<'a> {
    pub format: Option<&'a str>,
    pub locale: Option<&'a str>,
}

impl Default for ConvertToCoordinatesOptions<'_> {
    fn default() -> Self {
        ConvertToCoordinatesOptions {
            format: None,
            locale: None,
        }
    }
}

pub struct AutoSuggestOptions<'a> {
    pub focus_coordinates: Option<&'a Coordinates>,
    pub circle: Option<&'a Circle>,
    pub country: Option<&'a str>,
    pub bounding_box: Option<&'a BoundingBox>,
    pub polygon: Option<&'a Polygon>,
    pub language: Option<&'a str>,
    pub prefer_land: Option<bool>,
    pub locale: Option<&'a str>,
}

impl Default for AutoSuggestOptions<'_> {
    fn default() -> Self {
        AutoSuggestOptions {
            focus_coordinates: None,
            circle: None,
            country: None,
            bounding_box: None,
            polygon: None,
            language: None,
            prefer_land: None,
            locale: None,
        }
    }
}

pub struct GridSectionOptions<'a> {
    pub format: Option<&'a str>,
}

impl Default for GridSectionOptions<'_> {
    fn default() -> Self {
        GridSectionOptions { format: None }
    }
}

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
