# Description

This is a Rust client for the What3Words API, allowing you to convert coordinates to three-word addresses and vice versa.
This client is based on the provided [API documention](https://developer.what3words.com/public-api/docs) on the What3Words website.

This crate can be found [on crates.io](https://crates.io/crates/what3words) as the crate `what3words`.

# Features

The following endpoints have been implemented:
- Convert coordinates to 3words addresses;
- Convert 3words addresses to coordinates;
- Autosuggest 3words addresses based on given parameters;
- Retrieve a list of the coordinates of all what3words squares in a given rectangle which is defined by the coordinates of the southwestern and norteastern points;
- Retrieve the available languages and locales.

# Usage

## Initialization

Import the library and create a W3WClient instance.

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
}
```

## Convert coordinates to what3words address

### Fetch response

This code snippet fetches the entire response of the GET call to the endpoint `/convert-to-w3a`.

```rust
use what3words::{Coordinate, W3WClient, ConvertTo3WAOptions};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinate {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa(&coordinates, &ConvertTo3WAOptions::default());
}
```

This call fetches a what3word address for the coordinates (50.830005, 4.329982). We also use the default optional parameters, which are all set to `None`.


See the [Options](#options) section for more details on the optional parameters.

### Fetch JSON

```rust
use what3words::{Coordinate, W3WClient, ConvertTo3WAOptions};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinate {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa_json(&coordinates, &ConvertTo3WAOptions::default());
}
```

This function only fetches the JSON body of the response. This JSON body is of type `serde_json::Value`, so the result of this call is `Result<Value, Response>`.

### Fetch string

Only the 3word address can be fetched as well:

```rust
use what3words::{Coordinate, W3WClient, ConvertTo3WAOptions};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinate {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa_string(&coordinates, &ConvertTo3WAOptions::default());
}
```

Which will result in a `Result<String, Response>`.


## Convert what3word address to coordinates

### Fetch response

This code snippet fetches the entire response of the GET call to the endpoint `/convert-to-coordinates`.

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates(words, &ConvertToCoordinatesOptions::default());
}
```

Conversion from 3word address to coordinates can be done this way.
See the [Options](#options) section for more details on the optional parameters.

### Fetch JSON

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates_json(words, &ConvertToCoordinatesOptions::default());
}
```

This way we can fetch the response body JSON, so the returned object is of type `Result<Value, Respones>`.

### Fetch floats

Fetching only the floats can be done like this:

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates_floats(words, &ConvertToCoordinatesOptions::default());
}
```

Which will give us the the latitude and longitude in `f64`: `Result<(f64, f64), Response>`

## Autosuggest

Autosuggest 3word addresses based on provided parameters.

### No extra options

```rust
let incomplete_three_words: &str = "fight.offer.ai";
let autosuggest_resp = w3_client.autosuggest(incomplete_three_words, &AutoSuggestOptions::default());
```

### Focus coordinates

Get autosuggstions in order, based on the provided focus point.

```rust
let coordinates = Coordinate{
    latitude: 51.0,
    longitude: 4.0
};
let options = AutoSuggestOptions {
    focus_coordinates: Some(&coordinates),
    ..Default::default()
};
let autosuggest_resp = w3_client.autosuggest(incomplete_three_words, &options);
```

### Circle

Get autosuggestions within a given circle.

```rust
let coordinates = Coordinate {
    latitude: 51.0,
    longitude: 4.0
};
let circle = Circle {
    centerpoint: &coordinates,
    radius: 35.0
};
let options = AutoSuggestOptions {
    circle: Some(&circle),
    ..Default::default()
};
let autosuggest_resp = w3_client.autosuggest(incomplete_three_words, &options);
```

### Countries

Restricts AutoSuggest to only return results inside the countries specified by
comma-separated list of uppercase ISO 3166-1 alpha-2 country codes
(for example, to restrict to Belgium and the UK, use clip-to-country=GB,BE).
Clip-to-country will also accept lowercase country codes. Entries must be two a-z letters.
WARNING: If the two-letter code does not correspond to a country, there is no error:
API simply returns no results.

```rust
let countries = vec!["GB", "BE"];
let options = AutoSuggestOptions {
    countries: Some(&countries),
    ..Default::default()
};
let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
```

### BoundingBox

Restrict AutoSuggest results to a bounding box, specified by coordinates.
Coordinate(south_lat,west_lng),Coordinate(north_lat,east_lng), where:
south_lat less than or equal to north_latwest_lng less than or equal to east_lng.
In other words, latitudes and longitudes should be specified order of increasing size.
Lng is allowed to wrap, so that you can specify bounding boxes which cross
the ante-meridian: -4,178.2,22,195.4

```rust
let coordinate_sw = Coordinate {
    latitude: -4.0,
    longitude: 178.2
};
let coordinate_ne = Coordinate {
    latitude: 22.0,
    longitude: 195.4
};
let bounding_box = BoundingBox {
    south_west: &coordinate_sw,
    north_east: &coordinate_ne
};
let options = AutoSuggestOptions {
    bounding_box: Some(&bounding_box),
    ..Default::default()
};
let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
```

### Polygon

Restrict AutoSuggest results to a polygon, specified by a comma-separated list of lat,lng pairs.
The API is currently limited to accepting up to 25 pairs.

```rust
let coordinates1 = Coordinate {
    latitude: 51.521,
    longitude: -0.343,
};
let coordinates2 = Coordinate {
    latitude: 52.6,
    longitude: 2.3324,
};
let coordinates3 = Coordinate {
    latitude: 54.234,
    longitude: 8.343,
};
let polygon: Polygon = Polygon {
    coordinates: vec![&coordinates1, &coordinates2, &coordinates3],
};
let options = AutoSuggestOptions {
    polygon: Some(&polygon),
    ..Default::default()
};
let resp = w3_client.autosuggest_json(incomplete_three_words, &options);
```

## Options {#options}

The optional parameters of most calls can be given through some `Options` structs:

- Options for `convert_to_3wa` calls can be given through the `ConvertTo3WAOptions`.
- Options for `convert-to-coordinates` calls can be given through the `ConvertToCoordinatesOptions`.
- Options for `autosuggest` calls can be given through the `AutoSuggestOptions`.
- Options for `grid_section` calls can be given through the `GridSectionOptions`.

Defaults:

- The `format` parameter defaults in this crate to `None`, which the what3words API will interpret as `"json"`.
- The `language` parameter defaults in this crate to `None`, which the what3words API will interpret as `"en"`.

```rust
use what3words::{AutoSuggestOptions, ConvertTo3WAOptions, ConvertToCoordinatesOptions, Coordinate, GridSectionOptions};

fn main {
    let convert_to_3wa_options1 = ConvertTo3WAOptions {
        language: Some("nl"), // If unspecified, the what3words API defaults to "en"
        ..Default::default()
    };

    let convert_to_3wa_options2 = ConvertTo3WAOptions {
        format: Some("geojson"), // If unspecified, the what3words API defaults to "json"
        language: Some("zh"),
        locale: Some("zh_tr")
    };

    let convert_to_coordinates_options1 = ConvertToCoordinatesOptions {
        format: Some("geojson"),
        ..Default::default()
    };

    let convert_to_coordinates_options2 = ConvertToCoordinatesOptions {
        format: Some("geojson"),
        locale: Some("zh_tr")
    };

    let autosuggest_options1 = AutoSuggestOptions {
        focus_coordinates: Some(&Coordinate{latitude: 51.0, longitude: 4.0}),
        countries: Some(&vec!["BE", "GB"]),
        ..Default::default()
    };

    let grid_section_options1 = GridSectionOptions {
        format: Some("geojson")
    };
}
```

## Available languages

The available languages and locales can be fetched with the `available_languages` method.

### Available languages response

```rust
use what3words::W3WClient;

fn main() {
    let w3_client = W3WClient::new("<your API key>");
    let resp = w3_client.available_languages();
}
```

### Available languages JSON

```rust
use what3words::W3WClient;

fn main() {
    let w3_client = W3WClient::new("<your API key>");
    let resp = w3_client.available_languages_json();
}
```
