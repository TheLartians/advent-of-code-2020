use regex::Regex;
use std::env;
use std::fs;

fn is_in_range(input: &str, unit: &str, min: i32, max: i32) -> bool {
  Regex::new(&format!(r"^(\d+|[a-f]){}$", unit))
    .unwrap()
    .captures(input)
    .and_then(|m| m[1].parse::<i32>().ok())
    .and_then(|v| Some(v >= min && v <= max))
    .unwrap_or(false)
}

fn is_valid_field(key: &str, input: &str) -> bool {
  match key {
    "byr" => is_in_range(input, "", 1920, 2002),
    "iyr" => is_in_range(input, "", 2010, 2020),
    "eyr" => is_in_range(input, "", 2020, 2030),
    "hgt" => is_in_range(input, "cm", 150, 193) || is_in_range(input, "in", 59, 76),
    "hcl" => Regex::new(r"^#(\d|[a-f]){6}$").unwrap().is_match(input),
    "ecl" => Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
      .unwrap()
      .is_match(input),
    "pid" => Regex::new(r"^\d{9}$").unwrap().is_match(input),
    _ => false,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1]).unwrap();
  let passports: Vec<&str> = contents.split("\n\n").collect();

  let required_field_count = 7;
  let mut current_field_count = 0;

  for passport in passports {
    // parse fields
    // assume no duplicate entries
    let valid_fields = Regex::new(r"(\n| )")
      .unwrap()
      .split(passport)
      .filter(|s| s.len() > 0)
      .map(|s| s.split(":").collect::<Vec<&str>>())
      .filter_map(|v| match v.len() == 2 && is_valid_field(v[0], v[1]) {
        true => Some((v[0], v[1])),
        false => None,
      })
      .count();

    // check if all required fields are there
    if valid_fields == required_field_count {
      current_field_count = current_field_count + 1;
    }
  }

  println!("Counted {:?} valid passports", current_field_count)
}
