use serde::{Deserialize};
use serde_yaml::from_str;
use std::collections::HashMap;

use super::label::LabelMap;
use super::entity::Entity;

#[derive(Debug, Deserialize, Clone)]
pub struct Aggregate {
  pub labels: LabelMap,
  pub entities: HashMap<String, Entity>

}

impl Aggregate {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Aggregate> {
    serde_yaml::from_str(yaml)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_aggregate() -> String {
    String::from(
r#"
labels:
  en: "Expense Report"
entities:
  expense_report:
    labels:
      en: "Expense Report"
    fields:
      cost:
        labels:
          en: "Cost"
        data_type: Number
"#)
  }

  #[test]
  fn loads_yaml() {
    let a = Aggregate::load_from_str(&valid_aggregate()).unwrap();
    assert_eq!(a.labels.label_for_lang(&"en", &"en"), String::from("Expense Report"));
  }
}