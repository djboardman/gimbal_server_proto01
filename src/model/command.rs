use std::collections::HashMap;
use serde::Deserialize;

use super::data_type::DataType;
use super::event::Event;
use super::data::{Data, DataObject};


#[derive(Clone, Debug, Deserialize)]
pub enum CommandMapping {
  Direct,
  Script
}

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
  pub data_object: HashMap<String, DataType>,
  pub event_name: String,
  pub command_mapping: CommandMapping
}

impl Command {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Command> {
    serde_yaml::from_str(yaml)
  }

  pub fn process_command(&self, command_data: &DataObject, event: &Event) -> DataObject {
    match &self.command_mapping {
      CommandMapping::Direct => command_data.clone(),
      CommandMapping::Script => DataObject::new()
    }
  }
/*
  pub fn to_event(&self) -> Option<Event> {

  }
*/
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_command() -> String {
    String::from(
r#"
data_object: {cost: Number}
event_name: expense_report_created
command_mapping: Direct
"#)
  }

  fn valid_event() -> String {
    String::from(
r#"
data_object: [{cost: Number}]
"#)
  }

  #[test]
  fn loads_command() {
    let c = Command::load_from_str(&valid_command()).unwrap();
    assert!(c.data_object.get("cost").is_some());
  }

}