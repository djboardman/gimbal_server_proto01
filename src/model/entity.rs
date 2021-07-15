use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;

use super::label::LabelMap;
use super::field::Field;

#[derive(Debug, Clone, Deserialize)]
pub struct Entity {
  pub labels: LabelMap,
  fields: HashMap<String, Field>,
}


impl Entity {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Entity>{
    serde_yaml::from_str(yaml)
  }

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

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_entity() -> String {
    String::from(
r#"
labels:
  en: "Expense Report"
fields:
  cost:
    data_type: Number
    labels:
      en: "Cost"
"#)
  }
  #[test]
  fn loads_yaml() {
    let e = Entity::load_from_str(&valid_entity()).unwrap();
    assert_eq!(e.labels.label_for_lang(&"en", &"en"), String::from("Expense Report"));
  }
}