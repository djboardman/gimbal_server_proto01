use std::error;
use std::collections::HashMap;

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
  NoModel()
}

impl From<pest::error::Error<Rule>> for ModelParseError {
  fn from(err: pest::error::Error<Rule>) -> ModelParseError {
    ModelParseError::PestError(err)
  }
}



#[derive(Parser)]
#[grammar = "model.pest"]
pub struct ModelParser;


fn parse_model(source: &str) -> Result<ParseTree, pest::error::Error<Rule>> {
  Ok(ParseTree::new_root(ModelParser::parse(Rule::file, source)?)) //.map(|p| ParseTree::new(p))
  //Ok(ParsedPair::build_tree(&pairs))
  
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
  children: Vec<ParsedPair>
}

impl ParsedPair {
  fn new(pair: Pair<Rule>) -> ParsedPair {
    ParsedPair {
      rule: pair.as_rule(),
      value: pair.as_str().to_string(),
      source: pair.as_span().as_str().to_string(),
      start: pair.as_span().start_pos().line_col(),
      end: pair.as_span().end_pos().line_col(),
      children: pair.into_inner().map(|p| ParsedPair::new(p)).collect::<Vec<ParsedPair>>()
    }
  }
}



type ParsedItems = HashMap<String, ParsedPair>;


  fn parsed_items_from_pairs(pairs: Pairs<Rule>) -> ParsedItems {
    let mut hm = ParsedItems::new();
    let _ = pairs.map(|p| ParsedPair::new(p)).map(|pp| hm.insert(pp.clone().value, pp));
    hm
  }

#[derive(Debug)]
struct ParseTree {
  aggs: ParsedItems,
  ents: ParsedItems
}

impl ParseTree {


  fn new_root(pairs: Pairs<Rule>) -> ParseTree {
    let mut aggs = ParsedItems::new();
    let mut ents = ParsedItems::new();

    for pair in pairs {
      match pair.as_rule() {
        Rule::agg => {
          // The key of the HashMap needs to be the "name" from inside
          // Assuming unwrap cannot give an error because of parser rules
          aggs.insert(pair.clone().into_inner().next().unwrap().as_str().to_string(), ParsedPair::new(pair));
        }
        Rule::ent => {
          ents.insert(pair.clone().into_inner().next().unwrap().as_str().to_string(), ParsedPair::new(pair));
        }
        _ =>  {  }
      }
    }
    ParseTree {
      aggs: aggs,
      ents: ents
    }
  }
}



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

  fn import_file() -> String {
    format!(r#"def agg person "Person" {{ }}
def ent person "Person" for person {{ }}"#
//def model person {{  }}"#
)
  }

  #[test]
  fn successful_parse() {
    let s = &import_file();
    //let pairs = ModelParser::parse(Rule::file, s).unwrap();
    let parse_tree = parse_model(s);
    println!("****** {:?}", parse_tree);
    /*
    let parse_tree = ParsedPair::build_tree(&pairs);
    println!("**** {:?}", parse_tree);
    assert_eq!(parse_tree[0].value, "person".to_string());
    */
    //assert_eq!(m.name, format!("person"));
    //println!("{}", p.into_inner().as_str());
    //println!("{}", p.into_inner());
  }
}