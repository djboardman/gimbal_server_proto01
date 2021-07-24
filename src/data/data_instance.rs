

use rust_decimal::Decimal;


use super::super::model::data_type::{DataTypeData};

#[derive(Debug, Clone)]
pub enum DataInstance {
  String(String),
  Number(f64),
  Decimal(Decimal),
  Money(Decimal, DataTypeData),
  Quantity(i64),
  Amount(Decimal, DataTypeData)
}

impl DataInstance {
  pub fn display(&self) -> Option<String> {
    match &self {
      DataInstance::String(v) => Some(v.to_string()),
      DataInstance::Number(v) => Some(v.to_string()),
      DataInstance::Decimal(v) => Some(v.to_string()),
      DataInstance::Money(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None},
      DataInstance::Quantity(v) => Some(v.to_string()),
      DataInstance::Amount(v, d) => if let DataTypeData::Decimal(_, sym, _) = d {Some(format!("{}{}", sym, v.to_string()))} else {None}
    }
  }
}




#[cfg(test)]
mod tests {
  use super::*;
  use rust_decimal_macros::dec;

  #[test]
  fn displays_string() {
    let v = format!("Hello, world");
    let s = DataInstance::String(v.clone());
    assert_eq!(s.display().unwrap(), v);
  }

  #[test]
  fn displays_money() {
    let m = DataInstance::Money(dec!(100.10), DataTypeData::Decimal(format!("GBP"), format!("£"), format!("p")));
    assert_eq!(m.display().unwrap(), format!("£100.10"));
  }
}






