use std::iter::successors;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<u32> {
  input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<u32>) -> u32 {
  input.iter().map(|mass| calculate_fuel(mass).unwrap()).sum()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<u32>) -> u32 {
  input.iter().map(calculate_module_fuel).sum()
}

fn calculate_fuel(mass: &u32) -> Option<u32> {
  (mass / 3).checked_sub(2)
}

fn calculate_module_fuel(mass: &u32) -> u32 {
  successors(Some(*mass), calculate_fuel).skip(1).sum()
}

#[cfg(test)]
mod tests {
  use super::{calculate_fuel, calculate_module_fuel};

  #[test]
  fn test_1() {
    assert_eq!(calculate_fuel(&12), Some(2));
  }

  #[test]
  fn test_2() {
    assert_eq!(calculate_fuel(&14), Some(2));
  }

  #[test]
  fn test_3() {
    assert_eq!(calculate_fuel(&1969), Some(654));
  }

  #[test]
  fn test_4() {
    assert_eq!(calculate_fuel(&100756), Some(33583));
  }

  #[test]
  fn test_5() {
    assert_eq!(calculate_module_fuel(&14), 2);
  }

  #[test]
  fn test_6() {
    assert_eq!(calculate_module_fuel(&1969), 966);
  }

  #[test]
  fn test_7() {
    assert_eq!(calculate_module_fuel(&100756), 50346);
  }
}
