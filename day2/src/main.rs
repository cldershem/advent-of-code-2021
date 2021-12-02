use itertools::Itertools;

#[derive(Debug)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug)]
enum MoveInstruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl MoveInstruction {
    fn from_string(line: &str) -> MoveInstruction {
        let (direction, count) = line.split_whitespace().collect_tuple().unwrap();

        let count = count.parse::<i32>().unwrap();
        match direction.to_lowercase().as_str() {
            "forward" => MoveInstruction::Forward(count),
            "down" => MoveInstruction::Down(count),
            "up" => MoveInstruction::Up(count),
            _ => unreachable!(),
        }
    }
}

impl Submarine {
    fn _travel_one(&mut self, instruction: MoveInstruction) {
        match instruction {
            MoveInstruction::Forward(count) => self.horizontal += count,
            MoveInstruction::Down(count) => self.depth += count,
            MoveInstruction::Up(count) => self.depth -= count,
        };
    }

    fn travel_two(&mut self, instruction: MoveInstruction) {
        dbg!(&instruction);
        match instruction {
            MoveInstruction::Forward(count) => {
                self.horizontal += count;
                self.depth += count * self.aim;
            }
            MoveInstruction::Down(count) => self.aim += count,
            MoveInstruction::Up(count) => self.aim -= count,
        };
        dbg!(&self);
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let input: Vec<_> = input
        .lines()
        .map(|line| MoveInstruction::from_string(line))
        .collect();

    let mut submarine = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for instruction in input {
        submarine.travel_two(instruction);
    }

    let result = submarine.horizontal * submarine.depth;
    println!("answer: {}", result);
}
