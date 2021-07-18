use serde::{Deserialize};
use std::collections::HashMap;

mod aggregate;
mod entity;
mod label;
mod field;
mod command;
mod event;
mod data_type;
mod data;

use aggregate::Aggregate;
use label::LabelMap;
use data_type::DataType;

#[derive(Debug, Clone, Deserialize)]
pub struct Model {
  name: String,
  default_lang: String,
  aggregates: HashMap<String, Aggregate>,
  labels: LabelMap,
  data_types: Option<HashMap<String, DataType>>
}

impl Model {
  pub fn load_from_str(str: String) -> serde_yaml::Result<Model>  {
    serde_yaml::from_str(&str)
  }
}

/*

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_model() -> String{
    String::from(
r#"
name: "expenses"
default_lang: "en"
labels:
  en: "Expenses"
  fr: "Les Expenses"
aggregates:
  expense_report: 
    labels:
      en: "Expense Report"
    entities:
      expense_report:
        labels:
          en: "Expense Report"
        fields:
          cost:
            data_type: Number
            labels:
              en: "Cost"
          vat:
            data_type: Number
            labels:
              en: "VAT Amount"
            countries:
              - uk
              - fr
      expense_line:
        labels:
          en: "Expense Line"
        fields:
          cost:
            data_type: Number
            labels:
              en: "Cost"
  person: 
    entities:
      person:
        labels:
          en: "Person"
        fields:
          first_name:
            data_type: String
            labels:
              en: "First Name"
    labels:
      en: "Person"

"#)
  }

  fn scan_error_model() -> String {
    String::from(
  "
  model: :
  "
    )
  }

  fn load_error_model() -> String {
    String::from(
"
model:
")
  }

  fn valid_command() -> String {
    String::from(
r#"
data_object: {cost: Number}
event_name: expense_report_created
"#)
  }

  fn valid_event() -> String {
    String::from(
r#"
[{cost: Number}]
"#)
  }



  #[test]
  fn load_expenses() {
    let en = String::from("en");
    let m = Model::load_from_str(valid_model()).unwrap();
    assert_eq!(m.name, String::from("expenses"));
    assert_eq!(m.default_lang, String::from("en"));
    assert_eq!(m.labels.label_for_lang(&en, &m.default_lang), String::from("Expenses"));
    assert_eq!(m.labels.label_for_lang(&"fr", &m.default_lang), String::from("Les Expenses"));
    assert_eq!(m.aggregates.get("expense_report").unwrap().labels.label_for_lang(&en, &m.default_lang), String::from("Expense Report"));
    assert_eq!(m.aggregates.get("expense_report").unwrap().entities.get("expense_line").unwrap().labels.label_for_lang(&en, &m.default_lang), String::from("Expense Line"));
  }

  #[test]
  fn fields_for_specific_countries() {
    let en = String::from("en");
    let m = Model::load_from_str(valid_model()).unwrap();
    let e = m.aggregates.get("expense_report").unwrap().entities.get("expense_report").unwrap();
    assert_eq!(e.fields_for_country("us").unwrap().get("cost").unwrap().labels.label_for_lang(&en, &en), String::from("Cost"));
    assert!(e.fields_for_country("us").unwrap().get("vat").is_none());
  }
/*
  #[test]
  fn scan_error() {
    let y = Model::parse_str(scan_error_model());
    assert!(y.is_err());
  }

  #[test]
  fn load_error() {
    let y = Model::parse_str(load_error_model()).unwrap();
    let m = Model::load_yaml(y);
    assert!(m.is_err());
  }
*/
}
*/