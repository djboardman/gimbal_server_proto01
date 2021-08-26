use std::collections::HashMap;

use pest::{Parser, Span};
use pest::iterators::{Pair, Pairs};



#[derive(Parser)]
#[grammar = "model.pest"]
pub struct ModelParser;


fn parse_model(source: &str) -> Result<Vec<ModelTree>, pest::error::Error<Rule>>  {
  Ok(new_model(ModelParser::parse(Rule::file, source)?)?)
}

fn new_model(pairs: Pairs<Rule>) -> Result<Vec<ModelTree>, pest::error::Error<Rule>> {
  let item_rules = vec![Rule::model, Rule::aggregate, Rule::entity];

  let rule_tree = RuleTree::new();


  let model_items = pairs.map(|p| ModelItem::new(p));
  let grouped_items = item_rules.into_iter()
                                .map(|r| (r, model_items.clone().filter(|i| i.rule == r)
                               .collect::<Vec<ModelItem>>())).collect::<GroupedItems>();

  Ok(ModelTree::new(&grouped_items, rule_tree))

}

#[derive(Debug, Clone)]
struct RuleTree {
  rule: Rule,
  children: Vec<RuleTree>
}

impl RuleTree {
  fn new() -> RuleTree {
    RuleTree{ rule: Rule::model, children: vec![
      RuleTree{ rule: Rule::aggregate, children: vec![
        RuleTree{ rule: Rule::entity, children: vec![]}
      ]}
    ]}
  }
}

type GroupedItems = HashMap<Rule, Vec<ModelItem>>;

#[derive(Debug, Clone)]
struct ModelItem {
  rule: Rule,
  value: String,
  source: String,
  start: (usize, usize),
  end: (usize, usize),
  name: String,
  properties: Vec<ModelItem>
}

impl ModelItem {
  fn new(pair: Pair<Rule>) -> ModelItem {
    

    let rule = pair.as_rule();
    let value = pair.as_str().to_string();
    let source = pair.as_span().as_str().to_string();
    let start = pair.as_span().start_pos().line_col();
    let end = pair.as_span().end_pos().line_col();
    let next_pairs = pair.into_inner();
    let name = next_pairs.peek().map(|p| p.as_str().to_string()).unwrap_or("".to_string());
    let properties = next_pairs.map(|p| ModelItem::new(p)).collect::<Vec<ModelItem>>();
    //println!("==== {:?}", properties);
    ModelItem {
      rule,
      value,
      source,
      start,
      end,
      name,
      properties
    }
  }

  fn parent(&self) -> Option<String> {
    Some(self.clone().properties.into_iter()
        .find(|p| p.rule == Rule::parent)?
        .properties.into_iter()
        .find(|p| p.rule == Rule::name)?
        .value)
  }

  fn property(&self, rule: Rule) -> Option<&ModelItem> {
    self.properties.iter().find(|i| i.rule == rule)
  }

  fn multi_property(&self, rule: Rule) -> Vec<ModelItem> {
    self.properties.clone().into_iter().filter(|i| i.rule == rule).collect()
  }
}

#[derive(Debug)]
struct ModelTree {
  item: ModelItem,
  children: Vec<ModelTree>
}

impl ModelTree {
  fn new(grouped_items: &GroupedItems, rule_tree: RuleTree) -> Vec<ModelTree> {

        grouped_items.get(&rule_tree.rule).unwrap().into_iter()
                .map(|item|
                     
         ModelTree {
          item: item.clone(),
          children: rule_tree.clone().children.into_iter()
                                      .map(|r| ModelTree::new(&grouped_items, r)).flatten().collect()
        }

                ).collect()

 
  }

}
/*
#[derive(Debug)]
struct ModelTree {
  item: HashMap<String, ModelItem>,
  children: HashMap<Rule, HashMap<String, ModelItem>>
}
*/



#[cfg(test)]
mod tests {
  use super::*;

  fn agg_with_ent() -> String {
    format!(r#"def agg person "Person" for expenses {{ }}
def model expenses "Expenses" {{ 
  default_lang = en 
  labels {{
    de = "DE Label"
    es = "es-ES Label"
  }}
}}
def ent person "Person" for person {{ }}"#)
  }

  fn two_aggs() -> String {
    format!(r#"def agg person "Person" for expenses  {{ }}
def model expenses "Expenses" {{ }}
def agg claim "Claim" for expenses {{ }}"#)
  }

  #[test]
  fn successful_parse() {
    let s = &agg_with_ent();
    let result = parse_model(s).unwrap();
    assert_eq!(result[0].item.rule, Rule::model);
    assert_eq!(result[0].item.name, "expenses");
    assert_eq!(result[0].item.properties.iter().find(|n| n.rule == Rule::label).unwrap().value, "\"Expenses\"");
    assert_eq!(result[0].children[0].item.rule, Rule::aggregate);
    assert_eq!(result[0].children[0].item.name, "person");
    assert_eq!(result[0].children[0].children[0].item.rule, Rule::entity);
    assert_eq!(result[0].children[0].children[0].item.name, "person");

    assert_eq!("en", result[0].item.property(Rule::default_lang).unwrap().property(Rule::lang_tag).unwrap().value);
    assert_eq!("\"DE Label\"", result[0].item.multi_property(Rule::lang_label)[0].property(Rule::label).unwrap().value);
    //println!("++++++ {:?}", result[0].item.clone().multi_property(Rule::lang_label));
  }
/*
  #[test]
  fn parse_two_aggs() {
    let s = &two_aggs();
    let result = parse_model(s);
    println!("****** {:?}", result);
  }
*/
}
