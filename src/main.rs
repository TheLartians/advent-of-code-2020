use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn parse_rule_part(input: &str) -> (String, u32) {
  lazy_static! {
    static ref RULE_PART_REGEX: Regex = Regex::new(r"^(\d) ([a-z ]*) (?:bag|bags)$").unwrap();
  }

  let capture = RULE_PART_REGEX.captures(input).unwrap();

  return (
    String::from(capture.get(2).unwrap().as_str()),
    capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
  );
}

fn parse_rule(input: &str) -> (String, Vec<(String, u32)>) {
  lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z ]*) bags contain ([\da-z, ]*).$").unwrap();
  }

  let capture = RULE_REGEX.captures(input).unwrap();
  let name = String::from(capture.get(1).unwrap().as_str());
  let content_definitions = capture.get(2).unwrap().as_str();

  let content = content_definitions
    .split(", ")
    .filter(|s| *s != "no other bags")
    .map(|s| parse_rule_part(s))
    .collect();
  return (name, content);
}

fn add_inner_bags<'a>(
  res: &mut HashMap<&'a str, u32>,
  item: &'a str,
  rules: &'a HashMap<String, Vec<(String, u32)>>,
) -> u32 {
  let mut total = 1;

  match rules.get(item) {
    Some(rule) => {
      for (color, inner_count) in rule {
        match res.get(color as &str) {
          Some(content_count) => {
            total += inner_count * content_count;
          }
          None => total += inner_count * add_inner_bags(res, color, rules),
        }
      }
    }
    None => {}
  }

  res.insert(&item, total);
  return total;
}

fn get_inner_bag_count(item: &str, rules: &HashMap<String, Vec<(String, u32)>>) -> u32 {
  let mut res = HashMap::new();
  return add_inner_bags(&mut res, item, rules) - 1;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());

  let rules: HashMap<String, Vec<(String, u32)>> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| parse_rule(&s))
    .collect();

  println!(
    "shiny gold contains {} other bags.",
    get_inner_bag_count("shiny gold", &rules)
  );
}
