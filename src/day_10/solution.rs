use std::fs;
use std::collections::HashMap;

fn parse_file() -> Vec<usize> {
  let file = fs::read_to_string("src/day_10/input.txt").unwrap();

  let mut values: Vec<usize> = Vec::new();

  for l in file.lines() {
    values.push(l.parse::<usize>().unwrap());
  }


  return values;
}


pub fn run() {
  let mut adapters = parse_file().clone();

  let mut chain: Vec<usize> = vec![0];
  let mut differences: HashMap<usize, usize> = HashMap::new();

  let mut all_possible_chains: Vec<Vec<usize>> = vec![vec![0]];

  let max_adapter = adapters.iter().max().unwrap();
  let built_in_adapter = max_adapter + 3;
  adapters.push(built_in_adapter);
  adapters.sort();

  for a in adapters {
    let diff = a - chain[chain.len() - 1];

    if diff <= 3{
      chain.push(a);

      let diff_entry = differences.entry(diff).or_insert(0);
      *diff_entry += 1;
    }
  }

  let mut c = 0;

  while c < adapters.len() {

  }



  let one_diff = differences.get(&(1 as usize)).unwrap();
  let three_diff = differences.get(&(3 as usize)).unwrap();
  println!("one diff * three diff {}", one_diff * three_diff);
}