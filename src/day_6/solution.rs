use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn parse_file() -> Vec<Vec<String>> {
  let file = fs::read_to_string("src/day_6/input.txt").unwrap();
  let lines = file.as_str().lines();

  let mut values: Vec<Vec<String>> = Vec::new();

  let mut temp: Vec<String> = Vec::new();
  for l in lines {
    if l.len() == 0 {
      values.push(temp);

      temp = vec![];

      continue;
    }

    temp.push(l.to_string());
  }

  if temp.len() != 0 {
    values.push(temp);
  }

  return values;
}

pub fn run() {
  let answers = parse_file();
  let mut unique_ans_per_group: HashMap<usize, usize> = HashMap::new();
  let mut unique_ans_all_group: HashMap<usize, usize> = HashMap::new();

  for (idx, ans) in answers.iter().enumerate() {
    let mut ans_map: HashMap<char, usize> = HashMap::new();
    let ans_str = ans.iter().join("");

    for c in ans_str.chars() {
      let a = ans_map.entry(c).or_insert(0);
      *a += 1
    }

    unique_ans_per_group.entry(idx).or_insert(ans_map.keys().len());

    let ans_all_group = ans_map.values().filter(|x| **x == ans.len()).collect::<Vec<_>>().len();

    unique_ans_all_group.entry(idx).or_insert(ans_all_group);
  }

  println!("Unique Answers {:?}", unique_ans_per_group.values().sum::<usize>());
  println!("Unique All Answers {:?}", unique_ans_all_group.values().sum::<usize>());
}