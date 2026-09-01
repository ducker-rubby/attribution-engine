use crate::models::Event;
use std::error::Error;

pub enum ConversionType<'a> {
    SignUp(&'a str),
    Sale(&'a str),
}

impl<'a> TryFrom<&str> for ConversionType<'a> {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sign-up" => Ok(Self::SignUp("sign-up")),
            "sale" => Ok(Self::Sale("sale")),
            _ => Ok(Self::Sale("sale")),
        }
    }
}

pub struct ConversionEvent<'a> {
    conversion_type: ConversionType<'a>,
    click_ref: &'a str,
    metadata: Vec<(&'a str, &'a str)>,
}

impl<'a> ConversionEvent<'a> {
    pub fn build(click_ref: &'a str, conversion_type: &'a str) -> Result<Self, Box<dyn Error>> {
        let this_type: ConversionType<'a> = conversion_type.try_into()?;
        let metadata = vec![("click_ref", click_ref), ("conversion_type", "")];

        Ok(Self {
            conversion_type: this_type,
            click_ref,
            metadata,
        })
    }
}

impl<'a> Event for ConversionEvent<'a> {
    fn get_metadata(&self) -> &[(&str, &str)] {
        unimplemented!()
    }
}
