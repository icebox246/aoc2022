use std::str::FromStr;

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        match s.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => Ok(Noop),
            ["addx", v] => Ok(AddX(v.parse::<i32>().unwrap())),
            _ => Err(()),
        }
    }
}

struct CPU {
    register_x: i32,
    instructions: Vec<Instruction>,
    cycle_count: usize,
}

impl CPU {
    fn from_instructions(insts: &Vec<Instruction>) -> Self {
        let instructions = insts
            .iter()
            .flat_map(|i| match i {
                Instruction::Noop => vec![Instruction::Noop],
                Instruction::AddX(v) => vec![Instruction::Noop, Instruction::AddX(*v)],
            })
            .collect();
        Self {
            register_x: 1,
            instructions,
            cycle_count: 0,
        }
    }
}

impl Iterator for CPU {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle_count >= self.instructions.len() {
            return None;
        }

        let old_x = self.register_x;

        match self.instructions[self.cycle_count] {
            Instruction::Noop => {}
            Instruction::AddX(v) => {
                self.register_x += v;
            }
        }

        self.cycle_count += 1;

        Some((self.cycle_count, old_x))
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let instructions: Vec<Instruction> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();

    {
        let cpu = CPU::from_instructions(&instructions);

        let mut answer = 0;

        for (cycle, x) in cpu {
            if (cycle as i32 - 20) % 40 == 0 {
                answer += cycle as i32 * x;
            }
        }

        println!("Part 1:");
        println!("{}", answer);
    }

    {
        let cpu = CPU::from_instructions(&instructions);

        let mut cursor_x = 0;

        println!("Part 2:");
        for (_, x) in cpu {
            if (x - cursor_x).abs() <= 1 {
                print!("#");
            } else {
                print!(" ");
            }

            cursor_x += 1;
            if cursor_x == 40 {
                cursor_x = 0;
                print!("\n");
            }
        }
    }
}
