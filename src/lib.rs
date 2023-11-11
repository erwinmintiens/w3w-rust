extern crate reqwest;
use reqwest::blocking::Response;
use serde_json::Value;

const W3WHOST: &str = "https://api.what3words.com/v3";

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    fn to_string(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }
}

pub struct Circle {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
}

impl Circle {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.latitude, self.longitude, self.radius)
    }
}

pub struct BoundingBox {
    pub south_west: Coordinates,
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

#[derive(Debug)]
pub struct W3WClient {
    pub api_key: String,
    pub host: String,
    pub client: reqwest::blocking::Client,
}

impl W3WClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            host: W3WHOST.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_request(&self, url: String) -> Result<Response, Response> {
        let resp = self.client.get(url).send();
        let mut response = resp.unwrap();
        response = check_status_code(response)?;
        Ok(response)
    }

    pub fn convert_to_3wa(
        &self,
        coordinates: &Coordinates,
        language: Option<&str>,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-3wa?key={}&coordinates={}",
            self.host,
            self.api_key,
            coordinates.to_string(),
        );
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language));
        }
        if let Some(format) = format {
            url.push_str(&format!("&format={}", format));
        }
        if let Some(locale) = locale {
            url.push_str(&format!("&locale={}", locale));
        }
        println!("URL: {}", url);
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_3wa_json(
        &self,
        coordinates: &Coordinates,
        language: Option<&str>,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_3wa(coordinates, language, format, locale);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_3wa_string(
        &self,
        coordinates: &Coordinates,
        language: Option<&str>,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<String, Response> {
        let json = self.convert_to_3wa_json(coordinates, language, format, locale)?;
        let result = json["words"].to_string();
        Ok(result)
    }

    pub fn convert_to_coordinates(
        &self,
        three_words: &str,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/convert-to-coordinates?words={}&key={}",
            self.host, three_words, self.api_key
        );
        if let Some(format) = format {
            url.push_str(&format!("&format={}", format));
        }
        if let Some(locale) = locale {
            url.push_str(&format!("&locale={}", locale));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn convert_to_coordinates_json(
        &self,
        three_words: &str,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<Value, Response> {
        let resp = self.convert_to_coordinates(three_words, format, locale);
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn convert_to_coordinates_floats(
        &self,
        three_words: &str,
        format: Option<&str>,
        locale: Option<&str>,
    ) -> Result<(f64, f64), Response> {
        let three_words_json: Value =
            self.convert_to_coordinates_json(three_words, format, locale)?;

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
        focus_coordinates: Option<&Coordinates>,
        circle: Option<&Circle>,
        country: Option<&str>,
        bounding_box: Option<&BoundingBox>,
        language: Option<&str>,
        prefer_land: Option<bool>,
        locale: Option<&str>,
    ) -> Result<Response, Response> {
        let mut url = format!(
            "{}/autosuggest?key={}&input={}",
            self.host, self.api_key, input
        );
        if let Some(focus_coordinates) = focus_coordinates {
            url.push_str(&format!("&focus={}", focus_coordinates.to_string()));
        }
        if let Some(circle) = circle {
            url.push_str(&format!("&clip-to-circle={}", circle.to_string()));
        }
        if let Some(country_value) = country {
            url.push_str(&format!("&clip-to-country={}", country_value));
        }
        if let Some(bounding_box) = bounding_box {
            url.push_str(&format!(
                "&clip-to-bounding-box={}",
                bounding_box.to_string()
            ));
        }
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language));
        }
        if let Some(prefer_land) = prefer_land {
            url.push_str(&format!("&prefer-land={}", prefer_land));
        }
        if let Some(locale) = locale {
            url.push_str(&format!("&locale={}", locale));
        }
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn autosuggest_json(
        &self,
        input: &str,
        focus_coordinates: Option<&Coordinates>,
        circle: Option<&Circle>,
        country: Option<&str>,
        bounding_box: Option<&BoundingBox>,
        language: Option<&str>,
        prefer_land: Option<bool>,
        locale: Option<&str>,
    ) -> Result<Value, Response> {
        let resp = self.autosuggest(
            input,
            focus_coordinates,
            circle,
            country,
            bounding_box,
            language,
            prefer_land,
            locale,
        );
        let json = get_json(resp)?;
        Ok(json)
    }

    pub fn autosuggest_with_focus_coordinates(
        &self,
        input: &str,
        focus_coordinates: &Coordinates,
    ) -> Result<Response, Response> {
        let resp = self.autosuggest(
            input,
            Some(focus_coordinates),
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        Ok(resp)
    }

    pub fn autosuggest_with_clip_to_circle(
        &self,
        input: &str,
        circle: &Circle,
    ) -> Result<Response, Response> {
        let resp = self.autosuggest(input, None, Some(circle), None, None, None, None, None)?;
        Ok(resp)
    }

    pub fn autosuggest_with_country(
        &self,
        input: &str,
        country: &str,
    ) -> Result<Response, Response> {
        let resp = self.autosuggest(input, None, None, Some(country), None, None, None, None)?;
        Ok(resp)
    }

    pub fn autosuggest_with_clip_to_bounding_box(
        &self,
        input: &str,
        bounding_box: &BoundingBox,
    ) -> Result<Response, Response> {
        let resp = self.autosuggest(
            input,
            None,
            None,
            None,
            Some(bounding_box),
            None,
            None,
            None,
        )?;
        Ok(resp)
    }

    pub fn autosuggest_with_language(
        &self,
        input: &str,
        language: &str,
    ) -> Result<Response, Response> {
        let resp = self.autosuggest(input, None, None, None, None, Some(language), None, None)?;
        Ok(resp)
    }

    pub fn autosuggest_with_prefer_land(
        &self,
        input: &str,
        prefer_land: bool,
    ) -> Result<Response, Response> {
        let resp =
            self.autosuggest(input, None, None, None, None, None, Some(prefer_land), None)?;
        Ok(resp)
    }

    pub fn autosuggest_with_locale(&self, input: &str, locale: &str) -> Result<Response, Response> {
        let resp = self.autosuggest(input, None, None, None, None, None, None, Some(locale))?;
        Ok(resp)
    }

    pub fn grid_section(&self, bounding_box: &BoundingBox) -> Result<Response, Response> {
        let url = format!(
            "{}/grid-section?bounding-box={}&key={}",
            self.host,
            bounding_box.to_string(),
            self.api_key
        );
        let resp = self.get_request(url)?;
        Ok(resp)
    }

    pub fn grid_section_json(&self, bounding_box: &BoundingBox) -> Result<Value, Response> {
        let resp = self.grid_section(bounding_box);
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
