use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

lazy_static! {
  static ref RANGE_REGEX: Regex = Regex::new(r"^(\d+|[a-f])(.*)").unwrap();
  static ref COLOR_CODE_REGEX: Regex = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
  static ref EYE_COLOR_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  static ref ID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn is_in_range(input: &str, unit: &str, min: i32, max: i32) -> bool {
  RANGE_REGEX
    .captures(input)
    .and_then(|m| if &m[2] == unit { Some(m) } else { None })
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
    "hcl" => COLOR_CODE_REGEX.is_match(input),
    "ecl" => EYE_COLOR_REGEX.is_match(input),
    "pid" => ID_REGEX.is_match(input),
    _ => false,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1]).unwrap();
  let passports: Vec<&str> = contents.split("\n\n").collect();

  let required_field_count = 7;
  let mut current_field_count = 0;

  let field_separator_regex = Regex::new(r"(\n| )").unwrap();

  for passport in passports {
    // parse fields
    // assume no duplicate entries
    let valid_fields = field_separator_regex
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
