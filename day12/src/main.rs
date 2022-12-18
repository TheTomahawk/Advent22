use queues::*;
use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

type CellValueType = i16;

struct Cell {
  value: CellValueType,
  up: bool,
  down: bool,
  left: bool,
  right: bool,
  dist: i32, // from S
}

impl Cell {
  fn new(value: CellValueType) -> Cell {
    Cell {
      value: value,
      up: false,
      down: false,
      left: false,
      right: false,
      dist: i32::MAX,
    }
  }
  fn up(&mut self, b: bool) {
    self.up = b;
  }
  fn down(&mut self, b: bool) {
    self.down = b;
  }
  fn right(&mut self, b: bool) {
    self.right = b;
  }
  fn left(&mut self, b: bool) {
    self.left = b;
  }
  fn dist(&mut self, d: i32) {
    self.dist = d;
  }
}

type Grid = Vec<Vec<Cell>>;
type Coordinate = (usize, usize);

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let (mut grid, start, end) = load_grid(filename);

  grid = calculate_distances(grid, start);
  // print_grid_distances(&grid);

  println!("Part one: answer = {}", grid[end.1][end.0].dist);

  let mut shortest = grid[end.1][end.0].dist;

  let input_queue = find_all_zero_cells(&grid);
  let mut iter = input_queue.iter();
  while let Some(mut coords) = iter.next() {
    grid = reset_distances(grid);
    grid = calculate_distances(grid, *coords);
    let e = grid[end.1][end.0].dist;
    if e < shortest {
      shortest = e
    }
  }

  println!("part two: answer = {shortest}");
}

fn load_grid(filename: &String) -> (Grid, Coordinate, Coordinate) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut grid: Grid = Vec::new();

  let mut start: Coordinate = (0, 0);
  let mut end: Coordinate = (0, 0);

  let mut x = 0;
  let mut y = 0;

  while let Some(input) = lines.next() {
    if let Ok(input) = input {
      let mut line: Vec<Cell> = Vec::new();
      let mut chars = input.chars();
      while let Some(c) = chars.next() {
        if c == 'S' {
          line.push(Cell::new(0));
          start = (x, y);
        } else if c == 'E' {
          line.push(Cell::new(26));
          end = (x, y);
        } else {
          let val =
            c.to_ascii_lowercase() as CellValueType - 'a'.to_ascii_lowercase() as CellValueType;
          line.push(Cell::new(val));
        }
        x += 1;
      }
      grid.push(line);
      y += 1;
      x = 0;
    }
  }
  // print_grid(&grid);
  // println!("Start at {:?}, end at {:?}", start, end);

  grid = set_valid_paths(grid);
  // print_grid_available_paths(&grid);

  (grid, start, end)
}

fn set_valid_paths(mut grid: Grid) -> Grid {
  const OVER_26: CellValueType = 100;
  let width = grid[0].len();
  let height = grid.len();

  for y in 0..height {
    for x in 0..width {
      let test_val = grid[y][x].value + 1;
      let mut up = OVER_26;
      let mut down = OVER_26;
      let mut left = OVER_26;
      let mut right = OVER_26;
      if x == 0 {
        grid[y][x].left(false);
      } else {
        left = grid[y][x - 1].value;
      }
      if y == 0 {
        grid[y][x].up(false);
      } else {
        up = grid[y - 1][x].value
      }
      if x == width - 1 {
        grid[y][x].right(false);
      } else {
        right = grid[y][x + 1].value
      }
      if y == height - 1 {
        grid[y][x].down(false);
      } else {
        down = grid[y + 1][x].value
      }
      if up <= test_val {
        grid[y][x].up(true);
      }
      if down <= test_val {
        grid[y][x].down(true);
      }
      if left <= test_val {
        grid[y][x].left(true);
      }
      if right <= test_val {
        grid[y][x].right(true);
      }
    }
  }
  grid
}

fn calculate_distances(mut grid: Grid, start: Coordinate) -> Grid {
  let mut queue: Queue<Coordinate> = Queue::new();
  queue.add(start).expect("oopsy");
  let mut max = 0;
  grid[start.1][start.0].dist(0);

  while queue.size() > 0 {
    if let Ok((x, y)) = queue.remove() {
      // println!("  {:?}", (x,y));
      let d = grid[y][x].dist + 1;
      if grid[y][x].up {
        let (a, b) = (x, y - 1);
        let n = grid[b][a].dist;
        if d < n {
          queue.add((a, b)).expect("oopsy");
          grid[b][a].dist(d);
        }
      };
      if grid[y][x].down {
        let (a, b) = (x, y + 1);
        let n = grid[b][a].dist;
        if d < n {
          queue.add((a, b)).expect("oopsy");
          grid[b][a].dist(d);
        }
      };
      if grid[y][x].left {
        let (a, b) = (x - 1, y);
        let n = grid[b][a].dist;
        if d < n {
          queue.add((a, b)).expect("oopsy");
          grid[b][a].dist(d);
        }
      };
      if grid[y][x].right {
        let (a, b) = (x + 1, y);
        let n = grid[b][a].dist;
        if d < n {
          queue.add((a, b)).expect("oopsy");
          grid[b][a].dist(d);
        }
      };
      if d > max {
        max = d;
      }
    }
  }

  grid
}

fn find_all_zero_cells(grid: &Grid) -> Vec<Coordinate> {
  let mut ret: Vec<Coordinate> = Vec::new();

  let width = grid[0].len();
  let height = grid.len();

  for y in 0..height {
    for x in 0..width {
      if grid[y][x].value == 0 {
        ret.push((x, y));
      }
    }
  }
  ret
}

fn reset_distances(mut grid: Grid) -> Grid {
  let width = grid[0].len();
  let height = grid.len();

  for y in 0..height {
    for x in 0..width {
      grid[y][x].dist(i32::MAX);
    }
  }
  grid
}

// --- print some stuff

fn print_grid(grid: &Grid) {
  let mut iter = grid.iter();
  while let Some(row) = iter.next() {
    let mut iter = row.iter();
    while let Some(cell) = iter.next() {
      print!("{:>3}", cell.value);
    }
    println!("");
  }
}

fn print_grid_available_paths(grid: &Grid) {
  let mut iter = grid.iter();
  while let Some(row) = iter.next() {
    let mut iter = row.iter();
    while let Some(cell) = iter.next() {
      let up = {
        if cell.up {
          "^"
        } else {
          "."
        }
      };
      let down = {
        if cell.down {
          "v"
        } else {
          "."
        }
      };
      let left = {
        if cell.left {
          "<"
        } else {
          "."
        }
      };
      let right = {
        if cell.right {
          ">"
        } else {
          "."
        }
      };
      print!("{}{}{}{} ", left, up, down, right);
    }
    println!("");
  }
}

fn print_grid_distances(grid: &Grid) {
  let mut iter = grid.iter();
  while let Some(row) = iter.next() {
    let mut iter = row.iter();
    while let Some(cell) = iter.next() {
      print!(
        "{:>4},",
        if cell.dist == 2147483647 {
          0
        } else {
          cell.dist
        }
      );
    }
    println!("");
  }
}
