use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn join_vec(v: Vec<&str>) -> String {
  return v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
}

fn parse_file() -> Vec<String> {
  let mut values = Vec::new();

  let file = fs::read_to_string("src/day_4/input.txt").unwrap();
  let lines = file.split("\n").collect::<Vec<_>>();

  let mut temp: Vec<&str> = Vec::new();
  for i in lines {
    if i.len() == 0 {
      values.push(join_vec(temp));

      temp = vec![];
      
      continue;
    }

    temp.push(i);
  }

  if temp.len() != 0 {
    values.push(join_vec(temp));
  }

  return values;
}

fn get_passport_rules() -> HashMap<String, regex::Regex> {
  let mut regex_map: HashMap<String, regex::Regex> = HashMap::new();
  let byr_re = Regex::new(r"^(1|2)(9[2-9][0-9]|00[0-2])$").unwrap();
  let iyr_re = Regex::new(r"^20(1[0-9]|20)$").unwrap();
  let eyr_re = Regex::new(r"^20(2[0-9]|30)$").unwrap();
  let hgt_re = Regex::new(r"^((59|6[0-9]|7[0-6])in|1([5-8][0-9]|9[0-3])cm)$").unwrap();
  let hcl_re = Regex::new(r"^\#[a-z0-9]{6}$").unwrap();
  let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
  let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();

  regex_map.entry("byr".to_string()).or_insert(byr_re);
  regex_map.entry("iyr".to_string()).or_insert(iyr_re);
  regex_map.entry("eyr".to_string()).or_insert(eyr_re);
  regex_map.entry("hgt".to_string()).or_insert(hgt_re);
  regex_map.entry("hcl".to_string()).or_insert(hcl_re);
  regex_map.entry("ecl".to_string()).or_insert(ecl_re);
  regex_map.entry("pid".to_string()).or_insert(pid_re);

  return regex_map;
}

pub fn run() {
  let passports = parse_file();
  let passport_rules = get_passport_rules();
  let mut valid_passports = 0;

  let line_re = Regex::new(r"([a-z]{3}:\S+)").unwrap();

  for p in passports {
    let mut valid = true;

    let captures = line_re.find_iter(&p);
    let mut passport_map: HashMap<String, &str> = HashMap::new();
    for c in captures {
      let key_value = c.as_str().split(":").collect::<Vec<_>>();

      passport_map.entry(key_value[0].to_string()).or_insert(key_value[1]);
    }

    for (k, reg) in &passport_rules {
      let item = passport_map.get(k);

      if item.is_none() {
        valid = false;

        continue;
      }

      if !(reg.is_match(item.unwrap())) {
        valid = false;
      }
    }

    if valid {
      valid_passports += 1;
    }
  }

  println!("Valid passports {}", valid_passports);
}