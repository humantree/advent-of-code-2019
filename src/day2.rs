use Opcode::*;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<i32> {
  input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<i32>) -> i32 {
  let mut memory = input.clone();
  memory[1] = 12;
  memory[2] = 2;
  Intcode::new(memory).run()[0]
}

#[aoc(day2, part2)]
fn part2(input: &Vec<i32>) -> i32 {
  for i in 0..=99 {
    for j in 0..=99 {
      let mut memory = input.clone();
      memory[1] = i;
      memory[2] = j;

      if Intcode::new(memory).run()[0] == 19690720 {
        return 100 * i + j;
      }
    }
  }

  panic!("No matching noun and verb could be found");
}

enum Opcode {
  Add,
  Multiply,
  Halt,
}

impl Opcode {
  fn from_int(value: i32) -> Self {
    match value {
      1 => Add,
      2 => Multiply,
      99 => Halt,
      _ => panic!("Encountered unknown opcode"),
    }
  }
}

struct Intcode {
  memory: Vec<i32>,
  instruction_pointer: usize,
}

impl Intcode {
  pub fn new(memory: Vec<i32>) -> Self {
    Intcode {
      memory,
      instruction_pointer: 0,
    }
  }

  pub fn run(&mut self) -> Vec<i32> {
    loop {
      let opcode = Opcode::from_int(self.memory[self.instruction_pointer]);

      match opcode {
        Add => {
          let first = self.memory[self.memory[self.instruction_pointer + 1] as usize];
          let second = self.memory[self.memory[self.instruction_pointer + 2] as usize];
          let location = self.memory[self.instruction_pointer + 3] as usize;
          self.memory[location] = first + second;
        }
        Multiply => {
          let first = self.memory[self.memory[self.instruction_pointer + 1] as usize];
          let second = self.memory[self.memory[self.instruction_pointer + 2] as usize];
          let location = self.memory[self.instruction_pointer + 3] as usize;
          self.memory[location] = first * second;
        }
        Halt => break,
      }

      self.instruction_pointer += 4;
    }

    self.memory.to_vec()
  }
}

#[cfg(test)]
mod tests {
  use super::Intcode;

  #[test]
  fn test_1() {
    assert_eq!(
      Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run(),
      vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
  }

  #[test]
  fn test_2() {
    assert_eq!(
      Intcode::new(vec![1, 0, 0, 0, 99]).run(),
      vec![2, 0, 0, 0, 99]
    );
  }

  #[test]
  fn test_3() {
    assert_eq!(
      Intcode::new(vec![2, 3, 0, 3, 99]).run(),
      vec![2, 3, 0, 6, 99]
    );
  }

  #[test]
  fn test_4() {
    assert_eq!(
      Intcode::new(vec![2, 4, 4, 5, 99, 0]).run(),
      vec![2, 4, 4, 5, 99, 9801]
    );
  }

  #[test]
  fn test_5() {
    assert_eq!(
      Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run(),
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }
}
