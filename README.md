# Overview

## Description

This is a Rust client for the What3Words API, allowing you to convert coordinates to three-word addresses and vice versa.
This client is based on the provided [API documention](https://developer.what3words.com/public-api/docs) on the What3Words website.

## Features

The following endpoints have been implemented:
- Convert coordinates to 3words addresses;
- Convert 3words addresses to coordinates;
- Autosuggest 3words addresses based on given parameters;
- Retrieve a list of the coordinates of all what3words squares in a given rectangle which is defined by the coordinates of the southwestern and norteastern points;
- Retrieve the available languages and locales.

## Usage

### Initialization

Import the library and create a W3WClient instance.

```rust
extern crate what3words;
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
}
```

### Convert coordinates to what3words address

#### Fetch response

This code snippet fetches the entire response of the GET call to the endpoint `/convert-to-w3a`.

```rust
use what3words::{Coordinates, W3WClient};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinates {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa(&coordinates, Some("en") , Some("json"), None);
}
```

This call fetches a what3word address for the coordinates (50.830005, 4.329982). We also give the optional parameter `language` which we set to `Some("en")`, this will return us a 3 word value in the provided language (in our case English, which is de default).

The optional `format` parameter can be either `"json"` (default) or `"geojson"`.

The locale is used to specify a variant of a specific language. All supported languages and locales can be fetched with the `W3WClient::available_languages()` call.

#### Fetch JSON

```rust
use what3words::{Coordinates, W3WClient};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinates {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa_json(&coordinates, Some("en") , Some("json"), None);
}
```

This function only fetches the JSON body of the response. This JSON body is of type `serde_json::Value`, so the result of this call is `Result<Value, Response>`.

#### Fetch string

Only the 3word address can be fetched as well:

```rust
use what3words::{Coordinates, W3WClient};

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let coordinates = Coordinates {
        latitude: 50.830005,
        longitude: 4.329982,
    };
    let resp = w3_client.convert_to_3wa_string(&coordinates, Some("en") , Some("json"), None);
}
```

Which will result in a `Result<String, Response>`.


### Convert what3word address to coordinates

#### Fetch response

This code snippet fetches the entire response of the GET call to the endpoint `/convert-to-coordinates`.

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates(words, Some("json"), None);
}
```

Conversion from 3word address to coordinates can be done this way. In the above example we also provide the `format` parameter, but no `locale` parameter.

#### Fetch JSON

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates_json(words, Some("json"), None);
}
```

This way we can fetch the response body JSON, so the returned object is of type `Result<Value, Respones>`.

#### Fetch floats

Fetching only the floats can be done like this:

```rust
use what3words::W3WClient;

fn main {
    let w3_client = W3WClient::new("<your API key>");
    let words: &str = "fight.offer.airbag";
    let resp = w3_client.convert_to_coordinates_floats(words, Some("json"), None);
}
```

Which will give us the the latitude and longitude in `f64`: `Result<(f64, f64), Response>`
