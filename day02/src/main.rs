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

fn parse_string(str: &String) -> (char, char) {
  let mut chars = str.chars();
  let first = chars.next().unwrap();
  let _ = chars.next();
  let third = chars.next().unwrap();

  (first, third)
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut score: i32 = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      let (yours, mine) = parse_string(&line);

      match mine {
        'X' => score += 1,
        'Y' => score += 2,
        'Z' => score += 3,
        _ => {}
      }

      // check for a win
      if (yours == 'A' && mine == 'Y')
        || (yours == 'B' && mine == 'Z')
        || (yours == 'C' && mine == 'X')
      {
        score += 6; // winner
      }

      // check for a draw
      if (yours == 'A' && mine == 'X')
        || (yours == 'B' && mine == 'Y')
        || (yours == 'C' && mine == 'Z')
      {
        score += 3;
      }
    }
  }
  println!("Part one: Total score = {score}");
}

fn choice_score(choice: char) -> i32 {
  match choice {
    'A' => 1,
    'B' => 2,
    'C' => 3,
    _ => 0
  }
}


fn winning_move(choice: char) -> char {
  match choice {
    'A' => 'B',
    'B' => 'C',
    'C' => 'A',
    _ => choice
  }
}

fn losing_move(choice: char) -> char {
  match choice {
    'A' => 'C',
    'B' => 'A',
    'C' => 'B',
    _ => choice
  }
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut score: i32 = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      let (opponent, result) = parse_string(&line);
      match result {
        'X' => { // win
          score+=choice_score(losing_move(opponent));
        },
        'Y' => { // draw
          score+=choice_score(opponent);
          score+=3;

        },
        'Z' => { // lose
          score+=choice_score(winning_move(opponent));
          score+=6;
        },
        _ => {},
      }
    }
  }
  println!("Part two: Total score = {score}");
}
