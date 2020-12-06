use std::collections::HashSet;
use std::env;
use std::fs;

fn get_questions_in_group(group: &str) -> usize {
  let mut questions = HashSet::new();
  for person in group.split('\n') {
    for question in person.chars() {
      questions.insert(question);
    }
  }
  return questions.len();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();

  let question_count = input.split("\n\n")
  .filter(|s| s.len() > 0)
  .map(|s| get_questions_in_group(s))
  .fold(0, |a,b| a+b);
  
  println!("Number of questions with at least one 'yes': {}", question_count)
}
