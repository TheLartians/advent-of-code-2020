use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn is_in_range(input: &str, unit: &str, min: i32, max: i32) -> bool {
   Regex::new(&format!(r"^(\d+|[a-f]){}$", unit))
    .unwrap()
    .captures(input)
    .and_then(|m| Some(m[1].parse::<i32>().and_then(|v| Ok(v >= min && v <= max))))
    .unwrap_or(Ok(false))
    .unwrap()
}

fn is_valid_pp_entry(key: &str, input: &str) -> bool {
  match key {
    "byr" => is_in_range(input, "", 1920, 2002),
    "iyr" => is_in_range(input, "", 2010, 2020),
    "eyr" => is_in_range(input, "", 2020, 2030),
    "hgt" => is_in_range(input, "cm", 150, 193) || is_in_range(input, "in", 59, 76),
    "hcl" => Regex::new(r"^#(\d|[a-f]){6}$").unwrap().is_match(input),
    "ecl" => Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(input),
    "pid" => Regex::new(r"^\d{9}$").unwrap().is_match(input),
    _ => false,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("reading file {:?}", args[1]);

  let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
  let passports: Vec<&str> = contents.split("\n\n").collect();
  let separator = Regex::new(r"(\n| )").expect("Invalid regex");

  let mut valid = 0;
  let expected = 7;

  for passport in passports {
    // parse input and filter valid properties
    let parts: Vec<(&str, &str)> = separator
      .split(passport)
      .filter(|s| s.len() > 0)
      .map(|s| s.split(":").collect::<Vec<&str>>())
      .filter(|v| v.len() == 2 && is_valid_pp_entry(v[0], v[1]))
      .map(|v| (v[0], v[1]))
      .collect();

    // prevent duplicate entries from beeing counted
    let mut unique: HashSet<&str> = HashSet::new();

    for part in parts {
      unique.insert(part.0);
    }
    if unique.len() == expected {
      valid = valid + 1;
    }
  }
  println!("Counted {:?} valid passports", valid)
}
