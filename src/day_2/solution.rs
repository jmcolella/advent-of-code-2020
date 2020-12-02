use csv;
use regex::Regex;

pub fn run() {
  let mut reader = csv::Reader::from_path("src/day_2/input.csv").unwrap();

  let line_re = Regex::new(r"(\d+)-(\d+) ([a-z]): (\D+)").unwrap();
  let mut valid_passwords = 0;

  for result in reader.records() {
    let reader = result.unwrap();

    let cap = line_re.captures(reader.as_slice()).unwrap();

    let lower_bound = &cap[1].parse::<usize>().unwrap();
    let upper_bound = &cap[2].parse::<usize>().unwrap();
    let target = &cap[3];
    let password = &cap[4];
    
    // Part 1
    // Find passwords that include the target within the bounds
    // let matches: Vec<&str> = password.rmatches(target).collect();
    // let matches_amount = matches.len();

    // if matches_amount >= *lower_bound && matches_amount <= *upper_bound {
    //   valid_passwords += 1;
    // }

    let matched_indices: Vec<usize> = password.match_indices(target).map(|(idx, _)| idx).collect();
    let first_match = matched_indices.contains(&(lower_bound - 1));
    let second_match = matched_indices.contains(&(upper_bound - 1));

    if first_match && second_match {
      continue;
    }

    if first_match || second_match {
      valid_passwords += 1;
    }
  }

  println!("Good passwords: {}", valid_passwords);
}
