use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::data_type::{DataTypeData};

#[derive(Debug, Clone)]
enum Value {
  String(String),
  Number(f64),
  Decimal(Decimal),
  Money(Decimal, DataTypeData),
  Quantity(i64),
  Amount(Decimal, DataTypeData)
}

impl Value {
  pub fn display(&self) -> Option<String> {
    match &self {
      Value::String(v) => Some(v.to_string()),
      Value::Number(v) => Some(v.to_string()),
      Value::Decimal(v) => Some(v.to_string()),
      Value::Money(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None},
      Value::Quantity(v) => Some(v.to_string()),
      Value::Amount(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn displays_string() {
    let v = format!("Hello, world");
    let s = Value::String(v.clone());
    assert_eq!(s.display().unwrap(), v);
  }

  fn displays_money() {
    let m = Value::Money(dec!(100.10), DataTypeData::Decimal(format!("GBP"), format!("£"), format!("p")));
    assert_eq!(m.display().unwrap(), format!("£100.10"));
  }
}






