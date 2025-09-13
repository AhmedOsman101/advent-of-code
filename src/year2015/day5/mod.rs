use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> u32 {
  // Open the file
  let file = File::open("src/year2015/day5/input.txt").expect("input file missing");
  // Wrap it in a buffered reader
  let reader = BufReader::new(file);

  let vowels_regex = Regex::new(r"[aeiou]").unwrap();
  let twice_regex =
    Regex::new(r"(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)")
      .unwrap();
  let unwanted_regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();

  let mut nice_words: u32 = 0;

  for line in reader.lines() {
    let line = line.unwrap();
    let line = line.trim();
    let is_nice = vowels_regex.find_iter(line).count() >= 3
      && twice_regex.is_match(line)
      && !unwanted_regex.is_match(line);

    if is_nice {
      nice_words += 1;
    }
  }
  nice_words
}

pub fn part2() -> i32 {
  // Open the file
  let file = File::open("src/year2015/day5/input.txt").expect("input file missing");
  // Wrap it in a buffered reader
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.unwrap();
    let line = line.trim();

    println!("{line}");
  }
  0
}
