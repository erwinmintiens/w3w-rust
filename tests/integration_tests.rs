use what3words::{BoundingBox, Circle, Coordinate, Polygon};

const COORDINATE1: Coordinate = Coordinate {
    latitude: 50.12345,
    longitude: -3.98765,
};
const COORDINATE2: Coordinate = Coordinate {
    latitude: 51.0,
    longitude: -3.0,
};
const COORDINATE3: Coordinate = Coordinate {
    latitude: 56.22222,
    longitude: 1.11122,
};
const COORDINATE4: Coordinate = Coordinate {
    latitude: 57.0,
    longitude: 2.0,
};

#[test]
fn test_coordinates_to_string() {
    assert_eq!(COORDINATE1.to_string(), String::from("50.12345,-3.98765"));
}

#[test]
fn test_bounding_box_to_string() {
    let bounding_box = BoundingBox {
        south_west: &COORDINATE1,
        north_east: &COORDINATE2,
    };
    assert_eq!(
        bounding_box.to_string(),
        String::from(&format!(
            "{},{},{},{}",
            &COORDINATE1.latitude,
            &COORDINATE1.longitude,
            &COORDINATE2.latitude,
            &COORDINATE2.longitude
        ))
    );
}

#[test]
fn test_circle_to_string() {
    let circle = Circle {
        centerpoint: &COORDINATE1,
        radius: 12.3,
    };
    assert_eq!(
        circle.to_string(),
        format!("{},{}", circle.centerpoint.to_string(), circle.radius)
    );
}

#[test]
fn test_polygon_to_string() {
    let polygon1_list = vec![&COORDINATE1, &COORDINATE2, &COORDINATE3];
    let polygon1 = Polygon {
        coordinates: polygon1_list,
    };
    assert_eq!(
        polygon1.to_string(),
        format!(
            "{},{},{},{},{},{},{},{}",
            COORDINATE1.latitude,
            COORDINATE1.longitude,
            COORDINATE2.latitude,
            COORDINATE2.longitude,
            COORDINATE3.latitude,
            COORDINATE3.longitude,
            COORDINATE1.latitude,
            COORDINATE1.longitude
        )
    );
    assert_eq!(
        polygon1.to_string(),
        format!(
            "{},{},{},{}",
            COORDINATE1.to_string(),
            COORDINATE2.to_string(),
            COORDINATE3.to_string(),
            COORDINATE1.to_string()
        )
    );

    let polygon2_list = vec![&COORDINATE4, &COORDINATE3, &COORDINATE2, &COORDINATE1];
    let polygon2 = Polygon {
        coordinates: polygon2_list,
    };
    assert_eq!(
        polygon2.to_string(),
        format!(
            "{},{},{},{},{},{},{},{},{},{}",
            COORDINATE4.latitude,
            COORDINATE4.longitude,
            COORDINATE3.latitude,
            COORDINATE3.longitude,
            COORDINATE2.latitude,
            COORDINATE2.longitude,
            COORDINATE1.latitude,
            COORDINATE1.longitude,
            COORDINATE4.latitude,
            COORDINATE4.longitude
        )
    );
    assert_eq!(
        polygon2.to_string(),
        format!(
            "{},{},{},{},{}",
            COORDINATE4.to_string(),
            COORDINATE3.to_string(),
            COORDINATE2.to_string(),
            COORDINATE1.to_string(),
            COORDINATE4.to_string()
        )
    );
}
