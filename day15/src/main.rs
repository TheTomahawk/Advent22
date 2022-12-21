use regex::Regex;
use std::{
  cmp,
  collections::HashMap,
  env,
  fs::File,
  io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
struct Point {
  x: i64,
  y: i64,
}
impl Point {
  fn new(x: i64, y: i64) -> Point {
    Point { x: x, y: y }
  }
}
type Sensor = Point;
type Beacon = Point;
struct Pair {
  sensor: Sensor,
  beacon: Beacon,
  dist: u64,
}
impl Pair {
  fn new(s: Sensor, b: Beacon) -> Pair {
    Pair {
      dist: manhatten_distance(&s, &b),
      sensor: s,
      beacon: b,
    }
  }
}
pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let re = Regex::new(
    r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$",
  )
  .unwrap();

  let mut max_x = i64::MIN;
  let mut min_x = i64::MAX;

  let mut max_y = i64::MIN;
  let mut min_y = i64::MAX;

  let mut beacons: HashMap<Beacon, bool> = HashMap::new();
  let mut pairs: Vec<Pair> = Vec::new();

  let mut min_dist = u64::MAX;

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      for num in re.captures_iter(&line) {
        let sx: i64 = num[1].parse().unwrap();
        let sy: i64 = num[2].parse().unwrap();
        let bx: i64 = num[3].parse().unwrap();
        let by: i64 = num[4].parse().unwrap();

        beacons.insert(Point::new(bx, by), true);

        let pair = Pair::new(Point::new(sx, sy), Point::new(bx, by));
        (min_x, min_y, max_x, max_y) = update_bounds(&pair, min_y, min_y, max_x, max_y);

        if pair.dist < min_dist {
          min_dist = pair.dist;
        }

        pairs.push(pair);
      }
    }
  }
  println!("Bounds: {},{} to {},{}\n", min_x, min_y, max_x, max_y);

  let mut count = 0;
  let y = 2000000;
  for x in min_x..=max_x {
    for i in 0..pairs.len() {
      let t = Point::new(x, y);
      if beacons.contains_key(&t) {
        break;
      }
      let s = &pairs[i].sensor;
      let m = manhatten_distance(s, &t);

      if m <= pairs[i].dist {
        count += 1;
        break;
      }
    }
  }

  println!("Part one answer: {count}");

  let mut possibilities: Vec<Point> = Vec::new();
  for i in 0..pairs.len() {
    let mut locus = calculate_locus(&pairs[i].sensor, pairs[i].dist + 1);
    possibilities.append(&mut locus);
  }

  possibilities.sort();
  possibilities.dedup();

  println!("\nChecking {} possibilities...", possibilities.len());

  for point in possibilities {
    if point.x < 0 || point.y < 0 || point.x > 4000000 || point.y > 4000000 {
      // don't check them -- we don't care about them.
      continue;
    }

    let mut count = 0;
    for i in 0..pairs.len() {
      let s = &pairs[i].sensor;
      let m = manhatten_distance(s, &point);

      if m > pairs[i].dist {
        count += 1;
      } else {
        break;
      }
    }
    if count == pairs.len() {
      println!(
        "x:{}, y:{}, fq:{}",
        point.x,
        point.y,
        (point.x * 4000000) + point.y
      );
    }
  }
}

fn manhatten_distance(a: &Point, b: &Point) -> u64 {
  a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn update_bounds(
  pair: &Pair,
  mut min_x: i64,
  mut min_y: i64,
  mut max_x: i64,
  mut max_y: i64,
) -> (i64, i64, i64, i64) {
  let left = cmp::min(pair.beacon.x, pair.sensor.x) - pair.dist as i64;
  let top = cmp::min(pair.beacon.y, pair.sensor.x) - pair.dist as i64;
  let right = cmp::max(pair.beacon.x, pair.sensor.x) + pair.dist as i64;
  let bottom = cmp::max(pair.beacon.y, pair.sensor.y) + pair.dist as i64;

  if left < min_x {
    min_x = left;
  }
  if right > max_x {
    max_x = right;
  }
  if top < min_y {
    min_y = top;
  }
  if bottom > max_y {
    max_y = bottom;
  }
  (min_x, min_y, max_x, max_y)
}

fn calculate_locus(p: &Point, dist: u64) -> Vec<Point> {
  let mut ret: Vec<Point> = Vec::new();
  for i in 0..(dist as i64) {
    let dy = (dist as i64) - i;
    ret.push(Point::new(p.x - i, p.y - dy));
    ret.push(Point::new(p.x - i, p.y + dy));
    ret.push(Point::new(p.x + i, p.y - dy));
    ret.push(Point::new(p.x + i, p.y + dy));
  }

  ret
}
