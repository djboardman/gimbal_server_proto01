use std::error;
use std::collections::HashMap;
use std::collections::BTreeSet;

use pest::{Parser, Span};
use pest::iterators::{Pair, Pairs};


#[derive(Debug, Clone)]
pub struct Model {
  name: String,
  default_label: String
}




#[derive(Debug, Clone)]
struct Definition {
  name: String,
  default_label: String
}

#[derive(Debug)]
pub enum ModelParseError {
  PestError(pest::error::Error<Rule>),
  TreeError(),
  NoModel(),
  DupName(String, String, (usize, usize), (usize, usize))
}

impl From<pest::error::Error<Rule>> for ModelParseError {
  fn from(err: pest::error::Error<Rule>) -> ModelParseError {
    ModelParseError::PestError(err)
  }
}


#[derive(Parser)]
#[grammar = "model.pest"]
pub struct ModelParser;


fn parse_model(source: &str) -> Result<ParseTree, ModelParseError> {
  //Ok(ParseTree::new_root(ModelParser::parse(Rule::file, source)?)?) //.map(|p| ParseTree::new(p))
  //Ok(ParsedPair::build_tree(&pairs))
  ParseTree::new_root(ModelParser::parse(Rule::file, source)?)
  
}





  //let pairs_copy = pairs.clone();

  //let parse_tree = pairs.
/*
  for stmt in pairs {
    match stmt.as_rule() {
      Rule::import => {
        println!("import * {}", stmt.into_inner().as_str());    

      },
      Rule::definition => {
        for def in stmt.into_inner() {
          //println!("def model * {}", def.into_inner().as_str());
          let d = definition(&def)?;
          return Ok(Model{ name: d.name, default_label: d.default_label });
        }
      }
      _ => {}
    }
  }
  Err(ModelParseError::NoModel())
}
*/





#[derive(Debug, Clone)]
struct ParsedPair {
  rule: Rule,
  value: String,
  source: String,
  start: (usize, usize),
  end: (usize, usize),
  name: Option<String>,
  children: Vec<ParsedPair>
}

impl ParsedPair {
  fn new(pair: Pair<Rule>) -> ParsedPair {
    let rule = pair.as_rule();
    let value = pair.as_str().to_string();
    let source = pair.as_span().as_str().to_string();
    let start = pair.as_span().start_pos().line_col();
    let end = pair.as_span().end_pos().line_col();
    let next_pairs = pair.into_inner();
    let name = next_pairs.peek().map(|p| p.as_str().to_string());
    println!("==== {:?}", name);
    let children = next_pairs.map(|p| ParsedPair::new(p)).collect::<Vec<ParsedPair>>();
    ParsedPair {
      rule,
      value,
      source,
      start,
      end,
      name,
      children
    }
  }
}



// type ParsedItems = HashMap<String, ParsedPair>;


  fn parsed_items_from_pairs(pairs: Pairs<Rule>) -> ParsedItems {
    let mut hm = ParsedItems::new();
    let _ = pairs.map(|p| ParsedPair::new(p)).map(|pp| hm.insert(pp.clone().value, pp));
    hm
  }

#[derive(Debug)]
struct ParseTree {
  nodes: HashMap<String, ParsedPair>
}

fn item_list(item_type: String, rule: Rule, pairs: Pairs<Rule>) -> Result<ParsedItems, ModelParseError> {
  let parsed_pairs = pairs.filter(|p| p.as_rule() == rule).map(|p| ParsedPair::new(p));
  let mut parsed_items: HashMap<String, ParsedPair> = HashMap::new();
  let mut name_index: BTreeSet<String> = BTreeSet::new();
  for parsed_pair in parsed_pairs {
    let next_name = parsed_pair.name.clone();
    match next_name {
      Some(n) => if name_index.contains(&n) {
        return Err(ModelParseError::DupName(item_type, n, parsed_pair.start, parsed_pair.end)); 
      } else {
        name_index.insert(n.clone());
        parsed_items.insert(n, parsed_pair);
      }
      None => {}
    }
  }
 
  Ok(parsed_items)
}

