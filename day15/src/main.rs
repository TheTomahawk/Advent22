use regex::Regex;
use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

type Sensor = (i32, i32);
type Beacon = (i32, i32);
type Pair = (Sensor, Beacon);

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  // Input looks like:
  // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
  let re = Regex::new(
    r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$",
  )
  .unwrap();

  let mut max_x = i32::MIN;
  let mut min_x = i32::MAX;

  let mut max_y = i32::MIN;
  let mut min_y = i32::MAX;

  let mut sensors: Vec<Sensor> = Vec::new();
  let mut beacons: Vec<Beacon> = Vec::new();
  let mut pairs: Vec<Pair> = Vec::new();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      for num in re.captures_iter(&line) {
        let sx: i32 = num[1].parse().unwrap();
        let sy: i32 = num[2].parse().unwrap();
        let bx: i32 = num[3].parse().unwrap();
        let by: i32 = num[4].parse().unwrap();
        let sensor = (sx, sy);
        let beacon = (bx, by);
        sensors.push(sensor);
        beacons.push(beacon);
        pairs.push((sensor, beacon));

        if sx < min_x {
          min_x = sx;
        }
        if sx > max_x {
          max_x = sx;
        }
        if sy < min_y {
          min_y = sy;
        }
        if sy > max_y {
          max_y = sy;
        }
        if bx < min_x {
          min_x = bx;
        }
        if bx > max_x {
          max_x = bx;
        }
        if by < min_y {
          min_y = by;
        }
        if by > max_y {
          max_y = by;
        }
      }
    }
  }
  println!("Bounds: {},{} to {},{}\n", min_x, min_y, max_x, max_y);
  println!("Sensors: {:?}\n", sensors);
  println!("Beacons: {:?}\n", beacons);
  println!("Pairs: {:?}", pairs);
  // calculate Manahatten distance from each sensor to each beacon
  // for each point on the line, calculate the Manhattan distance to each sensor.
  // if it's a beacon, skip it.
  // if it's less than the distance to it's nearest beacon then... something...
  // if it's more, then... something else...
}

fn manhatten_distance() -> i32 {
  
}