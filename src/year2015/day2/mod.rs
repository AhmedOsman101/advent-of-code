use crate::bubble_sort;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> Result<i32, String> {
  // Open the file
  let file = File::open("src/year2015/day2/input.txt").expect("input file missing");
  // Wrap it in a buffered reader
  let reader = BufReader::new(file);
  let mut total = 0;

  for line in reader.lines() {
    let line = line.expect("Failed to read line");
    let mut parts = line.split('x');

    let l: i32 = parts
      .next()
      .expect("Dimesnsion l is missing")
      .parse()
      .expect("Not a number");

    let w: i32 = parts
      .next()
      .expect("Dimesnsion w is missing")
      .parse()
      .expect("Not a number");

    let h: i32 = parts
      .next()
      .expect("Dimesnsion h is missing")
      .parse()
      .expect("Not a number");

    let mut dimensions = [l, w, h];
    bubble_sort(&mut dimensions);

    total += (2 * l * w) + (2 * l * h) + (2 * w * h) + (dimensions[0] * dimensions[1]);
  }

  Ok(total)
}

pub fn part2() -> Result<i32, String> {
  // let input = fs::read_to_string("src/year2015/day2/input.txt").expect("input file missing");
  // Open the file
  let file = File::open("src/year2015/day2/input.txt").expect("input file missing");
  // Wrap it in a buffered reader
  let reader = BufReader::new(file);
  let mut total = 0;

  for line in reader.lines() {
    let line = line.expect("Failed to read line");
    let mut parts = line.split('x');

    let l: i32 = parts
      .next()
      .expect("Dimesnsion l is missing")
      .parse()
      .expect("Not a number");

    let w: i32 = parts
      .next()
      .expect("Dimesnsion w is missing")
      .parse()
      .expect("Not a number");

    let h: i32 = parts
      .next()
      .expect("Dimesnsion h is missing")
      .parse()
      .expect("Not a number");

    let mut dimensions = [l, w, h];
    bubble_sort(&mut dimensions);
    let (min_1, min_2) = (dimensions[0], dimensions[1]);

    total += (min_1 * 2) + (min_2 * 2) + (l * w * h);
  }

  Ok(total)
}
