use std::collections::HashMap;
use serde::Deserialize;
use super::field::DataType;

#[derive(Clone, Debug, Deserialize)]
pub struct Event {
  pub data_object: HashMap<String, DataType>
}

impl Event {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Event> {
    serde_yaml::from_str(yaml)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_event() -> String {
    String::from(
r#"
[{cost: Number, country: String, vat: Number}]
"#)
  }

  #[test]
  fn loads_event() {
    let c = Event::load_from_str(&valid_event()).unwrap();
    assert!(c.data_object.get("cost").is_some());
  }
}