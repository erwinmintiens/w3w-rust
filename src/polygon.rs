//! A Polygon is a figure defined by multiple coordinates and can be used in certain what3words API
//! calls.

use crate::coordinate::Coordinate;
use crate::traits::Printable;

/// A polygon defined by at least 3 coordinates. The what3words API only supports up to 25
/// coordinates at the moment.
#[derive(Debug, Clone)]
pub struct Polygon<'a> {
    /// Vector of the coordinates of the polygon
    pub coordinates: Vec<&'a Coordinate>,
}
//
// impl Polygon<'_> {
//     /// Returns a string of all the coordinates of the polygon separated with a comma. As last
//     /// element, the first coordinate is added again as per the what3words API documentation.
//     pub fn to_string(&self) -> String {
//         let mut url: String = String::new();
//         for item in self.coordinates.iter() {
//             url.push_str(&format!("{},", &item.to_string()));
//         }
//         url.push_str(&self.coordinates[0].to_string());
//         url
//     }
// }

impl Printable for Polygon<'_> {
    /// Returns a string of all the coordinates of the polygon separated with a comma. As last
    /// element, the first coordinate is added again as per the what3words API documentation.
    fn to_string(&self) -> String {
        let mut result: String = String::new();
        for item in self.coordinates.iter() {
            result.push_str(&format!("{},", &item.to_string()));
        }
        result.push_str(&self.coordinates[0].to_string());
        result
    }
}
