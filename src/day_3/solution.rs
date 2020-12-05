use std::collections::HashMap;

fn find_item(pattern: &Vec<String>, position: usize) -> bool {
  let tree_char = "#";
  let pattern_len = pattern.len();
  let borrow_pattern: Vec<_> = pattern.iter().map(|x| x).collect();


  return borrow_pattern[position % pattern_len] == tree_char;

  // Old long way
  // Keep making the pattern longer until the position is found

  // let tree_char: String = "#".to_string();
  // let mut item: String = "".to_string();
  // let mut found_item = false;
  // let borrow_pattern: Vec<_> = pattern.iter().map(|x| x).collect();
  // let mut repeat_trees: Vec<_> = pattern.iter().map(|x| x).collect();

  // while !found_item {
  //   let found_position = repeat_trees.iter().enumerate().find(|(idx, _char)| *idx == position );

  //   if found_position.is_none() {
  //     repeat_trees = repeat_trees.iter().chain(borrow_pattern.iter()).collect::<Vec<_>>().iter().map(|x| **x).collect::<Vec<_>>();

  //     continue;
  //   }

  //   let (_idx, c) = found_position.unwrap();

  //   item = c.to_string();
  //   found_item = true;
  // }

  // return item == tree_char;
}

// Ignore this, trying to reduce the amount of duplication with config maps
// fn setup_map(position_to_add: usize, position: usize, line_to_add: usize) -> HashMap<String, usize> {
//   let mut map = HashMap::new();

//   map.insert("position_to_add".to_string(), position_to_add);
//   map.insert("position".to_string(), position);
//   map.insert("line_to_add".to_string(), line_to_add);

//   return map;
// }

fn parse_file(path: &str) -> Vec<Vec<String>> {
  let mut reader = csv::Reader::from_path(path).unwrap();
  let mut values: Vec<Vec<String>> = Vec::new();

  for record in reader.records() {
    let r = record.unwrap();
    let r_arr = r.as_slice().chars().map(|x| x.to_string()).collect::<Vec<_>>();

    values.push(r_arr);
  }

  return values;
}

pub fn run() {
  let geo_map = parse_file("src/day_3/input.csv");

  let mut position_1: usize = 0;
  let mut position_3: usize = 0;
  let mut position_5: usize = 0;
  let mut position_7: usize = 0;
  let mut position_2_down: usize = 0;
  let mut position_2_down_line: u64 = 0;

  let mut tree_tracker: HashMap<&str, usize> = HashMap::new();

  let mut counter = 0;
  for row in geo_map {
    if counter == 0 {
      position_1 += 1;
      position_3 += 3;
      position_5 += 5;
      position_7 += 7;
      position_2_down += 1;
      position_2_down_line += 2;
      counter += 1;

      continue;
    }

    let item_1 = find_item(&row, position_1);
    let item_3 = find_item(&row, position_3);
    let item_5 = find_item(&row, position_5);
    let item_7 = find_item(&row, position_7);


    if item_1 {
      let count = tree_tracker.entry("position_1").or_insert(0);
      *count += 1;
    }
    if item_3 {
      let count = tree_tracker.entry("position_3").or_insert(0);
      *count += 1;
    }
    if item_5 {
      let count = tree_tracker.entry("position_5").or_insert(0);
      *count += 1;
    }
    if item_7 {
      let count = tree_tracker.entry("position_7").or_insert(0);
      *count += 1;
    }

    if position_2_down_line == counter {
      let item_2_down = find_item(&row, position_2_down);

      if item_2_down {
        let count = tree_tracker.entry("position_2_down").or_insert(0);
        *count += 1;
      }

      position_2_down += 1;
      position_2_down_line += 2;
    }

    position_1 += 1;
    position_3 += 3;
    position_5 += 5;
    position_7 += 7;
    counter += 1;
  }

  println!("Total trees {:?}", tree_tracker);

  println!("Multiply trees {:?}", tree_tracker.values().product::<usize>());
}