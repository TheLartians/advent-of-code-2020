use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

enum Instruction {
  ACC,
  JMP,
  NOP,
}

fn parse_instruction(input: &str) -> (Instruction, i32) {
  let mut parts = input.split(' ');
  let instruction = match parts.next().unwrap() {
    "acc" => Instruction::ACC,
    "jmp" => Instruction::JMP,
    _ => Instruction::NOP,
  };
  let value = parts.next().unwrap().parse::<i32>().unwrap();
  return (instruction, value);
}

fn run_until_loop(instructions: &Vec<(Instruction, i32)>) -> i32 {
  let mut accumulator = 0;
  let mut position = 0;
  let mut history: Vec<bool> = instructions.iter().map(|_| false).collect();

  loop {
    if history[position] {
      break;
    } else {
      history[position] = true;
      let (instruction, value) = &instructions[position];
      match instruction {
        Instruction::ACC => {
          accumulator += value;
          position += 1;
        }
        Instruction::JMP => {
          position = position.wrapping_add(*value as usize);
        }
        Instruction::NOP => {
          position += 1;
        }
      }
    }
  }

  return accumulator;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());

  let instructions: Vec<(Instruction, i32)> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| parse_instruction(&s))
    .collect();

  println!(
    "the program repeated the loop with {}",
    run_until_loop(&instructions)
  );
}
