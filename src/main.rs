pub mod year2015;

fn main() {
  println!("{}", year2015::day5::part1());
  // println!("{}", year2015::day5::part2());
}

pub fn bubble_sort<T: Ord>(numbers: &mut [T]) {
  let len = numbers.len();
  for i in 0..len {
    let mut swapped = false;

    for j in 0..len - i - 1 {
      if numbers[j] > numbers[j + 1] {
        numbers.swap(j, j + 1);
        swapped = true;
      }
    }

    if !swapped {
      break;
    }
  }
}
