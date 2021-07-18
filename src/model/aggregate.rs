use serde::{Deserialize};
use serde_yaml::from_str;
use std::collections::HashMap;

use super::label::LabelMap;
use super::entity::Entity;
use super::command::Command;
use super::event::Event;
use super::data::Data;

#[derive(Debug, Deserialize, Clone)]
pub struct Aggregate {
  pub labels: LabelMap,
  pub entities: HashMap<String, Entity>,
  pub commands: HashMap<String, Command>,
  pub events: HashMap<String, Event>

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
commands:
  create_expense_report:
    data_object: {cost: Number}
    event_name: expense_report_created
    command_mapping: Direct
events:
  expense_report_created:
    data_object: {cost: Number}
"#)
  }

  #[test]
  fn loads_yaml() {
    let a = Aggregate::load_from_str(&valid_aggregate()).unwrap();
    assert_eq!(a.labels.label_for_lang(&"en", &"en"), String::from("Expense Report"));
  }

  #[test]
  fn simple_command_to_event() {
    let a = Aggregate::load_from_str(&valid_aggregate()).unwrap();
    let d = Data::Number(23.0);
    let mut command_object = super::super::data::DataObject::new();
    command_object.insert(format!("cost"), d);
    let c = a.commands.get(&format!("create_expense_report")).unwrap();
    let e = a.events.get(&c.event_name).unwrap();
    let event_object = c.process_command(&command_object, e);
  }
}