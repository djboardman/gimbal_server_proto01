use std::collections::HashMap;
use serde::Deserialize;
use super::data_type::DataType;

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
  parameters: HashMap<String, DataType>
}

impl Command {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<Command> {
    serde_yaml::from_str(yaml)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_command() -> String {
    String::from(
r#"
[{cost: Number}]
"#)
  }

  #[test]
  fn loads_command() {
    let c = Command::load_from_str(&valid_command()).unwrap();
    assert!(c.parameters.get("cost").is_some());
  }
}