use reqwest::blocking;
use std::error::Error as StdError;
use std::fmt;
use std::result;
use serde::{Serialize, Deserialize};

const API_URL_SEARCH : &'static str = "https://api-adresse.data.gouv.fr/search/?q=";
const API_URL_REVERSE : &'static str = "https://api-adresse.data.gouv.fr/reverse/?";

#[derive(Debug)]
pub enum Error {
    HttpError,
    GetTextError,
    UnmarshalJsonError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter <'_>) -> fmt::Result {
        match &*self {
            HttpError => write!(f, "Can't access to http://api-adresse.data.gouv.fr"),
            GetTextError => write!(f, "Can't unmarshal data response to text"),
            UnmarshalJsonError => write!(f, "Can't unmarshal text response to json")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResult {
    pub r#type: String,
    pub version: String,
    pub features: Vec<Feature>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Coordinates
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    #[serde(rename = "0")]
    pub lat: f64,
    #[serde(rename = "1")]
    pub lon: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub label: String,
    pub score: f64,
    pub housenumber: Option<String>,
    pub id: String,
    pub r#type: String,
    pub x: f64,
    pub y: f64,
    pub importance: f64,
    pub name: String,
    pub postcode: String,
    pub citycode: String,
    pub context: String,
    pub street: Option<String>
}

fn get_data(url: &str) -> Result<AddressResult, Error> {
    let response = match blocking::get(url) {
        Ok(value) => value,
        _ => return Err(Error::HttpError)
    };

    let value = match response.text() {
        Ok(value) => value,
        _ => return Err(Error::GetTextError)
    };

    let data: AddressResult = match serde_json::from_str(&value) {
        Ok(value) => value,
        _ => return Err(Error::UnmarshalJsonError)
    };
    Ok(data)

}

pub fn get_address_info(search: &str) -> Result<AddressResult, Error>  {
    let url = format!("{}{}", API_URL_SEARCH, search);
    get_data(&*url)
}

pub fn get_reverse_info(lon: f64, lat: f64) -> Result<AddressResult, Error> {
    let url = format!("{}lon={}&lat={}", API_URL_REVERSE, lon, lat);
    get_data(&*url)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_address_info() {
        let result =  get_address_info("200 Chemin de puy petit").unwrap();
        assert_eq!(result.features[0].properties.postcode, "26270");
        assert_eq!(result.features[0].properties.citycode, "26166");
    }

    #[test]
    fn test_get_reverse_info() {
        let result = get_reverse_info(2.37, 48.357).unwrap();
        assert_eq!(result.features[0].properties.label, "23 Chemin de Pithiviers 91720 Prunay-sur-Essonne");
    }
}
