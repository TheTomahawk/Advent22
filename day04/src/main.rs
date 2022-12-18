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

// outer and inner are of the form number-number (e.g. 4-19)
/** does outer contain inner? */
fn contains(outer: &str, inner: &str) -> bool {
  let outer_str = String::from(outer);
  let mut outer_range = outer_str.split("-");
  let outer_lower = outer_range.next().unwrap().parse::<u32>().unwrap();
  let outer_upper = outer_range.next().unwrap().parse::<u32>().unwrap();

  let inner_str = String::from(inner);
  let mut inner_range = inner_str.split("-");
  let inner_lower = inner_range.next().unwrap().parse::<u32>().unwrap();
  let inner_upper = inner_range.next().unwrap().parse::<u32>().unwrap();

  inner_lower >= outer_lower && inner_upper <= outer_upper
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut count: u32 = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      let mut assignments = line.split(",");
      let elf_a = assignments.next().unwrap();
      let elf_b = assignments.next().unwrap();
      if contains(elf_a, elf_b) || contains(elf_b, elf_a) {
        count += 1;
      }
    }
  }
  println!("Part one: answer = {count}");
}

/** does first overlap with second? */
fn overlap(first: &str, second: &str) -> bool {
  let first_str = String::from(first);
  let mut first_range = first_str.split("-");
  let first_lower = first_range.next().unwrap().parse::<u32>().unwrap();
  let first_upper = first_range.next().unwrap().parse::<u32>().unwrap();

  let second_str = String::from(second);
  let mut second_range = second_str.split("-");
  let second_lower = second_range.next().unwrap().parse::<u32>().unwrap();
  let second_upper = second_range.next().unwrap().parse::<u32>().unwrap();

  (first_lower >= second_lower && first_lower <= second_upper)
    || (first_upper >= second_lower && first_upper <= second_upper)
    || (second_lower >= first_lower && second_lower <= first_upper)
    || (second_upper >= first_lower && second_upper <= first_upper)
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut count: u32 = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      let mut assignments = line.split(",");
      let elf_a = assignments.next().unwrap();
      let elf_b = assignments.next().unwrap();
      if overlap(elf_a, elf_b) || contains(elf_b, elf_a) {
        count += 1;
      }
    }
  }
  println!("Part two: answer = {count}");
}
