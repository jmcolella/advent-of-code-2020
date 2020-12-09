use std::fs;

fn parse_file() -> Vec<usize> {
  let file = fs::read_to_string("src/day_9/input.txt").unwrap();

  let mut values: Vec<usize> = Vec::new();

  for l in file.lines() {
    values.push(l.parse::<usize>().unwrap());
  }


  return values;
}

fn part_one(list: &Vec<usize>) -> usize {
  let preamble = 25;
  let mut counter = preamble;
  let mut invalid_val: usize = 0;

  while (counter as usize) <= list.len() {
    let mut valid_next_val = false;
    let bottom = (counter - preamble) as usize;
    let top = (counter - 1) as usize;
    let section = &list[bottom..=top];
    let val_to_check = list[counter] as usize;

    for i in section {
      for j in section {
        if i +j == val_to_check {
          valid_next_val = true;
        }
      }
    }

    if !valid_next_val {
      invalid_val = val_to_check;
      break;
    }

    counter += 1;
  }

  return invalid_val;
}

fn part_two(invalid_val: &usize, list: &Vec<usize>) -> usize {
  let mut invalid_sum = 0;
  let mut outer_counter: usize = 0;
  let mut inner_counter: usize = 0;

  let list_len = &list.len() - 1;

  while outer_counter < list_len {
    let i = &list[outer_counter];
    let mut found_range = false;
    let mut invalid_range: Vec<usize> = vec![*i];
    let mut running_sum: usize = *i;

    while inner_counter + 1 < list_len {
      let j = &list[inner_counter + 1];

      if running_sum + j == *invalid_val {
        found_range = true;
        invalid_range.push(*j);

        break;
      }

      if running_sum + j > *invalid_val {
        inner_counter = outer_counter + 1;
        running_sum = 0;

        break;
      }

      invalid_range.push(*j);
      running_sum += j;
      inner_counter += 1;
    }

    if found_range {
      invalid_range.sort();

      println!("invalid range {:?}", invalid_range);

      invalid_sum = invalid_range.iter().min().unwrap() + invalid_range.iter().max().unwrap();
      break;
    }
    outer_counter += 1;
  }

  return invalid_sum;
}

pub fn run() {
  let xmas = parse_file();

  let invalid_val = part_one(&xmas);

  println!("invalid val {}", invalid_val);

  let invalid_sum = part_two(&invalid_val, &xmas);

  println!("invalid range sum {}", invalid_sum);
}