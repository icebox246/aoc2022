use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Add, Range};

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let elf_positions = input
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut sim = Simulation::new(elf_positions);

    // println!("Starting");
    // sim.print_state(-10..15, -10..15);

    for _i in 1..=10 {
        sim.simulate_round();
        // println!("After round {}", i);
        // sim.print_state(-10..15, -10..15);
    }

    let answer = sim.min_containing_rect_area() - sim.elf_positions.len();

    println!("Part 1:");
    println!("{}", answer);

    while sim.simulate_round() {}

    println!("Part 2:");
    println!("{}", sim.rounds_done);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Simulation {
    elf_positions: HashSet<Point>,
    rounds_done: usize,
}

impl Simulation {
    fn new(elf_positions: HashSet<Point>) -> Self {
        Self {
            elf_positions,
            rounds_done: 0,
        }
    }

    #[allow(dead_code)]
    fn print_state(&self, rx: Range<i64>, ry: Range<i64>) {
        for y in ry.clone() {
            for x in rx.clone() {
                if self.elf_positions.contains(&Point(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn simulate_round(&mut self) -> bool {
        let mut elf_proposal_options = vec![
            ElfProposalVariant::North,
            ElfProposalVariant::South,
            ElfProposalVariant::West,
            ElfProposalVariant::East,
        ];

        elf_proposal_options = elf_proposal_options
            .iter()
            .cycle()
            .skip(self.rounds_done)
            .take(4)
            .cloned()
            .collect();

        let mut elf_proposal_counts = HashMap::new();

        let elf_proposals: Vec<(Point, Point)> = self
            .elf_positions
            .iter()
            .map(|elf| {
                let prop = if self.has_neighbours(elf) {
                    elf_proposal_options
                        .iter()
                        .find_map(|opt| opt.propose(elf, self))
                        .unwrap_or(*elf)
                } else {
                    *elf
                };

                if let Some(prev) = elf_proposal_counts.get(&prop) {
                    elf_proposal_counts.insert(prop, prev + 1);
                } else {
                    elf_proposal_counts.insert(prop, 1);
                }

                (*elf, prop)
            })
            .collect();

        let mut did_move = false;

        for (elf, prop) in elf_proposals {
            if elf_proposal_counts[&prop] == 1 && elf != prop {
                self.elf_positions.remove(&elf);
                self.elf_positions.insert(prop);

                did_move = true;
            }
        }

        self.rounds_done += 1;

        return did_move;
    }

    fn has_neighbours(&self, elf: &Point) -> bool {
        for y in (elf.1 - 1)..=(elf.1 + 1) {
            for x in (elf.0 - 1)..=(elf.0 + 1) {
                if y == elf.1 && x == elf.0 {
                    continue;
                }
                if self.elf_positions.contains(&Point(x, y)) {
                    return true;
                }
            }
        }
        return false;
    }

    fn min_containing_rect_area(&self) -> usize {
        let mut x_min = i64::MAX;
        let mut x_max = i64::MIN;
        let mut y_min = i64::MAX;
        let mut y_max = i64::MIN;

        self.elf_positions.iter().for_each(|elf| {
            x_min = x_min.min(elf.0);
            x_max = x_max.max(elf.0);
            y_min = y_min.min(elf.1);
            y_max = y_max.max(elf.1);
        });

        (x_max - x_min + 1) as usize * (y_max - y_min + 1) as usize
    }
}

#[derive(Clone)]
enum ElfProposalVariant {
    North,
    South,
    West,
    East,
}

impl ElfProposalVariant {
    fn propose(&self, elf: &Point, sim: &Simulation) -> Option<Point> {
        use ElfProposalVariant::*;
        match self {
            North => {
                if sim.elf_positions.contains(&(*elf + Point(0, -1)))
                    || sim.elf_positions.contains(&(*elf + Point(-1, -1)))
                    || sim.elf_positions.contains(&(*elf + Point(1, -1)))
                {
                    None
                } else {
                    Some(*elf + Point(0, -1))
                }
            }
            South => {
                if sim.elf_positions.contains(&(*elf + Point(0, 1)))
                    || sim.elf_positions.contains(&(*elf + Point(-1, 1)))
                    || sim.elf_positions.contains(&(*elf + Point(1, 1)))
                {
                    None
                } else {
                    Some(*elf + Point(0, 1))
                }
            }
            West => {
                if sim.elf_positions.contains(&(*elf + Point(-1, 0)))
                    || sim.elf_positions.contains(&(*elf + Point(-1, -1)))
                    || sim.elf_positions.contains(&(*elf + Point(-1, 1)))
                {
                    None
                } else {
                    Some(*elf + Point(-1, 0))
                }
            }
            East => {
                if sim.elf_positions.contains(&(*elf + Point(1, 0)))
                    || sim.elf_positions.contains(&(*elf + Point(1, -1)))
                    || sim.elf_positions.contains(&(*elf + Point(1, 1)))
                {
                    None
                } else {
                    Some(*elf + Point(1, 0))
                }
            }
        }
    }
}
