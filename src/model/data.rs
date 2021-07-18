use std::collections::HashMap;
use std::error::Error;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::data_type::{DataTypeData};

#[derive(Debug, Clone)]
pub enum Data {
  String(String),
  Number(f64),
  Decimal(Decimal),
  Money(Decimal, DataTypeData),
  Quantity(i64),
  Amount(Decimal, DataTypeData)
}

impl Data {
  pub fn display(&self) -> Option<String> {
    match &self {
      Data::String(v) => Some(v.to_string()),
      Data::Number(v) => Some(v.to_string()),
      Data::Decimal(v) => Some(v.to_string()),
      Data::Money(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None},
      Data::Quantity(v) => Some(v.to_string()),
      Data::Amount(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None}
    }
  }
}

pub type DataObject = HashMap<String, Data>;


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn displays_string() {
    let v = format!("Hello, world");
    let s = Data::String(v.clone());
    assert_eq!(s.display().unwrap(), v);
  }

  fn displays_money() {
    let m = Data::Money(dec!(100.10), DataTypeData::Decimal(format!("GBP"), format!("£"), format!("p")));
    assert_eq!(m.display().unwrap(), format!("£100.10"));
  }
}






