use crate::models::Event;
use std::{error::Error, fmt};

pub enum ConversionType {
    SignUp,
    Sale,
}

impl<'a> TryFrom<&str> for ConversionType {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sign-up" => Ok(Self::SignUp),
            "sale" => Ok(Self::Sale),
            _ => Err(Box::from("No conversion type found")),
        }
    }
}

impl fmt::Display for ConversionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionType::SignUp => write!(f, "sign-up"),
            ConversionType::Sale => write!(f, "sale"),
        }
    }
}

pub struct ConversionEvent<'a> {
    conversion_type: ConversionType,
    conversion_type_str: String,
    click_ref: &'a str,
}

impl<'a> ConversionEvent<'a> {
    pub fn build(click_ref: &'a str, conversion_type: &'a str) -> Result<Self, Box<dyn Error>> {
        unimplemented!()
    }
}

impl<'a> Event for ConversionEvent<'a> {
    fn get_metadata(&self) -> Vec<(&str, &str)> {
        vec![("conversion_type", self.conversion_type_str.as_str())]
    }
}
