std::collections::HashMap;
searde::Deserialize;
super::Field::DataType;


pub struct Command {
  parameters: HashMap<String, Params>
}

impl Comman {
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
"#
  }

  #[test]
  fn loads_command() {
    let c = Command::load_from_str(&valid_command()).unwrap();
    assert!(c.params.get(&"cost").unwrap().is_some());
  }
}


