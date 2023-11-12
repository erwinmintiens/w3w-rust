//! Here are all `Options` structs defined which can be used to pass to what3words endpoints.

use crate::bounding_box::BoundingBox;
use crate::circle::Circle;
use crate::coordinate::Coordinate;
use crate::polygon::Polygon;

/// The optional parameters for the `convert_to_3wa` calls.
#[derive(Debug)]
pub struct ConvertTo3WAOptions<'a> {
    /// language of the returned 3 words
    pub language: Option<&'a str>,
    /// format of the returned payload. Either `"json"` or `"geojson"`
    pub format: Option<&'a str>,
    /// locale to specify a variant of a language
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
    pub focus_coordinates: Option<&'a Coordinate>,
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
