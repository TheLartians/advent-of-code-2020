use std::collections::HashSet;
use std::env;
use std::fs;

fn get_questions_in_group(group: &str) -> usize {
  let mut people = group.split('\n').filter(|v| v.len() > 0);
  let first = people.next().unwrap();
  let mut questions: HashSet<u8> = first.bytes().collect();
  for person in people {
    questions = person.bytes().filter(|c| questions.contains(c)).collect();
  }
  return questions.len();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();

  let question_count = input
    .split("\n\n")
    .filter(|s| s.len() > 0)
    .map(|s| get_questions_in_group(s))
    .fold(0, |a, b| a + b);

  println!(
    "Questions where everyone answered 'yes': {}",
    question_count
  )
}
