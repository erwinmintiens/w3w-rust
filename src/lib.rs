//! # What3Words Rust Client
//!
//! ## Description
//!
//! This is a client for the What3Words API, allowing you to convert coordinates to three-word addresses and vice versa.
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
mod coordinate;
mod options;
mod polygon;
pub use bounding_box::BoundingBox;
pub use circle::Circle;
pub use coordinate::Coordinate;
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
    /// run a W3W endpoint locally.
    pub host: String,
    /// The API client
    pub client: reqwest::blocking::Client,
}

impl W3WClient {
    /// Creates a new instance of the What3Words client with the provided API key.
    ///
    /// # Example
    ///
    /// ```
    /// let w3_client = W3WClient::new("your_api_key");
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

    /// Converts a coordinate to a 3word address.
    ///
    /// # Example
    ///
    /// ```
    /// let coordinate = Coordinate {
    ///     latitude: 50.01,
    ///     longitude: 4.53234
    /// }
    /// let resp = w3_client.convert_to_3wa(&coordinate, &ConvertTo3WAOptions::default());
    /// ```
    pub fn convert_to_3wa(
        &self,
        coordinates: &Coordinate,
        options: &ConvertTo3WAOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-3wa?key={}&coordinates={}",
            self.host,
            self.api_key,
            coordinates.to_string(),
        );
        if let Some(language) = options.language {
            url = parse_url(url, "language", language);
        }
        if let Some(format) = options.format {
            url = parse_url(url, "format", format);
        }
        if let Some(locale) = options.locale {
            url = parse_url(url, "locale", locale);
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    /// Converts a coordinate to a 3word address and returns the JSON body.
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coordinate {
    ///     latitude: 50.0012,
    ///     longitude: -3.23
    /// }
    /// let resp_json = w3_client.convert_to_3wa_json(&coordinate, &ConvertTo3WAOptions::default());
    /// ```
    ///
    /// Different options can be added to the call:
    ///
    /// ```
    /// let options = ConvertTo3WAOptions {
    ///     language: Some("nl"),
    ///     ..Default::default()
    /// }
    /// let resp_json = w3_client.convert_to_3wa_json(&coordinate, &options);
    /// ```
    pub fn convert_to_3wa_json(
        &self,
        coordinates: &Coordinate,
        options: &ConvertTo3WAOptions,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_3wa(coordinates, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    /// Convert a coordinate to a 3word address and return the string.
    ///
    /// # Example
    ///
    /// ```
    /// let coordinate = Coordinate {
    ///     latitude: 50.0012,
    ///     longitude: -3.23
    /// }
    /// let resp_string = w3_client.convert_to_3wa_string(&coordinate,
    /// ConvertTo3WAOptions::default());
    /// ```
    pub fn convert_to_3wa_string(
        &self,
        coordinates: &Coordinate,
        options: &ConvertTo3WAOptions,
    ) -> Result<String, Response> {
        let json = self.convert_to_3wa_json(coordinates, options)?;
        let result = json["words"].to_string();
        Ok(result)
    }

    /// Convert a 3word address to a coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// let three_word_address = "fight.offer.airbag";
    /// let resp = w3_client.convert_to_coordinates(three_word_address,
    /// ConvertToCoordinatesOptions::default());
    /// ```
    pub fn convert_to_coordinates(
        &self,
        three_words: &str,
        options: &ConvertToCoordinatesOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-coordinates?words={}&key={}",
            self.host, three_words, self.api_key
        );
        if let Some(format) = options.format {
            url = parse_url(url, "format", format);
        }
        if let Some(locale) = options.locale {
            url = parse_url(url, "locale", locale);
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    /// Convert a 3word address to a coordinate and fetch the JSON body from the response.
    ///
    /// # Example
    ///
    /// ```
    /// let three_word_address = "fight.offer.airbag";
    /// let options = ConvertToCoordinatesOptions {
    ///     format: Some("geojson"),
    ///     ..Default::default()
    /// }
    /// let resp_json = w3_client.convert_to_coordinates_json(three_word_address, &options)?;
    /// ```
    pub fn convert_to_coordinates_json(
        &self,
        three_words: &str,
        options: &ConvertToCoordinatesOptions,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_coordinates(three_words, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    /// Convert a 3word address to a coordinate and fetch the latitude and longitude.
    ///
    /// # Example
    ///
    /// ```
    /// let three_word_address = "fight.offer.airbag";
    /// let resp_coordinate = w3_client.convert_to_coordinates_and_get_coordinate(three_word_address,
    /// ConvertToCoordinatesOptions::default());
    /// ```
    pub fn convert_to_coordinates_and_get_coordinate(
        &self,
        three_words: &str,
        options: &ConvertToCoordinatesOptions,
    ) -> Result<Coordinate, Response> {
        let three_words_json: Value = self.convert_to_coordinates_json(three_words, options)?;
        let latitude: f64 = three_words_json["coordinates"]["lat"]
            .as_f64()
            .expect("Failed to parse JSON latitude to f64");
        let longitude: f64 = three_words_json["coordinates"]["lng"]
            .as_f64()
            .expect("Failed to parse JSON longitude to f64");
        Ok(Coordinate {
            latitude,
            longitude,
        })
    }

    /// Get all available languages and locales.
    ///
    /// # Examples
    ///
    /// ```
    /// let languages_resp = w3_client.available_languages();
    /// ```
    pub fn available_languages(&self) -> Result<Response, Response> {
        let url = format!("{}/available-languages?key={}", self.host, self.api_key);
        let resp = self.get_request(url);
        resp
    }

    /// Get all available languages and locales response JSON body.
    ///
    /// # Example
    ///
    /// ```
    /// let languages_resp = w3_client.available_languages_json();
    /// ```
    pub fn available_languages_json(&self) -> Result<Value, Response> {
        let resp = self.available_languages();
        let json = get_json(resp)?;
        Ok(json)
    }

    /// Autosuggest 3word addresses based on provided parameters.
    ///
    /// # Examples
    ///
    /// ## No extra options
    ///
    /// ```
    /// let incomplete_three_words: &str = "fight.offer.ai";
    /// let autosuggest_resp = w3_client.autosuggest(incomplete_three_words,
    /// &AutoSuggestOptions::default());
    /// ```
    ///
    /// ## Focus coordinates
    ///
    /// Get autosuggstions in order, based on the provided focus point.
    ///
    /// ```
    /// let coordinates = Coordinate{
    ///     latitude: 51.0,
    ///     longitude: 4.0
    /// };
    /// let options = AutoSuggestOptions {
    ///     focus_coordinates: Some(&coordinates),
    ///     ..Default::default()
    /// };
    /// let autosuggest_resp = w3_client.autosuggest(incomplete_three_words, &options);
    /// ```
    ///
    /// ## Circle
    ///
    /// Get autosuggestions within a given circle.
    ///
    /// ```
    /// let coordinates = Coordinate{
    ///     latitude: 51.0,
    ///     longitude: 4.0
    /// };
    /// let circle = Circle {
    ///     centerpoint: &coordinates,
    ///     radius: 35.0
    /// };
    /// let options = AutoSuggestOptions {
    ///     circle: Some(&circle),
    ///     ..Default::default()
    /// };
    /// let autosuggest_resp = w3_client.autosuggest(incomplete_three_words, &options);
    /// ```
    ///
    /// ## Countries
    ///
    /// Restricts AutoSuggest to only return results inside the countries specified by
    /// comma-separated list of uppercase ISO 3166-1 alpha-2 country codes
    /// (for example, to restrict to Belgium and the UK, use clip-to-country=GB,BE).
    /// Clip-to-country will also accept lowercase country codes. Entries must be two a-z letters.
    /// WARNING: If the two-letter code does not correspond to a country, there is no error:
    /// API simply returns no results.
    ///
    /// ```
    /// let countries = vec!["GB", "BE"];
    /// let options = AutoSuggestOptions {
    ///     countries: Some(&countries),
    ///     ..Default::default()
    /// };
    /// let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
    /// ```
    ///
    /// ## BoundingBox
    ///
    /// Restrict AutoSuggest results to a bounding box, specified by coordinates.
    /// Coordinate(south_lat,west_lng),Coordinate(north_lat,east_lng), where:
    /// south_lat less than or equal to north_latwest_lng less than or equal to east_lng.
    /// In other words, latitudes and longitudes should be specified order of increasing size.
    /// Lng is allowed to wrap, so that you can specify bounding boxes which cross
    /// the ante-meridian: -4,178.2,22,195.4
    ///
    /// ```
    /// let coordinate_sw = Coordinate {
    ///     latitude: -4.0,
    ///     longitude: 178.2
    /// };
    /// let coordinate_ne = Coordinate {
    ///     latitude: 22.0,
    ///     longitude: 195.4
    /// };
    /// let bounding_box = BoundingBox {
    ///     south_west: &coordinate_sw,
    ///     north_east: &coordinate_ne
    /// };
    /// let options = AutoSuggestOptions {
    ///     bounding_box: Some(&bounding_box),
    ///     ..Default::default()
    /// };
    /// let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
    /// ```
    ///
    /// ## Polygon
    ///
    /// Restrict AutoSuggest results to a polygon, specified by a comma-separated list of lat,lng pairs.
    /// The API is currently limited to accepting up to 25 pairs.
    ///
    /// ```
    /// let coordinates1 = Coordinate {
    ///     latitude: 51.521,
    ///     longitude: -0.343,
    /// };
    /// let coordinates2 = Coordinate {
    ///     latitude: 52.6,
    ///     longitude: 2.3324,
    /// };
    /// let coordinates3 = Coordinate {
    ///     latitude: 54.234,
    ///     longitude: 8.343,
    /// };
    /// let polygon: Polygon = Polygon {
    ///     coordinates: vec![&coordinates1, &coordinates2, &coordinates3],
    /// };
    /// let options = AutoSuggestOptions {
    ///     polygon: Some(&polygon),
    ///     ..Default::default()
    /// };
    /// let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
    /// ```
    pub fn autosuggest(
        &self,
        input: &str,
        options: &AutoSuggestOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/autosuggest?key={}&input={}",
            self.host, self.api_key, input
        );
        if let Some(focus_coordinates) = options.focus_coordinates {
            url = parse_url(url, "focus", &focus_coordinates.to_string());
        }
        if let Some(circle) = options.circle {
            url = parse_url(url, "clip-to-circle", &circle.to_string());
        }
        if let Some(country_value) = &options.countries {
            let mut countries: String = String::new();
            for country in country_value.iter() {
                countries.push_str(&format!("{},", &country));
            }
            countries.pop();
            url = parse_url(url, "clip-to-country", &countries);
        }
        if let Some(bounding_box) = options.bounding_box {
            url = parse_url(url, "clip-to-bounding-box", &bounding_box.to_string());
        }
        if let Some(polygon) = options.polygon {
            url = parse_url(url, "clip-to-polygon", &polygon.to_string());
        }
        if let Some(language) = options.language {
            url = parse_url(url, "language", language);
        }
        if let Some(prefer_land) = options.prefer_land {
            url = parse_url(url, "prefer-land", &format!("{}", prefer_land));
        }
        if let Some(locale) = options.locale {
            url = parse_url(url, "locale", locale);
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    /// Autosuggest 3word addresses based on provided parameters and fetch the JSON body.
    /// ```
    /// let incomplete_three_words: &str = "fight.offer.ai";
    /// let autosuggest_resp = w3_client.autosuggest_json(incomplete_three_words,
    /// AutoSuggestOptions::default());
    /// ```
    pub fn autosuggest_json(
        &self,
        input: &str,
        options: &AutoSuggestOptions,
    ) -> Result<Value, Response> {
        let resp = self.autosuggest(input, options);
        let json = get_json(resp)?;
        Ok(json)
    }

    /// Retrieve a list of the coordinates of all what3words squares in a given rectangle
    /// which is defined by the coordinates of the southwestern and norteastern points.
    ///
    /// # Example
    ///
    /// ```
    /// let coordinate_sw = Coordinate {
    ///     latitude: -4.0,
    ///     longitude: 178.2
    /// };
    /// let coordinate_ne = Coordinate {
    ///     latitude: 22.0,
    ///     longitude: 195.4
    /// };
    /// let bounding_box = BoundingBox {
    ///     south_west: coordinate_sw,
    ///     north_east: coordinate_ne
    /// };
    /// let resp = w3_client.grid_section(&bounding_box, GridSectionOptions::default());
    /// ```
    pub fn grid_section(
        &self,
        bounding_box: &BoundingBox,
        options: &GridSectionOptions,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/grid-section?bounding-box={}&key={}",
            self.host,
            bounding_box.to_string(),
            self.api_key
        );
        if let Some(format) = options.format {
            url = parse_url(url, "format", format);
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    /// Fetch the JSON body of the `grid_section` call.
    ///
    /// # Example
    ///
    /// ```
    /// let coordinate_sw = Coordinate {
    ///     latitude: -4.0,
    ///     longitude: 178.2
    /// };
    /// let coordinate_ne = Coordinate {
    ///     latitude: 22.0,
    ///     longitude: 195.4
    /// };
    /// let bounding_box = BoundingBox {
    ///     south_west: coordinate_sw,
    ///     north_east: coordinate_ne
    /// };
    /// let resp_json = w3_client.grid_section_json(&bounding_box, GridSectionOptions::default());
    /// ```
    pub fn grid_section_json(
        &self,
        bounding_box: &BoundingBox,
        options: &GridSectionOptions,
    ) -> Result<Value, Response> {
        let resp = self.grid_section(bounding_box, options);
        let json = get_json(resp)?;
        Ok(json)
    }
}

/// Fetch the JSON body from a Response.
fn get_json(resp: Result<Response, Response>) -> Result<Value, Response> {
    let json: Value = resp?
        .json()
        .expect("An error occurred while extracting JSON from response");
    Ok(json)
}

/// Check the status code of a response.
/// If the status code is between 400 and 599, a error will be printed to io::stderr
fn check_status_code(response: Response) -> Result<Response, Response> {
    let status_code = response.status();
    if status_code.is_client_error() || status_code.is_server_error() {
        eprintln!(
            "The response returned an error, status code: {}",
            status_code
        );
        return Err(response);
    }
    Ok(response)
}

/// Parse the URL based on a given keyword and value.
fn parse_url(mut url: String, keyword: &str, value: &str) -> String {
    url.push_str(&format!("&{}={}", keyword, value));
    url
}

#[cfg(test)]
mod tests {
    use crate::{parse_url, AutoSuggestOptions, W3WClient};

    #[test]
    fn test_parsing_url() {
        let mut w3_client = W3WClient::new("mock-api-key");
        w3_client.host = String::from("https://test.com");
        let options = AutoSuggestOptions {
            language: Some("nl"),
            prefer_land: Some(false),
            ..Default::default()
        };

        let mut url = format!("{}/endpoint?key={}", w3_client.host, w3_client.api_key);
        if let Some(language) = options.language {
            url = parse_url(url, "language", language);
        }
        if let Some(prefer_land) = options.prefer_land {
            url = parse_url(url, "prefer-land", &format!("{}", prefer_land));
        }
        assert_eq!(
            url.to_string(),
            "https://test.com/endpoint?key=mock-api-key&language=nl&prefer-land=false"
        );
    }
}