impl ParseTree {
  fn new_root(pairs: Pairs<Rule>, root_name: &str, root_name_singular: &str) -> Result<ParseTree, ModelParseError> {
    let root = HashMap::new();
    root.insert(root_name, item_list(root_name_singular.to_string(), Rule::agg, pairs)?);
    
/*
    //let mut aggs = ParsedItems::new();
    let aggs: ParsedItems = pairs.filter(|p| p.as_rule() == Rule::agg).map(|p| ParsedPair::new(p)).map(|p| (p.clone().into_inner().unwrap().as_str(), p)).collect();
    let mut ents = ParsedItems::new();
    let mut current_items: Option<&ParsedItems>;
    let mut current_item: &str;

    let parse_tree = pairs.map(|p| )

*/
    /*
    for pair in pairs {
      // The key of the HashMap needs to be the "name" from inside
      // Assuming unwrap cannot give an error because of parser rules
      let name = pair.clone().into_inner().next().unwrap().as_str(); 
      match pair.as_rule() { 
        Rule::agg => {
          current_items = Some(&aggs);
        }
        Rule::ent => {
 
        }
        _ =>  {  }
      }
      match current_items {
        Some(c) => {
          let e = check_dup_name(&c, name, &pair);
          if e.is_some() {return Err(e.unwrap()) };
          aggs = add_to_items(&c, name, pair);
        }
        None => {}
      }
    }
    */
    
  
  }
}

/*
fn check_dup_name(existing: &ParsedItems, new_name: &str, pair: &Pair<Rule>) -> Option<ModelParseError> {
  if existing.get(new_name).is_some() {
    Some(ModelParseError::DupName("aggregate".to_string(), new_name.to_string(), pair.as_span().start_pos().line_col(), pair.as_span().end_pos().line_col()))  
  } else {
    None
  }
}

fn add_to_items(mut items: &ParsedItems, name: &str, pair: Pair<Rule>) -> ParsedItems {
  items.insert(name.to_string(), ParsedPair::new(pair));
  items
}
*/
/*
type ParsedTree = HashMap<String, ParsedPair>;


impl ParseTree {
  fn new(pairs: &Pairs<Rule>) -> ParseTree {
    let mut hm = ParsedTree::new();
    pairs.map(|p| hm.insert(p.as_str().to_string(), ParsedPair()))
  }
}
*/

/*
impl ParsedPair {
  fn new(pair: &Pair<Rule>) -> ParsedPair2 {
    
    
    ParsedPair{ 
 
      rule: pair.as_rule(),
      value: pair.as_str().to_string(),
      children: Box::new(ParsedPair::build_tree(&pair.clone().into_inner()))
    }
  }

  fn build_tree(pairs: &Pairs<Rule>) -> Vec<ParsedPair> {
    pairs.clone().map(|p| ParsedPair::new(&p)).collect::<Vec<ParsedPair>>()
  }
}
*/



#[cfg(test)]
mod tests {
  use super::*;

  fn working_code() -> String {
    format!(r#"def agg person "Person" {{ }}
def ent person "Person" for person {{ }}"#
//def model person {{  }}"#
)
  }

  fn dup_name() -> String {
    format!(r#"def agg person "Person" {{ }}
def agg person "Person" {{ }}"#
    )
  }

  #[test]
  fn successful_parse() {
    let s = &working_code();
    //let pairs = ModelParser::parse(Rule::file, s).unwrap();
    let parse_tree = parse_model(s);
    println!("++++++ {:?}", parse_tree);
    /*
    let parse_tree = ParsedPair::build_tree(&pairs);
    println!("**** {:?}", parse_tree);
    assert_eq!(parse_tree[0].value, "person".to_string());
    */
    //assert_eq!(m.name, format!("person"));
    //println!("{}", p.into_inner().as_str());
    //println!("{}", p.into_inner());
  }


  #[test]
  fn reject_dup_name() {
    let s = &dup_name();
    //let parse_tree = ModelParser::parse(Rule::file, s).unwrap().map(|p| ParsedPair::new(p)).collect::<Vec<ParsedPair>>();
    let parse_tree = parse_model(s);
    println!("****** {:?}", parse_tree);
  }
}