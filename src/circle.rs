//! The `Circle` can be used to define a circle which can be used in some What3Words API calls.
//! A circle consist of a centerpoint coordinate and a radius in kilometers.

use crate::coordinate::Coordinate;

/// A circle constructed of a centerpoint which is a coordinate and a radius in
/// kilometers.
#[derive(Debug)]
pub struct Circle<'a> {
    /// The coordinates of the centerpoint
    pub centerpoint: &'a Coordinate,
    /// The radius in kilometers
    pub radius: f64,
}

impl Circle<'_> {
    pub fn to_string(&self) -> String {
        format!("{},{}", self.centerpoint.to_string(), self.radius)
    }
}
