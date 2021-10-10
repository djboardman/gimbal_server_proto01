use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize};

use super::label::LabelMap;

#[derive(Debug, Deserialize, Clone)]
pub enum DataType {
  #[serde(rename = "String")]
  String,
  #[serde(rename = "Number")]
  Number
}

#[derive(Deserialize, Debug, Clone)]
pub struct Field {
  pub labels: LabelMap,
  pub countries: Option<Vec<String>>,
  pub data_type: DataType
}

impl Field {
  pub fn field_for_country(&self, country: &str) -> bool {
    self.countries.is_none() || self.countries.as_ref().unwrap().iter().any(|c| c == country)
  }
 
}



