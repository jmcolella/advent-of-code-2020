use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn parse_file() -> HashMap<String, HashMap<String, String>> {
  let file = fs::read_to_string("src/day_7/input.txt").unwrap();

  let line_re = Regex::new(r"^(?P<bag_type>[a-z]+ [a-z]*) bags contain (?P<rest>.+)\.").unwrap();
  let inner_bags_re = Regex::new(r"^(?P<num>(\d+|no)) (?P<type>\D+) bag(s*)$").unwrap();

  let mut values: HashMap<String, HashMap<String, String>> = HashMap::new();
  for l in file.lines() {
    let line_caps = line_re.captures(l).unwrap();
    let bag_type = line_caps.name("bag_type").unwrap().as_str();
    let inner_bags = line_caps.name("rest").unwrap().as_str().split(", ").collect::<Vec<_>>();

    for i in inner_bags {
      let bags_map: HashMap<String, String> = HashMap::new();
      let bags = inner_bags_re.captures(i).unwrap();
      let num = bags.name("num").unwrap().as_str().to_string();
      let inner_type = bags.name("type").unwrap().as_str().to_string();

      if num == "no" {
        values.entry(bag_type.to_string()).or_insert(bags_map);

        continue;
      }

      let test = values.entry(bag_type.to_string()).or_insert(bags_map);
      test.entry(inner_type).or_insert(num);
    }
  }

  return values;
}

fn get_bags(bag: &String, map: &HashMap<String, HashMap<String, String>>) -> Vec<String> {
  let mut values: Vec<String> = Vec::new();

  for (k, v) in map {
    let has_bag = v.keys().find(|x| *x == bag);

    if has_bag.is_some() {
      values.push(k.to_string());

      values.extend(get_bags(k, map));
    }
  }

  values.sort();
  values.dedup();

  return values;
}

fn get_num(bag: &String,  map: &HashMap<String, HashMap<String, String>>) -> u64 {
  let mut val = 1;
  if bag == "shiny gold" {
    val = 0;
  }

  let bag_rules = map.get(bag);
  if bag_rules.is_some() {
    let rules = bag_rules.unwrap();

    for (r, num) in rules {
      let parsed = num.parse::<u64>().unwrap();

      val += parsed * get_num(r, map);
    }
  }

  return val;
}

pub fn run() {
  let my_bag = "shiny gold".to_string();

  let bags_map = parse_file();

  let found_bags = get_bags(&my_bag, &bags_map);

  let shiny_gold_num = get_num(&my_bag, &bags_map);

  println!("found bags count {:?}", found_bags.len());
  println!("shiny gold num {:?}", shiny_gold_num);

}