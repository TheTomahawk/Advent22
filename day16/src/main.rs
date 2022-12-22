use regex::Regex;
use std::{
  cmp,
  collections::HashMap,
  env,
  fs::File,
  io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
struct Valve {
  rate: u32,
  tunnels: Vec<String>,
  is_open: bool,
  visited: bool,
}

impl Valve {
  fn new(rate: u32) -> Valve {
    Valve {
      rate: rate,
      tunnels: Vec::new(),
      is_open: false,
      visited: false,
    }
  }
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let line_re =
    Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)$").unwrap();

  let mut valves: HashMap<String, Valve> = HashMap::new();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let mut valve: Valve;
      for cap in line_re.captures_iter(&line) {
        let name: String = cap[1].parse().unwrap();
        println!("  {name}");
        let rate: u32 = cap[2].parse().unwrap();
        valve = Valve::new(rate);

        let list: String = cap[3].parse().unwrap();
        let split = list.split(", ");
        for tunnel in split {
          valve.tunnels.push(tunnel.to_string());
        }
        valves.insert(name, valve);
      }
    }
  }

  println!("{:?}", valves);

  part1(&valves);
  part2();
}

fn part1(valves: &HashMap<String, Valve>) {
  let start = "AA".to_string();


  println!("Part one: answer = ");
}

fn find_max_path(
  valves: &HashMap<String, Valve>,
  path_from_start: &Vec<String>,
  next_valve: String,
  counter: u32,
  max_steps: u32,
) -> (Vec<String>, u32) {
  if counter + 1 == max_steps {
    return (Vec::new(), 0);
  }

  println!("I am {:?}", next_valve);

  let valve = valves.get(&next_valve).expect("");
  let mut path_to_here = path_from_start.clone();
  path_to_here.push(String::from(&next_valve));

  println!("   Path to here: {:?}", path_to_here);


  let mut possible_paths: Vec<(Vec<String>, u32)> = Vec::new();

  for i in 0..valve.tunnels.len() {
    let v = &valve.tunnels[i];
    if !path_from_start.contains(&v) {
      let (p, s) = find_max_path(valves, &path_to_here, v.to_string(), counter + 1, max_steps);
      possible_paths.push((p, s));
    }
  }

  if possible_paths.len() > 0 {
    let mut max: u32 = 0;
    let mut index: usize = 0;
    for i in 0..possible_paths.len() {
      let (p, s) = &possible_paths[i];
      if *s > max {
        index = i;
        max = *s;
      }
    }
    possible_paths[index].0.insert(0, String::from(next_valve));
    return (
      possible_paths[index].0.clone(),
      possible_paths[index].1 + (max_steps - counter) * valve.rate,
    );
  } else {
    return (Vec::new(), 0);
  }
}

fn part2() {
  println!("Part two: answer = ");
}
