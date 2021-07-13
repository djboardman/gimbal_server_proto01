use serde::{Deserialize};
use std::collections::HashMap;

use super::label::LabelMap;
use super::entity::Entity;

#[derive(Debug, Deserialize, Clone)]
pub struct Aggregate {
  pub labels: LabelMap,
  pub entities: HashMap<String, Entity>
}


