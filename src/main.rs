use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

fn parse_ingredients(input: &str) -> (Vec<String>, Vec<String>) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
  }
  let capture = RE.captures(input).unwrap();
  let ingredients = capture[1]
    .split(" ")
    .map(|s| String::from(s))
    .collect::<Vec<_>>();
  let allergens = capture[2]
    .split(", ")
    .map(|s| String::from(s))
    .collect::<Vec<_>>();
  return (ingredients, allergens);
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|line| line.ok())
    .map(|s| parse_ingredients(&s))
    .collect::<Vec<_>>();

  let mut allergen_sets: HashMap<&str, HashSet<&str>> = HashMap::new();

  for product in &input {
    let ingredients_set = product
      .0
      .iter()
      .map(|s| s.as_str())
      .collect::<HashSet<&str>>();

    for allergen in &product.1 {
      let allergen_ingredients = allergen_sets
        .entry(&allergen)
        .or_insert_with(|| ingredients_set.clone());

      let intersection = allergen_ingredients
        .intersection(&ingredients_set)
        .map(|&s| s)
        .collect::<HashSet<&str>>();

      allergen_sets.insert(&allergen, intersection);
    }
  }

  let mut ingredients_set: HashSet<&str> = input
    .iter()
    .map(|(ingredients, _)| ingredients)
    .flatten()
    .map(|s| s.as_str())
    .collect();

  for (_, ingredients) in &allergen_sets {
    for ingredient in ingredients {
      ingredients_set.remove(ingredient);
    }
  }

  let mut result = 0;
  for product in &input {
    for ingredient in &product.0 {
      if ingredients_set.contains(ingredient.as_str()) {
        result += 1;
      }
    }
  }

  println!("the result is {}", result);
}
