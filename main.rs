use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  while let Some(input) = lines.next() {
    if let Ok(line) = input {
    }
  }

  println!("Part one: answer = ");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  while let Some(input) = lines.next() {
    if let Ok(line) = input {
    }
  }

  println!("Part two: answer = ");
}
