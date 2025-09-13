use md5;
use std::fs;

pub fn part1() -> u64 {
  let input = fs::read_to_string("src/year2015/day4/input.txt").expect("input file missing");
  let input = input.trim();
  let mut i: u64 = 0;
  let pattern = "00000";

  loop {
    let candidate = format!("{input}{i}");
    let digest = md5::compute(candidate.as_bytes());
    let hex = format!("{:x}", digest);

    if hex.starts_with(pattern) {
      break;
    }
    i += 1;
  }
  i
}

pub fn part2() -> u64 {
  let input = fs::read_to_string("src/year2015/day4/input.txt").expect("input file missing");
  let input = input.trim();
  let mut i: u64 = 0;
  let pattern = "000000";

  loop {
    let candidate = format!("{input}{i}");
    let digest = md5::compute(candidate.as_bytes());
    let hex = format!("{:x}", digest);

    if hex.starts_with(pattern) {
      break;
    }
    i += 1;
  }
  i
}
