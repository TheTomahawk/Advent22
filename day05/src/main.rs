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

fn parse_stacks(line: &String) -> (Vec<char>, usize) {
  let len = line.len();
  let chars: Vec<_> = line.chars().collect();

  let num_cols = (len + 1) / 4;

  let mut cols: Vec<char> = Vec::with_capacity(num_cols);

  let mut count = 0;

  for i in 0..num_cols {
    let num = 1 + (i * 4);
    if !chars[num].is_numeric() {
      cols.push(chars[num]);
      count += 1;
    }
  }

  (cols, count)
}

fn parse_instructions(line: &String) -> (usize, usize, usize) {
  let parts: Vec<_> = line.split(" ").collect();

  (
    parts[1].parse().unwrap(),
    parts[3].parse().unwrap(),
    parts[5].parse().unwrap(),
  )
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut stacks: Vec<Vec<char>> = Vec::new();

  let mut lines = infile.lines();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      if line == "" {
        break;
      }
      let (crates, num) = parse_stacks(&line);
      if stacks.is_empty() {
        for _ in 0..num {
          stacks.push(Vec::new());
        }
      }
      for i in 0..num {
        if crates[i] != ' ' {
          stacks[i].push(crates[i]);
        }
      }
    }
  }

  for i in 0..stacks.len() {
    stacks[i].reverse();
  }

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let (num, from, to) = parse_instructions(&line);
      for i in 0..num {
        let block = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(block)
      }
    }
  }

  print!("Part one: answer = ");
  for i in 0..stacks.len() {
    print!("{}", stacks[i].pop().unwrap());
  }
  println!("");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut stacks: Vec<Vec<char>> = Vec::new();

  let mut lines = infile.lines();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      if line == "" {
        break;
      }
      let (crates, num) = parse_stacks(&line);
      if stacks.is_empty() {
        for _ in 0..num {
          stacks.push(Vec::new());
        }
      }
      for i in 0..num {
        if crates[i] != ' ' {
          stacks[i].push(crates[i]);
        }
      }
    }
  }

  for i in 0..stacks.len() {
    stacks[i].reverse();
  }

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let (num, from, to) = parse_instructions(&line);
      let mut blocks: Vec<char> = Vec::new();
      for i in 0..num {
        let block = stacks[from - 1].pop().unwrap();
        blocks.push(block);
      }
      for i in 0..num {
        let block = blocks.pop().unwrap();
        stacks[to - 1].push(block);
      }
    }
  }

  print!("Part two: answer = ");
  for i in 0..stacks.len() {
    print!("{}", stacks[i].pop().unwrap());
  }
  println!("");
}
