use csv;
use regex::Regex;

fn parse_file() -> Vec<Vec<String>> {
  let mut reader = csv::Reader::from_path("src/day_5/input.csv").unwrap();

  let mut values: Vec<Vec<String>> = Vec::new();

  let line_re = Regex::new(r"^([FB]+)([LR]+)$").unwrap();

  for r in reader.records() {
    let record = r.unwrap();

    let captures = line_re.captures(record.as_slice()).unwrap();

    let row = &captures[1];
    let column = &captures[2];

    values.push(vec![row.to_string(), column.to_string()]);
  }

  return values;
}

fn find_item(item: &str, range: &Vec<u64>, lower: &str, upper: &str) -> u64 {
  let item_len = item.len();

  if item_len == 1 {
    if item == lower {
      return range[0];
    } else {
      return range[1];
    }
  }


  let current_val = item.get(0..1).unwrap();
  let next_item = item.get(1..=item_len-1).unwrap();
  let half_range = range.len() / 2;

  if current_val == lower {
    let (left, _) = range.split_at(half_range);

    return find_item(next_item, &left.to_vec(), lower, upper);
  }

  if current_val == upper {
    let (_, right) = range.split_at(half_range);

    return find_item(next_item, &right.to_vec(), lower, upper);
  }

  return 0;
}

pub fn run() {
  let boarding_passes = parse_file();

  let mut rows: Vec<u64> = Vec::new();
  let mut cols: Vec<u64> = Vec::new();

  for i in 0..=127 {
    rows.push(i);
  }

  for j in 0..=7 {
    cols.push(j);
  }

  let mut highest_seat_id = 0;

  let mut all_seats: Vec<u64> = Vec::new();

  for p in boarding_passes {
    let found_row = find_item(&p[0], &rows, "F", "B");
    let found_col = find_item(&p[1], &cols, "L", "R");

    let seat_id = (found_row * 8) + found_col;

    all_seats.push(seat_id);

    if seat_id > highest_seat_id {
      highest_seat_id = seat_id;
    }
  }

  all_seats.sort();

  for (idx, s) in all_seats.iter().enumerate() {
    if (idx+1) >= all_seats.len() {
      break;
    }

    if s + 1 != all_seats[idx+1] {
      println!("My seat {}", s+1);
    }
  }


  println!("Highest Seat Id {}", highest_seat_id);
}