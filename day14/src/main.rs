use std::{
  env, fmt,
  fs::File,
  io::{BufRead, BufReader},
};

struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point { x: x, y: y }
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let (mut rocks, mut top_left, mut bottom_right) = parse_file(&filename);
  simulate(&rocks, Point::new(500, 0), &top_left, &bottom_right);

  let expansion = bottom_right.y+5;

  // let mid_x = (( bottom_right.x - top_left.x ) / 2) + top_left.x;
  // for i in mid_x-expansion..mid_x+expansion {
  //   rocks.push(Point::new(i, bottom_right.y+2));
  // }

  top_left.x -= expansion;
  bottom_right.x += expansion;
  bottom_right.y += 2;

  for i in top_left.x ..=bottom_right.x {
    rocks.push(Point::new(i, bottom_right.y));
  }

  simulate(&rocks, Point::new(500,0), &top_left, &bottom_right);

}

fn parse_file(filename: &String) -> (Vec<Point>, Point, Point) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut rocks: Vec<Point> = Vec::new();
  let mut min_x: usize = usize::MAX;
  let mut max_x: usize = 0;
  let mut min_y: usize = usize::MAX;
  let mut max_y: usize = 0;

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      let coords = line.split(" -> ");
      let mut c = 0;
      let mut inputs: Vec<Point> = Vec::new();
      for coord in coords {
        let xy: Vec<&str> = coord.split(",").collect();
        let x: usize = xy[0].parse().unwrap();
        let y: usize = xy[1].parse().unwrap();
        inputs.push(Point::new(x, y));
        if x < min_x {
          min_x = x
        };
        if y < min_y {
          min_y = y
        };
        if x > max_x {
          max_x = x
        };
        if y > max_y {
          max_y = y
        };

        if c > 0 {
          rocks.extend(calculate_line(&inputs[c - 1], &inputs[c]));
        } else {
          rocks.push(Point::new(x, y));
        }
        c += 1;
      }
    }
  }

  println!(
    "bounds are ({}, {}) and ({}, {})",
    min_x, min_y, max_x, max_y
  );

  (rocks, Point::new(min_x, min_y), Point::new(max_x, max_y))
}

fn calculate_line(start: &Point, end: &Point) -> Vec<Point> {
  let mut ret: Vec<Point> = Vec::new();

  if start.x == end.x {
    if start.y < end.y {
      for i in start.y + 1..=end.y {
        ret.push(Point::new(start.x, i));
      }
    } else {
      for i in (end.y..start.y).rev() {
        ret.push(Point::new(start.x, i));
      }
    }
  } else if start.y == end.y {
    if start.x < end.x {
      for i in start.x + 1..=end.x {
        ret.push(Point::new(i, start.y));
      }
    } else {
      for i in (end.x..start.x).rev() {
        ret.push(Point::new(i, start.y));
      }
    }
  } else {
    // panic?
  }

  ret
}

fn populate_grid(
  rocks: &Vec<Point>,
  sand_start: &Point,
  top_left: &Point,
  bottom_right: &Point,
) -> Vec<Vec<bool>> {
  let mut start_y = top_left.y;
  if sand_start.y < top_left.y {
    start_y = sand_start.y;
  }

  let height = (bottom_right.y - start_y) + 1;
  let width = (bottom_right.x - top_left.x) + 1;

  let left = top_left.x;

  let mut grid: Vec<Vec<bool>> = Vec::new();
  for _ in 0..height {
    let mut row: Vec<bool> = Vec::new();
    for _ in 0..width {
      row.push(false);
    }
    grid.push(row);
  }

  for i in 0..rocks.len() {
    let p = &rocks[i];
    grid[p.y][p.x - left] = true;
  }

  grid
}

fn simulate(rocks: &Vec<Point>, sand_start: Point, top_left: &Point, bottom_right: &Point) {
  let left = top_left.x;
  let mut grid = populate_grid(rocks, &sand_start, &top_left, &bottom_right);
  // print_grid(&grid);
  let mut still_on_grid=true;
  let mut c = 0;

  while still_on_grid {
    (grid, still_on_grid) = run_simulation(grid, sand_start.x - left, sand_start.y);
    if still_on_grid {
      c+=1;
    }
  }
  // print_grid(&grid);
  println!("c is {c}")
}

fn run_simulation(mut grid: Vec<Vec<bool>>, x: usize, y: usize) -> (Vec<Vec<bool>>, bool) {
  let mut still_on_grid = true;
  if y >= grid.len()-1 {
    still_on_grid = false;
  } else if !grid[y + 1][x] {  // space directly below
    (grid, still_on_grid) =  run_simulation(grid, x, y + 1);

  } else if x == 0 {
    still_on_grid = false;

  } else if !grid[y + 1][x - 1] {  // space below and left
    (grid, still_on_grid) =  run_simulation(grid, x - 1, y + 1);

  } else if x == grid[0].len() {
    still_on_grid = false;

  } else if !grid[y + 1][x + 1] { // space below and right
    (grid, still_on_grid) =  run_simulation(grid, x + 1, y + 1);

  } else if grid[y][x] { 
    still_on_grid = false;

  } else {  
    grid[y][x]=true;
  }
  (grid, still_on_grid)
}

fn _print_grid(grid: &Vec<Vec<bool>>) {
  for y in 0..grid.len() {
    let row = &grid[y];
    for x in 0..row.len() {
      if grid[y][x] {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}
