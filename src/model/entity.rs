use serde::Deserialize;
use std::collections::HashMap;

use super::label::LabelMap;
use super::field::Field;

#[derive(Debug, Clone, Deserialize)]
pub struct Entity {
  pub labels: LabelMap,
  fields: HashMap<String, Field>,
}


impl Entity {
  pub fn fields_for_country(&self, country: &str) -> Option<HashMap<String, Field>> {
    let mut fs: HashMap<String, Field> = HashMap::new();
    for f in &self.fields {
      if f.1.field_for_country(country) {
        fs.insert(f.0.clone(), f.1.clone());
      }
    }
    if fs.is_empty() {
      None
    } else {
      Some(fs)
    }
  }
}

