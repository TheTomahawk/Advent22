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

fn decode_char(character: char) -> u32 {
  let lower_a = 'a' as u32;
  let lower_z = 'z' as u32;
  let upper_a = 'A' as u32;
  let upper_z = 'Z' as u32;

  let c = character as u32;

  let ret: u32;

  if c >= lower_a && c <= lower_z {
    ret = 26 - (lower_z - c);
  } else if c >= upper_a && c <= upper_z {
    ret = (26 - (upper_z - c)) + 26;
  } else {
    ret = 0;
  }

  ret
}

fn encode_char(mut num: u64) -> u32 {
  let mut val: u32 = 0;
  while num > 1 {
    if num != 1 {
      val += 1;
    }
    num = num >> 1;
  }
  val
}

fn parse_string(str: &String) -> u32 {
  let len = str.len() / 2;
  let mut the_string = String::from(str);

  let binding = the_string.split_off(len);
  let mut left = binding.chars();
  let mut right = the_string.chars();

  let mut l: u64 = 0;
  let mut r: u64 = 0;
  for _ in 0..len {
    l |= 1 << decode_char(left.next().unwrap());
    r |= 1 << decode_char(right.next().unwrap());
  }
  let dup = l & r;
  encode_char(dup)
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut score: u32 = 0;
  for input in infile.lines() {
    if let Ok(line) = input {
      score += parse_string(&line);
    }
  }
  println!("Part one: Total score = {score}");
}

fn parse_line(line: &String) -> u64 {
  let chars = line.chars();

  let mut r: u64 = 0;
  for c in chars {
    r |= 1 << decode_char(c);
  }
  r
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut count = 0;

  let mut pattern: [u64; 3] = [0; 3];

  let mut sum = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      pattern[count] = parse_line(&line);
      count += 1;
      if count == 3 {
        let dup = pattern[0] & pattern[1] & pattern[2];
        sum += encode_char(dup);
        count = 0;
      }
    }
  }
  println!("Part two: Total score = {sum}");
}
