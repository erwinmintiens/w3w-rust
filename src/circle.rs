use crate::coordinates::Coordinates;
/// A circle constructed of a centerpoint which has a latitude and longitude and a radius in
/// kilometers.
pub struct Circle {
    /// The coordinates of the centerpoint
    pub centerpoint: Coordinates,
    /// The radius in kilometers
    pub radius: f64,
}

impl Circle {
    pub fn to_string(&self) -> String {
        format!("{},{}", self.centerpoint.to_string(), self.radius)
    }
}
