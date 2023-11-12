/// Represents geographical coordinates with latitude and longitude.
pub struct Coordinates {
    /// The latitude value
    pub latitude: f64,
    /// The longitude value
    pub longitude: f64,
}

impl Coordinates {
    pub fn to_string(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }
}
