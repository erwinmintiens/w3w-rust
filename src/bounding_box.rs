//! A `BoundingBox` is a rectangle defined by 2 coordinates: the southwestern and northeastern coordinates.
//! This BoundingBox can be used to pass as an option to certain what3words calls.

use crate::coordinate::Coordinate;

/// A rectangle which is defined by the coordinate of the southwestern point and the coordinate
/// of the northeastern point.
#[derive(Debug)]
pub struct BoundingBox<'a> {
    /// Coordinates of the southwestern point
    pub south_west: &'a Coordinate,
    /// Coordinates of the northeastern point
    pub north_east: &'a Coordinate,
}

impl BoundingBox<'_> {
    /// Return the BoundingBox as a String in the form
    /// `"<south_west.latitude>,<south_west.longitude>,<north_east.latitude>,<north_east.longitude>"`
    pub fn to_string(&self) -> String {
        format!(
            "{},{}",
            self.south_west.to_string(),
            self.north_east.to_string()
        )
    }
}
