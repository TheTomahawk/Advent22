use regex::Regex;
use std::{
  cmp,
  collections::HashMap,
  env,
  fs::File,
  io::{BufRead, BufReader},
};

// #[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let re = Regex::new(
    r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$",
  ).unwrap();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      // for num in re.captures_iter(&line) {
      // }
    }
  }

  part1();
  part2();
}

fn part1() {
  println!("Part one: answer = ");
}

fn part2() {
  println!("Part two: answer = ");
}
