use crate::coordinates::Coordinates;
/// A rectangle which is defined by the coordinates of the southwestern point and the coordinates
/// of the northeastern point.
pub struct BoundingBox {
    /// Coordinates of the southwestern point
    pub south_west: Coordinates,
    /// Coordinates of the northeastern point
    pub north_east: Coordinates,
}

impl BoundingBox {
    pub fn to_string(&self) -> String {
        format!(
            "{},{}",
            self.south_west.to_string(),
            self.north_east.to_string()
        )
    }
}
