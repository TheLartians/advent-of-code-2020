use pest::iterators::Pair;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct NewMathParser;

type Scalar = i64;

fn evaluate_expression(pair: Pair<Rule>) -> Scalar {
  match pair.as_rule() {
    Rule::number => pair.as_str().parse::<Scalar>().unwrap(),
    Rule::atom => evaluate_expression(pair.into_inner().next().unwrap()),
    Rule::addition | Rule::multiplication => {
      let mut inner_rules = pair.into_inner();
      let mut result = evaluate_expression(inner_rules.next().unwrap());
      while let Some(op) = inner_rules.next() {
        let rhs = evaluate_expression(inner_rules.next().unwrap());
        match op.as_str() {
          "+" => result += rhs,
          "*" => result *= rhs,
          _ => unreachable!(),
        }
      }
      return result;
    }
    Rule::brackets => evaluate_expression(pair.into_inner().next().unwrap()),
    Rule::expression => evaluate_expression(pair.into_inner().next().unwrap()),
    _ => unreachable!(),
  }
}

fn parse_expression(expr: &str) -> Scalar {
  let result = evaluate_expression(
    NewMathParser::parse(Rule::expression, &expr)
      .unwrap_or_else(|e| panic!("{}", e))
      .next()
      .unwrap(),
  );
  println!("{} = {}", expr, result);
  return result;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|line| line.ok())
    .filter(|s| s.len() > 0);

  let result = input.map(|s| parse_expression(&s)).fold(0, |a, b| a + b);
  println!("the result is {}", result);
}
