use reqwest::blocking;
use serde::{Serialize, Deserialize};
use serde_json::{Result};

const API_URL_SEARCH : &'static str = "https://api-adresse.data.gouv.fr/search/?q=";

#[derive(Serialize, Deserialize)]
struct AddressResult {
    r#type: String,
    version: String,
    features: Vec<Feature>
}
#[derive(Serialize, Deserialize)]
struct Feature {
    r#type: String,
    geometry: Geometry,
    properties: Properties
}

#[derive(Serialize, Deserialize)]
struct Geometry {
    r#type: String,
    coordinates: Coordinates
}

#[derive(Serialize, Deserialize)]
struct Coordinates {
    #[serde(rename = "0")]
    lat: f64,
    #[serde(rename = "1")]
    lon: f64
}

#[derive(Serialize, Deserialize)]
struct Properties {
    label: String,
    score: f64,
    housenumber: Option<String>,
    id: String,
    r#type: String,
    x: f64,
    y: f64,
    importance: f64,
    name: String,
    postcode: String,
    citycode: String,
    context: String,
    street: Option<String>

}

fn get_address_info(search: &str) -> Result<AddressResult>  {
    let url = format!("{}{}", API_URL_SEARCH, search);
    let value = blocking::get(&url)
        .expect("Error when request api")
        .text()
        .expect("Error when get text value from api");
    let data: AddressResult = serde_json::from_str(&value).expect("Error when parse JSON");
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result =  get_address_info("200 Chemin de puy petit").unwrap();
        assert_eq!(result.features[0].properties.postcode, "26270");
        assert_eq!(result.features[0].properties.citycode, "26166");
    }
}
