use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  both_parts(&filename);
}

fn check_visiblity(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> (bool, usize) {
  let height = forest.len();
  let width = forest[0].len();
  let val = forest[x][y];
  let mut result: [bool; 4] = [true; 4];
  let mut dist: [usize; 4] = [1; 4];

  let mut count: usize = 0;
  for i in (0..x).rev() {
    count += 1;
    if val <= forest[i][y] {
      result[0] = false;
      break;
    }
  }
  dist[0] = count;

  count = 0;
  for i in x + 1..width {
    count += 1;
    if val <= forest[i][y] {
      result[1] = false;
      break;
    }
  }
  dist[1] = count;

  count = 0;
  for i in (0..y).rev() {
    count += 1;
    if val <= forest[x][i] {
      result[2] = false;
      break;
    }
  }
  dist[2] = count;

  count = 0;
  for i in y + 1..height {
    count += 1;
    if val <= forest[x][i] {
      result[3] = false;
      break;
    }
  }
  dist[3] = count;

  return (
    result[0] | result[1] | result[2] | result[3],
    dist[0] * dist[1] * dist[2] * dist[3],
  );
}

fn count_visible_trees(forest: &Vec<Vec<u32>>) -> (usize, usize) {
  let mut count: usize = 0;
  let height = forest.len();
  let width = forest[0].len();
  count += (height * 2) + (width * 2) - 4; // the corners are doubled

  let mut max_scenic_score: usize = 1;
  for x in 1..width - 1 {
    for y in 1..height - 1 {
      let (visible, scenic_score) = check_visiblity(&forest, x, y);
      if visible {
        count += 1
      }
      if scenic_score > max_scenic_score {
        max_scenic_score = scenic_score;
      }
    }
  }

  (count, max_scenic_score)
}

fn both_parts(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut forest: Vec<Vec<u32>> = Vec::new();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let mut row: Vec<u32> = Vec::new();
      for c in line.chars() {
        row.push(c.to_digit(10).unwrap());
      }
      forest.push(row);
    }
  }

  let (count, max_scenic_score) = count_visible_trees(&forest);

  println!("Part one: answer = {count}");
  println!("Part two: answer = {max_scenic_score}");
}
