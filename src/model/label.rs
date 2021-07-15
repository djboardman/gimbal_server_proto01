use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct LabelMap(HashMap<String, String>);

impl LabelMap {
  pub fn label_for_lang(&self, lang: &str, default_lang: &str) -> String {
    match &self.0.get(lang) {
      Some(l) => l.to_string(),
      // If it panics then the model hasn't been validated so we'll let it panic
      _ => self.0.get(default_lang).unwrap().to_string()
    }
  }
}


