//! The Coordinate struct which is used in the `Circle`, `BoundingBox` and `Polygon` structs.
//! A coordinate is made up of a latitude and a longitude and can be printed as
//! `<latitude>,<longitude>`.

/// Represents geographical coordinates with latitude and longitude.
#[derive(Debug)]
pub struct Coordinate {
    /// The latitude value
    pub latitude: f64,
    /// The longitude value
    pub longitude: f64,
}

impl Coordinate {
    /// Return the coordinate as a string in the form `"<latitude>,<longitude>"`
    pub fn to_string(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }
}
