use std::collections::HashMap;
use std::fs;

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

  let max_adapter = adapters.iter().max().unwrap();
  let built_in_adapter = max_adapter + 3;
  adapters.push(built_in_adapter);
  adapters.sort();

  for a in adapters {
    let diff = a - chain[chain.len() - 1];

    if diff <= 3 {
      chain.push(a);

      let diff_entry = differences.entry(diff).or_insert(0);
      *diff_entry += 1;
    }
  }

  let mut permutations = 1;

  let mut stuff: Vec<usize> = Vec::new();

  // let mut others: Vec<&Vec<usize>> = vec![&chain];
  println!("chain {:?}", chain);

  // let mut counter = 0;
  // let mut left_off = 0;

  // Try to build all of the chain permutations but i don't know how
  // while counter <= &chain.len() - 1 {}

  for i in &chain {
    let mut additional_paths = 0;

    for j in &chain {
      if i == j {
        println!("i {}", i);
        println!("addititonal paths {}", additional_paths);
        if additional_paths > 1 {
          stuff.push(additional_paths as usize);
          permutations += additional_paths - 1;
        }
        break;
      }

      if i - j <= 3 {
        additional_paths += 1;
      }
    }
  }

  // for i in &chain {
  //   let mut additional_paths = 0;
  //   let mut reverse_counter = chain.len() - 1;

  //   while reverse_counter > 0 {
  //     if &chain[reverse_counter] == i {
  //       println!("i backwards {}", i);
  //       println!("addititonal paths backwards {}", additional_paths);
  //       if additional_paths > 0 {
  //         permutations += additional_paths - 1;
  //       }
  //       break;
  //     }

  //     if &chain[reverse_counter] - i <= 3 {
  //       additional_paths += 1;
  //     }

  //     reverse_counter -= 1;
  //   }
  // }

  println!("stuff {:?}", stuff);

  println!("total paths {}", permutations);

  let one_diff = differences.get(&(1 as usize)).unwrap();
  let three_diff = differences.get(&(3 as usize)).unwrap();
  println!("one diff * three diff {}", one_diff * three_diff);
}
