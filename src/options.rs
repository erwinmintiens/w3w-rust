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
