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

fn run_bytecode<'a>(instructions: &'a Vec<(Instruction, i32)>) -> Result<i32, i32> {
  let mut accumulator = 0;
  let mut position = 0;
  let mut history: Vec<bool> = instructions.iter().map(|_| false).collect();

  loop {
    if position == instructions.len() {
      return Ok(accumulator);
    } else if history[position] {
      return Err(accumulator);
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
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());

  let mut instructions: Vec<(Instruction, i32)> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| parse_instruction(&s))
    .collect();

  let mut result: Result<i32, i32> = Err(0);
  for i in 0..instructions.len() {
    match instructions[i].0 {
      Instruction::JMP => {
        instructions[i].0 = Instruction::NOP;
        result = run_bytecode(&instructions);
        instructions[i].0 = Instruction::JMP;
      }
      Instruction::NOP => {
        instructions[i].0 = Instruction::JMP;
        result = run_bytecode(&instructions);
        instructions[i].0 = Instruction::NOP;
      }
      _ => {}
    }
    if result.is_ok() {
      println!("changed instruction at {} and got result {:?}", i, result);
      break;
    }
  }
}
