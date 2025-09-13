use std::fs;

pub fn part1() -> i32 {
  let input = fs::read_to_string("src/year2015/day1/input.txt").expect("input file missing");
  let mut floor = 0;
  for char in input.chars() {
    if char == '(' {
      floor += 1;
    } else {
      floor -= 1;
    }
  }
  floor
}

pub fn part2() -> usize {
  let input = fs::read_to_string("src/year2015/day1/input.txt").expect("input file missing");
  let mut floor = 0;
  let mut target = 0;
  for (position, char) in input.chars().enumerate() {
    if char == '(' {
      floor += 1;
    } else {
      floor -= 1;
    }

    if floor == -1 {
      target = position + 1;
      break;
    }
  }

  target
}
