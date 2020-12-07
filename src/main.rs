use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn push_dependencies<'a>(
  res: &mut HashSet<&'a str>,
  item: &str,
  dep_map: &'a HashMap<&str, Vec<&str>>,
) {
  match dep_map.get(item) {
    Some(deps) => {
      for dep in deps {
        if !res.contains(dep) {
          push_dependencies(res, dep, dep_map);
          res.insert(dep);
        }
      }
    }
    None => {}
  }
}

fn get_dependencies<'a>(item: &str, dep_map: &'a HashMap<&str, Vec<&str>>) -> HashSet<&'a str> {
  let mut res = HashSet::new();
  push_dependencies(&mut res, item, dep_map);
  return res;
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

  let mut dependencies: HashMap<&str, Vec<&str>> = HashMap::new();
  for (name, content) in &rules {
    for (bag, _) in content {
      dependencies.entry(bag).or_default().push(&name);
    }
  }

  println!(
    "shiny gold can be contained in {} bags.",
    get_dependencies("shiny gold", &dependencies).len()
  );
}
