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

  let mut allergen_ingredients = HashMap::new();

  while allergen_sets.len() > 0 {
    let to_remove = allergen_sets
      .iter()
      .filter(|(_, i)| i.len() == 1)
      .map(|(&a, i)| (a, *i.iter().next().unwrap()))
      .collect::<Vec<_>>();
    for (allergen, ingredient) in to_remove {
      allergen_sets.remove(allergen);
      allergen_ingredients.insert(allergen, ingredient);
      for (_, ingredients) in &mut allergen_sets {
        ingredients.remove(ingredient);
      }
    }
  }

  println!(
    "allergen_ingredients: {:?}",
    allergen_ingredients
      .iter()
      .sorted()
      .map(|(_, i)| i)
      .join(",")
  );
}
