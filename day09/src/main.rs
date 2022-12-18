use std::{
  collections::HashMap,
  env,
  fs::File,
  io::{BufRead, BufReader},
};

fn pull_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
  if head.0.abs_diff(tail.0) > 1 && head.1.abs_diff(tail.1) > 1 {
    return ((head.0 - tail.0) / 2, (head.1 - tail.1) / 2);
  }
  if head.0.abs_diff(tail.0) > 1 {
    return ((head.0 - tail.0) / 2, (head.1 - tail.1));
  }
  if head.1.abs_diff(tail.1) > 1 {
    return (head.0 - tail.0, (head.1 - tail.1) / 2);
  }
  (0, 0)
}

fn run_simulation(filename: &String, num_knots: usize) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut knots: Vec<(i32, i32)> = Vec::new();
  for _ in 0..num_knots {
    knots.push((0, 0));
  }

  let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
  grid.insert(knots[0], true);

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let v: Vec<&str> = line.split(" ").collect();
      let dir = v[0];
      let num: i32 = v[1].parse().unwrap();
      for _ in 0..num {
        match dir {
          "U" => knots[0] = (knots[0].0, knots[0].1 + 1),
          "D" => knots[0] = (knots[0].0, knots[0].1 - 1),
          "R" => knots[0] = (knots[0].0 + 1, knots[0].1),
          "L" => knots[0] = (knots[0].0 - 1, knots[0].1),
          s => println!("Unknown movement {s}"),
        }

        for x in 0..num_knots - 1 {
          let delta = pull_tail(knots[x], knots[x + 1]);
          knots[x + 1] = (knots[x + 1].0 + delta.0, knots[x + 1].1 + delta.1);
          if x == num_knots - 2 {
            grid.insert(knots[x + 1], true);
          }
        }
      }
    }
  }
  println!("Answer for {} knots = {}", num_knots, grid.len());
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

fn part1(filename: &String) {
  run_simulation(filename, 2);
}

fn part2(filename: &String) {
  run_simulation(filename, 10);
}
