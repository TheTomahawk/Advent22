use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut x: i32 = 1;
  let mut signal: Vec<i32> = Vec::new();
  signal.push(x); // ??

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      if line == "noop" {
        signal.push(x);
      } else {
        let mut parts = line.split(" ");
        let _ = parts.next();
        let d: i32 = parts.next().unwrap().parse().unwrap();
        signal.push(x);
        x += d;
        signal.push(x);
      }
    }
  }

  let len = signal.len();
  let mut counter: usize = 20 - 1;
  let mut sum = 0;
  while counter < len {
    sum += (counter + 1) as i32 * signal[counter];
    counter += 40;
  }

  println!("Part one: answer = {}", sum);

  for i in 0..6 {
    for j in 0..40 {
      let vecref = (40 * i) + j;
      let sprite_pos = signal[vecref];

      let v = j as i32;

      if v==sprite_pos-1 || v==sprite_pos || v==sprite_pos+1 {
        print!("#")
      } else {
        print!(".")
      }
    }
    println!("");
  }
}
