use serde::Deserialize;
use rust_decimal::Decimal;
//use rust_decimal_macros::dec;

#[derive(Debug, Clone, Deserialize)]
pub enum DataType {
  String,
  Number,
  Decimal,
  Bool,
  Money(MoneyMeta),
  Quantity,
  Amount(AmountMeta)
}

impl DataType {
  pub fn load_from_str(yaml: &str) -> serde_yaml::Result<DataType> {
    serde_yaml::from_str(yaml)
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_number() -> String {
    String::from("Number")
  }

  fn valid_money_meta() -> String {
    String::from(
r#"
name: USD
symbol: $
cent_symbol: c
"#
    )
  }

  fn valid_money() -> String {
    String::from(
r#"
Money:
  name: USD
  symbol: $
  cent_symbol: c
"#)
  }

  #[test]
  fn loads_number() {
    let n = DataType::load_from_str(&valid_number());
    assert!(n.is_ok());
  }

  #[test]
  fn loads_money_meta() {
    let mm = MoneyMeta::load_from_str(&valid_money_meta());
    assert_eq!(mm.unwrap().name, String::from("USD"));
  }

  #[test]
  fn loads_money() {
    let m = DataType::load_from_str(&valid_money()).unwrap();
    if let DataType::Money(meta) = m {
      assert_eq!(meta.name, String::from("USD"));
    } else {
      assert!(false);
    }
  }
}





