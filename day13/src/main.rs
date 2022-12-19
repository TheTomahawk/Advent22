use std::{
  env, fmt,
  fs::File,
  io::{BufRead, BufReader},
  str::Chars,
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

#[derive(Eq, PartialEq)]
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

impl fmt::Debug for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if !self.is_list {
      write!(f, "{}", self.val)
    } else {
      write!(f, "{:?}", self.list)
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
          let (ret, _) = compare_lists(&left, &right);
          if ret {
            sum += counter;
          }
          counter += 1;
        },
        _ => {}
      }
      c += 1;
      c %= 3;
    }
  }

  // EOF won't kick the counter up one, so we need one last compare
  let (ret, _) = compare_lists(&left, &right);
  if ret {
    sum += counter;
  }

  println!("Part one: answer = {sum}");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut all_the_lines: Vec<Vec<Node>> = Vec::new();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let l = parse_line(&line);
      if l.len() != 0 {
        all_the_lines.push(l);
      }
    }
  }

  all_the_lines.push(val_as_list_as_vec(2));
  all_the_lines.push(val_as_list_as_vec(6));

  all_the_lines.sort_by(|left,right| if compare_lists(left, right).0 { std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater});

  let mut product = 1;
  let comp2 = val_as_list(2);
  let comp6 = val_as_list(6);

  for i in 0..all_the_lines.len() {
    if all_the_lines[i].contains(&comp2) || all_the_lines[i].contains(&comp6) {
      product *= i+1;
    }
  }

  println!("Part two: answer = {}", product);
}

fn val_as_list(val: u16) -> Node {
  let mut n = Node::new();
  n.is_list = false;
  n.val = val;
  let mut as_list = Node::new();
  as_list.is_list = true;
  as_list.list.push(n);

  return as_list;
}

fn val_as_list_as_vec(val: u16) -> Vec<Node> {
  let mut as_vec: Vec<Node> = Vec::new();
  as_vec.push(val_as_list(val));

  return as_vec;
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
  (chars, list)
}

fn parse_line(line: &String) -> Vec<Node> {
  let mut ret = Vec::new();

  let chars = line.chars();
  (_, ret) = parse_chars(chars, ret);

  ret
}

fn compare_lists(left: &Vec<Node>, right: &Vec<Node>) -> (bool, bool) {
  let mut ret = true;

  if left.len() == 0 { // left ran out of terms
    return (true, right.len() == 0);

  } else if right.len() == 0 {  // right ran out of terms, but left didn't
    return (false, false);

  } else { // compare values in the lists
    for n in 0..left.len() {
      if n < right.len() {  
        let (r,c) = compare_nodes(&left[n], &right[n]);
        ret &= r;
        if !r {
          return (false, false);
        }
        if !c {
          return (ret, false);
        }
      } else { // right ran out of terms, but left didn't
        return (false, false);
      }
    }
  }
  (ret, ret)
}

fn compare_nodes(left: &Node, right: &Node) -> (bool, bool) {
  if left.is_list && right.is_list {
    return compare_lists(&left.list, &right.list);

  } else if left.is_list && !right.is_list {
    let mut val_as_list = Node::new();
    val_as_list.is_list = true;
    let mut new_node = Node::new();
    new_node.val = right.val;
    val_as_list.list.push(new_node);
    return compare_nodes(left, &val_as_list);

  } else if !left.is_list && right.is_list {
    let mut val_as_list = Node::new();
    val_as_list.is_list = true;
    let mut new_node = Node::new();
    new_node.val = left.val;
    val_as_list.list.push(new_node);
    return compare_nodes(&val_as_list, right);

  } else {
    if left.val < right.val { // left < right => correct order
      return (true, false);

    } else if left.val > right.val {  // left > left => wrong order
      return (false, false);

    } else {  // left == right => keep checking
      return (true, true);
    }
  }
}
