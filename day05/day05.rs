use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

struct Command {
    move_count: u32,
    source: usize,
    destination: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();
        assert!(words.len() == 6);

        let move_count: u32 = words[1].parse()?;
        let source: usize = words[3].parse::<usize>()? - 1;
        let destination: usize = words[5].parse::<usize>()? - 1;

        Ok(Command {
            move_count,
            source,
            destination,
        })
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();

    let mut stack_lines: Vec<&str> = vec![];
    let mut command_lines: Vec<&str> = vec![];

    {
        let mut is_on_commands = false;
        for line in input.split('\n') {
            if line.is_empty() {
                if is_on_commands {
                    break;
                } else {
                    is_on_commands = true;
                    continue;
                }
            }

            if is_on_commands {
                command_lines.push(line);
            } else {
                stack_lines.push(line);
            }
        }
    }

    let stack_count = stack_lines
        .last()
        .unwrap()
        .trim()
        .split("  ")
        .into_iter()
        .count();

    // println!("There are {} stacks!", stack_count);

    let mut orig_stacks: Vec<Vec<char>> = vec![];
    for _ in 1..=stack_count {
        orig_stacks.push(vec![]);
    }

    for line in stack_lines.iter().rev().skip(1) {
        line.char_indices().for_each(|(i, c)| {
            if i % 4 == 1 {
                let stack_index = (i - 1) / 4;
                if c.is_alphabetic() {
                    orig_stacks[stack_index].push(c);
                }
            }
        })
    }

    let commands: Vec<Command> = command_lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    // println!("Start stack state:\n{:?}", orig_stacks);

    {
        let mut stacks = orig_stacks.clone();

        for cmd in &commands {
            for _ in 1..=cmd.move_count {
                let val = stacks[cmd.source].pop().unwrap();
                stacks[cmd.destination].push(val);
            }
        }

        let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

        println!("Part 1:");
        println!("{}", result);
    }

    {
        let mut stacks = orig_stacks.clone();

        for cmd in &commands {
            let mut buff: Vec<char> = vec![];
            for _ in 1..=cmd.move_count {
                buff.push(stacks[cmd.source].pop().unwrap());
            }
            stacks[cmd.destination].extend(buff.iter().rev());
        }

        let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

        println!("Part 2:");
        println!("{}", result);
    }
}
