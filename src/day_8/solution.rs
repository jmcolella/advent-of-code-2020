use std::fs;
use std::collections::HashMap;
use regex;

fn parse_file() -> Vec<HashMap<String, String>> {
  let file = fs::read_to_string("src/day_8/input.txt").unwrap();

  let mut values: Vec<HashMap<String, String>> = Vec::new();

  let line_re = regex::Regex::new(r"^(?P<type>\D+) (?P<operation>\S)(?P<value>\d+)$").unwrap();
  
  for (idx, l) in file.lines().enumerate() {
    let captures = line_re.captures(l).unwrap();
    let op_type = captures.name("type").unwrap().as_str();
    let op = captures.name("operation").unwrap().as_str();
    let val = captures.name("value").unwrap().as_str();

    let mut map: HashMap<String, String> = HashMap::new();

    map.entry("operation_type".to_string()).or_insert(op_type.to_string());
    map.entry("operation".to_string()).or_insert(op.to_string());
    map.entry("value".to_string()).or_insert(val.to_string());
    map.entry("position".to_string()).or_insert(idx.to_string());

    values.push(map);
  }


  return values;
}

fn all_variations(instructions: &Vec<HashMap<String, String>>) -> Vec<Vec<HashMap<String, String>>> {
  let mut all: Vec<Vec<HashMap<String, String>>> = Vec::new();

  for (idx, i) in instructions.iter().enumerate() {
    let mut instructions_copy = instructions.clone();

    if i.get("operation_type").unwrap().as_str() == "nop" {
      instructions_copy[idx].entry("operation_type".to_string()).and_modify(|s| *s = "jmp".to_string());
    } else if i.get("operation_type").unwrap().as_str() == "jmp" {
      instructions_copy[idx].entry("operation_type".to_string()).and_modify(|s| *s = "nop".to_string());
    }

    all.push(instructions_copy);
  }

  return all;
}

fn do_operation(operation: &str, left: i32, right: &str) -> i32 {
  let parsed_right = right.parse::<i32>().unwrap();

  match operation {
    "+" => left + parsed_right,
    "-" => left - parsed_right,
    _ => 0
  }
}

fn part_two(instructions: &Vec<HashMap<String, String>>) {
  let all_v = all_variations(instructions);

  let mut accumulator = 0;
  let mut tracker = 0;

  for (idx, v) in all_v.iter().enumerate() {
    println!("idx {}", idx);

    for _j in v.iter() {
      if (tracker as usize) == v.len() {
        break;
      }

      let i = v.iter().find(|x| x.get("position").unwrap().as_str().parse::<i32>().unwrap() == tracker).unwrap();
      let operation_type = i.get("operation_type").unwrap().as_str();
      let operation = i.get("operation").unwrap().as_str();
      let value = i.get("value").unwrap().as_str();


      match operation_type {
        "nop" => {
          tracker += 1;
        },
        "acc" => {
          let next_acc = do_operation(operation, accumulator, value);

          accumulator = next_acc;
          tracker += 1;
        },
        "jmp" => {
          let new_line = do_operation(operation, tracker, value);
          
          tracker = new_line;
        },
        _ => println!("No operation type match")
      }
    }

    if (tracker as usize) == v.len() {
      break;
    }

    accumulator = 0;
    tracker = 0;
  }

  println!("Termination accumulator {}", accumulator);
}

pub fn run() {
  let instructions = parse_file();

  let mut completed_positions: Vec<i32> = Vec::new();
  let mut accumulator = 0;
  let mut tracker = 0;

  while !(completed_positions.contains(&tracker)) {
    let i = instructions.iter().find(|x| x.get("position").unwrap().as_str().parse::<i32>().unwrap() == tracker).unwrap();
    let operation_type = i.get("operation_type").unwrap().as_str();
    let operation = i.get("operation").unwrap().as_str();
    let value = i.get("value").unwrap().as_str();

    match operation_type {
      "nop" => {
        completed_positions.push(tracker);
        tracker += 1;
      },
      "acc" => {
        let next_acc = do_operation(operation, accumulator, value);

        accumulator = next_acc;
        completed_positions.push(tracker);
        tracker += 1;
      },
      "jmp" => {
        let new_line = do_operation(operation, tracker, value);
        
        completed_positions.push(tracker);
        tracker = new_line;
      },
      _ => println!("No operation type match")
    }
  }

  part_two(&instructions);

  println!("Accumulator {}", accumulator);
  println!("Tracker {}", tracker);
}