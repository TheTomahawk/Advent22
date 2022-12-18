use std::{
  env, fmt,
  fmt::{Error, Formatter},
  fs::File,
  io::{BufRead, BufReader},
  str::Chars,
};

#[derive(Debug,PartialEq)]
enum Comparison {
  LT,
  GT,
  EQ,
  NONE
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

#[derive(Debug)]
struct Node {
  val: u16,
  is_list: bool,
  list: Vec<Node>,
}

impl Node {
  fn new() -> Node {
    Node {
      val: 0,
      is_list: false,
      list: Vec::new(),
    }
  }
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut c = 0;
  let mut counter = 1;
  let mut right: Vec<Node> = Vec::new();
  let mut left = Vec::new();
  let mut sum = 0;

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      match c {
        0 => left = parse_line(&line),
        1 => right = parse_line(&line),
        2 => {
          if compare_lists(&left, &right) {
            sum += counter
          };
        }
        _ => {}
      }
      c += 1;
      c %= 3;
      if c == 0 {
        counter += 1;
      }
    }
  }

  // EOF won't kick the counter up one, so we need one last compare
  if compare_lists(&left, &right) {
    sum += counter
  };

  println!("Part one: answer = {sum}");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  while let Some(input) = lines.next() {
    if let Ok(line) = input {}
  }

  println!("Part two: answer = ");
}

fn parse_chars(mut chars: Chars, mut list: Vec<Node>) -> (Chars, Vec<Node>) {
  let mut snum = String::new();

  while let Some(c) = chars.next() {
    match c {
      '[' => {
        let mut node = Node::new();
        node.is_list = true;
        (chars, node.list) = parse_chars(chars, node.list);
        list.push(node);
      }
      ']' => {
        if snum.len() > 0 {
          let num = snum.parse().unwrap();
          let mut node = Node::new();
          node.val = num;
          list.push(node);
        }
        break;
      }
      ',' => {
        if snum.len() > 0 {
          let num = snum.parse().unwrap();
          let mut node = Node::new();
          node.val = num;
          list.push(node);
        }
        snum = String::new();
      }
      d => {
        snum.push(d);
      }
    }
  }

  // println!("  returning list: {:?}", list);
  (chars, list)
}

fn parse_line(line: &String) -> Vec<Node> {
  let mut ret = Vec::new();
  println!("Parsing: {}", line);
  // can be a number [ ] or ,
  let chars = line.chars();

  (_, ret) = parse_chars(chars, ret);

  // println!("Returning: {:?}", ret);
  ret
}

fn compare_lists(left: &Vec<Node>, right: &Vec<Node>) -> bool {
  println!("Comparing...");
  println!("   left: {:?}", left);
  println!("  right: {:?}", right);
  println!("");

  let mut ret = true;

  if left.len() == 0 || right.len() == 0 {
    ret = true;
    
  } else {
    for n in 0..left.len() {
      if n < right.len() {
        if compare_nodes(&left[n], &right[n]).0 == Comparison::LT {
          break;
        }
      } else {
        ret = false;
      }
    }
  }
  println!("  Result:  {}\n\n", ret);
  ret
}

fn compare_nodes(left: &Node, right: &Node) -> (Comparison, bool) {
  print!("n");
  let mut ret:(Comparison, bool);

  if left.is_list && right.is_list {
    ret = (Comparison::NONE, compare_lists(&left.list, &right.list));

  } else if left.is_list && !right.is_list {
    let mut val_as_list = Node::new();
    val_as_list.is_list = true;
    let mut new_node = Node::new();
    new_node.val = right.val;
    val_as_list.list.push(new_node);
    ret = compare_nodes(left, &val_as_list);

  } else if !left.is_list && right.is_list {
    let mut val_as_list = Node::new();
    val_as_list.is_list = true;
    let mut new_node = Node::new();
    new_node.val = left.val;
    val_as_list.list.push(new_node);
    ret = compare_nodes(&val_as_list, right);

  } else {
    ret = compare_vals(left.val, right.val);
  }

  println!(" : {:?}", ret);
  ret
}

fn compare_vals(left: u16, right: u16) -> (Comparison, bool) {
  print!("v");

  let ret;

  if left == right {
    ret = Comparison::EQ
  } else if left < right {
    ret = Comparison::LT
  } else {
    ret = Comparison::GT
  }

  println!(" {}<={}: {:?}", left, right, ret);
  (ret, true)
}
