use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

pub fn main(){
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

fn part1(filename: &String) {
  let mut max = 0;
  let mut calories = 0;

  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  for input in infile.lines() {
    if let Ok(line) = input {
      if line.eq("") {
        if calories > max {
          max = calories;
        }
        calories = 0;
      } else {
        let x: i32 = line.trim().parse().expect("Input not an integer");
        calories += x;
      }
    }
  }
  println!("Part one answer is: {}", max);
}

fn part2(filename: &String) {
  let mut maxes = [0, 0, 0];
  let mut calories = 0;

  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  for input in infile.lines() {
    if let Ok(line) = input {
      if line.eq("") {
        if calories > maxes[0] {
          maxes[2] = maxes[1];
          maxes[1] = maxes[0];
          maxes[0] = calories;
        } else if calories > maxes[1] {
          maxes[2] = maxes[1];
          maxes[1] = calories;
        } else if calories > maxes[2] {
          maxes[2] = calories;
        }
        calories = 0;
      } else {
        let x: i32 = line.trim().parse().expect("Input not an integer");
        calories += x;
      }
    }
  }
  println!(
    "Part two answer is ({} + {} + {}): {}",
    maxes[0],
    maxes[1],
    maxes[2],
    maxes[0] + maxes[1] + maxes[2]
  );
}
