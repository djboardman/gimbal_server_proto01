use std::collections::HashMap;

use serde::Deserialize;



#[derive(Debug, Clone, Deserialize)]
pub enum DataType {
  String,
  Number,
  Decimal,
  Bool,
  Money,
  Quantity,
  Amount
}

impl DataType {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<DataType> {
    serde_yaml::from_str(yaml)
  }
}

#[derive(Debug, Clone, Deserialize)]
pub enum DataTypeData {
  Decimal(String, String, String),
  CompoundString(HashMap<String, String>)
}

impl DataTypeData {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<DataTypeData> {
    serde_yaml::from_str(yaml)
  }
}

/*
Probably delete all this
#[derive(Debug, Clone, Deserialize)]
pub struct MoneyMeta {
  name: String,
  symbol: String,
  cent_symbol: String
}


impl MoneyMeta {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<MoneyMeta> {
    serde_yaml::from_str(yaml)
  }
}


#[derive(Debug, Clone, Deserialize)]
pub struct AmountMeta {
  name: String,
  symbol: String,
  cent_symbol: String 
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_number() -> String {
    String::from("Number")
  }

  fn valid_data_type_data() -> String {
    String::from(
  "
  Decimal:
    - GBP
    - £
    - p
  "
    )
  }

  #[test]
  fn loads_number() {
    let n = DataType::load_from_str(&valid_number());
    assert!(n.is_ok());
  }

  #[test]
  fn loads_decimal_data_type_data() {
    let d = DataTypeData::load_from_str(&valid_data_type_data()).unwrap();
    if let DataTypeData::Decimal(name, _, _) = d {
      assert_eq!(name, format!("GBP"));
    }
    
  }

}





