use std::{collections::HashSet, fs};

pub fn part1() -> usize {
  let input = fs::read_to_string("src/year2015/day3/input.txt").expect("input file missing");
  let mut coords = (0, 0);
  let mut coords_set: HashSet<(i32, i32)> = HashSet::new();
  coords_set.insert(coords);

  for char in input.chars() {
    if char == '<' {
      coords.0 -= 1;
    } else if char == '>' {
      coords.0 += 1;
    }

    if char == '^' {
      coords.1 += 1;
    } else if char == 'v' {
      coords.1 -= 1;
    }
    coords_set.insert(coords);
  }

  coords_set.len()
}

pub fn part2() -> usize {
  let input = fs::read_to_string("src/year2015/day3/input.txt").expect("input file missing");
  let mut santa_coords = (0, 0);
  let mut robo_coords = (0, 0);
  let mut coords_set: HashSet<(i32, i32)> = HashSet::new();
  coords_set.insert((0, 0));

  for (turn, char) in input.chars().enumerate() {
    let coords = match turn % 2 {
      0 => &mut santa_coords,
      1 => &mut robo_coords,
      _ => panic!("no such tuple"),
    };

    if char == '<' {
      coords.0 -= 1;
    } else if char == '>' {
      coords.0 += 1;
    }

    if char == '^' {
      coords.1 += 1;
    } else if char == 'v' {
      coords.1 -= 1;
    }

    // Dereference the borrowed values
    coords_set.insert(*coords);
  }

  coords_set.len()
}
