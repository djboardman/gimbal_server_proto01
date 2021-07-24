use std::collections::HashMap;

use serde::Deserialize;
use rhai::{Engine, Scope, ParseError, EvalAltResult};

use super::data_type::DataType;
use super::event::{Event, EventInstance};
use super::super::data::data_instance::DataInstance;
use super::super::data::data_object_instance::DataObjectInstance;

#[derive(Debug)]
pub enum CommandError {
  ScriptParse(ParseError),
  ScriptEval(EvalAltResult),
  ScriptMissing
}
/*
impl std::fmt::Display for CommandError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      CommandError::ScriptParse(ref err) => err.fmt(f),
      CommandError::ScriptEval(ref err) => err.fmt(f),
      CommandError::ScriptMissing => format!("The script was missing")
    }
  }
}
*/

impl From<ParseError> for CommandError {
  fn from(error: ParseError) -> CommandError {
    CommandError::ScriptParse(error)
  }
}

impl From<Box<EvalAltResult>> for CommandError {
  fn from(error: Box<EvalAltResult>) -> CommandError {
    CommandError::ScriptEval(*error)
  }
}

#[derive(Clone, Debug, Deserialize)]
pub enum CommandMapping {
  Direct,
  Script(String)
}


#[derive(Clone, Debug, Deserialize)]
pub struct Command {
  pub data_object: HashMap<String, DataType>,
  pub event_name: String,
  pub command_mapping: CommandMapping,
}

impl Command {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Command> {
    serde_yaml::from_str(yaml)
  }

  pub fn process_command(&self, command_data: &DataObjectInstance, event: &Event) -> Result<EventInstance, CommandError> {
    let data_object: Result<DataObjectInstance, CommandError> = match &self.command_mapping {
      CommandMapping::Direct => Ok(command_data.clone()),
      CommandMapping::Script(_) => self.run_script()
    };
    match data_object {
      Ok(d_o) => Ok(EventInstance{event_name: self.event_name.clone(), data_object: command_data.clone()}),
      Err(e) => Err(e)
    }
  }

  pub fn run_script(&self) -> Result<DataObjectInstance, CommandError> {
    let script = if let CommandMapping::Script(s) = &self.command_mapping {s.clone()} else {return Err(CommandError::ScriptMissing)};
    let engine = Engine::new();
    let ast = engine.compile(&script)?;
    let r: f64 = engine.call_fn(&mut Scope::new(), &ast, &"create_expense_report", (12_9f64,))?;
    let mut d_o = DataObjectInstance::new();
    d_o.insert(format!("cost"), DataInstance::Number(r));
    Ok(d_o)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_command() -> String {
    String::from(
r#"
data_object: {cost: Number}
event_name: expense_report_created
command_mapping:
      Script: >
        fn create_expense_report(cost) {
          cost * 2
        }
"#)
  }

  fn valid_event() -> String {
    String::from(
r#"
[{cost: Number}]
"#)
  }

  #[test]
  fn loads_command() {
    let c = Command::load_from_str(&valid_command()).unwrap();
    let e = Event::load_from_str(&valid_event()).unwrap();
    let mut d_o = DataObjectInstance::new();
    d_o.insert(format!("cost"), DataInstance::Number(12.3f64));
    let e_o = c.process_command(&d_o, &e).unwrap();
    let v = if let DataInstance::Number(f) = e_o.data_object.get(&format!("cost")).unwrap() {f} else {&0f64};
    assert_eq!(*v, 12.3f64);
    println!("{}", v.to_string());
  }
}