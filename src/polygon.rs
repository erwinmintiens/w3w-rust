use crate::coordinates::Coordinates;

pub struct Polygon {
    pub coordinates: Vec<Coordinates>,
}

impl Polygon {
    pub fn to_string(&self) -> String {
        let mut url: String = String::new();
        for item in self.coordinates.iter() {
            url.push_str(&format!("{},", &item.to_string()));
        }
        url.push_str(&self.coordinates[0].to_string());
        url
    }
}
