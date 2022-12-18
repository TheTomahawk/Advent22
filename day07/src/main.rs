use std::{
  env,
  fs::File,
  io::{BufRead, BufReader, Lines},
};

#[derive(PartialEq)]
enum InodeType {
  Dir,
  File,
}

struct Inode {
  name: String,
  itype: InodeType, // `type` is a reserved word
  size: u64,
  children: Vec<Inode>,
}

impl Inode {
  fn add_file(&mut self, filename: &str, size: u64) {
    let node = Inode {
      name: String::from(filename),
      itype: InodeType::File,
      size: size,
      children: Vec::new(),
    };
    self.children.push(node);
    self.size += size;
  }
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut fs: Inode = Inode {
    name: String::from("/"),
    itype: InodeType::Dir,
    size: 0, // if a file, the size of the file.  If a dir, the size of all files and subdirs
    children: Vec::new(),
  };

  let mut sum = 0;

  (lines, fs, sum) = parse(lines, fs, 100000, sum);

  // print_fs(&fs);

  println!("Part one: answer = {}", sum);

  let total_space = 70000000;
  let required_free = 30000000;
  let used = fs.size;
  let free = total_space - used;
  let required = required_free - free;

  println!("We need to free up {}", required);

  let mut answer = total_space;
  answer = find_the_dir(&fs, required, answer);

  println!("Part two: answer = {answer}");
}

fn find_the_dir(dir: &Inode, min_size: u64, mut best_fit: u64) -> u64 {
  let mut iter = dir.children.iter();
  while let Some(child) = iter.next() {
    if child.itype == InodeType::Dir {  // we can ignore files for this bit
      best_fit = find_the_dir(child, min_size, best_fit);
      if child.size < best_fit && child.size > min_size {
        best_fit=child.size
      }
    }
  }

  best_fit
}

fn parse(
  mut lines: Lines<BufReader<File>>,
  mut dir: Inode,
  limit: u64,
  mut sum: u64
) -> (Lines<BufReader<File>>, Inode, u64) {
  // println!("parse {}", dir.name);
  while let Some(input) = lines.next() {
    if let Ok(mut line) = input {
      // println!("  [{}]", line);
      if line.starts_with("$ ") {
        let mut command = line.split_off(2);
        if command.starts_with("cd ") {
          let dirname = command.split_off(3);
          if dirname == ".." {
            break;
          } else if dirname == "/" {
            // ignore -- we assume we are starting with `cd /`
            continue;
          } else {
            let mut new_dir = Inode {
              name: String::from(dirname),
              itype: InodeType::Dir,
              size: 0,
              children: Vec::new(),
            };
            (lines, new_dir, sum) = parse(lines, new_dir, limit, sum);
            if new_dir.size < limit {
              sum += new_dir.size;
            }
            dir.size += new_dir.size;
            dir.children.push(new_dir);
          }
        } else if command.starts_with("ls") {
          continue;
        }
      } else if line.starts_with("dir") {
        // ignore -- we add the directory on a `cd`
        continue;
      } else {
        // it starts with a number.
        let strings: Vec<_> = line.split(" ").collect();
        let str_size = strings[0];
        let filename = strings[1];
        let size: u64 = str_size.parse().unwrap();
        // println!("Adding file {} to dir {}", filename, dir.name);
        dir.add_file(filename, size);
      }
    }
  }
  // println!("cd ..");
  (lines, dir, sum)
}

fn print_fs(fs: &Inode) {
  println!("/ ({})", fs.size);
  print_dir(&fs, 2);
}

fn print_dir(dir: &Inode, indent: usize) {
  let mut iter = dir.children.iter();
  while let Some(child) = iter.next() {
    if child.itype == InodeType::Dir {
      println!(
        "{s:>width$} - {val} (dir: {size})",
        s = " ",
        width = indent,
        val = child.name,
        size = child.size
      );
      print_dir(&child, indent + 1);
    } else {
      println!(
        "{s:>width$} - {val} (file: {size})",
        s = " ",
        width = indent,
        size = child.size,
        val = child.name
      );
    }
  }
}
