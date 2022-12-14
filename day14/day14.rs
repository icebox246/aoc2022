use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let filename = "test.in";
    // let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let rock_lines = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.parse::<RockLine>().unwrap())
        .collect();

    {
        let mut sim = SimulationField::new(&rock_lines, (500, 0), SimulationFloorRule::HasNoFloor);
        let mut sand_count = 0;

        loop {
            match sim.drop_sand() {
                SimulationResult::SandRests => {
                    sand_count += 1;
                }
                SimulationResult::SandFallsThrough => break,
                SimulationResult::SandBlocked => unreachable!(),
            }
        }
        sim.print_state();

        println!("Part 1:");
        println!("{}", sand_count);
    }

    {
        let mut sim = SimulationField::new(&rock_lines, (500, 0), SimulationFloorRule::HasFloor);
        let mut sand_count = 0;

        loop {
            match sim.drop_sand() {
                SimulationResult::SandRests => {
                    sand_count += 1;
                }
                SimulationResult::SandBlocked => break,
                SimulationResult::SandFallsThrough => unreachable!(),
            }
        }
        sim.print_state();

        println!("Part 2:");
        println!("{}", sand_count);
    }
}

#[derive(Debug)]
struct RockLine {
    points: Vec<(usize, usize)>,
}

impl FromStr for RockLine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            points: s
                .split(" -> ")
                .map(|ss| ss.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect(),
        })
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Rock,
    Air,
    Sand,
}

struct SimulationField {
    tiles: Vec<Vec<Tile>>,
    origin: (usize, usize),
    rect: ((usize, usize), (usize, usize)),
    floor_rule: SimulationFloorRule,
}

enum SimulationResult {
    SandRests,
    SandFallsThrough,
    SandBlocked,
}

#[derive(PartialEq)]
enum SimulationFloorRule {
    HasNoFloor,
    HasFloor,
}

impl SimulationField {
    fn new(
        rock_lines: &Vec<RockLine>,
        origin: (usize, usize),
        floor_rule: SimulationFloorRule,
    ) -> Self {
        let mut rect = (origin.clone(), origin.clone());

        rock_lines.iter().for_each(|rock_line| {
            rock_line.points.iter().for_each(|&(x, y)| {
                rect.0 .0 = rect.0 .0.min(x - 1);
                rect.1 .0 = rect.1 .0.max(x + 1);
                rect.0 .1 = rect.0 .1.min(y - 1);
                rect.1 .1 = rect.1 .1.max(y + 1);
            })
        });

        if floor_rule == SimulationFloorRule::HasFloor {
            rect.0 .0 = origin.0 - rect.1 .1;
            rect.1 .0 = origin.0 + rect.1 .1;
        }

        let tiles = vec![vec![Tile::Air; rect.1 .0 - rect.0 .0 + 1]; rect.1 .1 - rect.0 .1 + 1];

        let mut sim = Self {
            tiles,
            origin,
            rect,
            floor_rule,
        };

        rock_lines.iter().for_each(|rock_line| {
            rock_line
                .points
                .iter()
                .zip(rock_line.points.iter().skip(1))
                .for_each(|(&(cx, cy), &(px, py))| {
                    if cx == px {
                        for y in (cy.min(py))..=(cy.max(py)) {
                            sim.set_tile(&(cx, y), &Tile::Rock);
                        }
                    } else {
                        for x in (cx.min(px))..=(cx.max(px)) {
                            sim.set_tile(&(x, cy), &Tile::Rock);
                        }
                    }
                });
        });

        sim
    }

    fn set_tile(&mut self, pos: &(usize, usize), tile: &Tile) {
        self.tiles[pos.1 - self.rect.0 .1][pos.0 - self.rect.0 .0] = tile.clone();
    }

    fn get_tile(&self, pos: &(usize, usize)) -> Tile {
        self.tiles[pos.1 - self.rect.0 .1][pos.0 - self.rect.0 .0].clone()
    }

    fn print_state(&self) {
        for y in (self.rect.0 .1)..=(self.rect.1 .1) {
            for x in (self.rect.0 .0)..=(self.rect.1 .0) {
                match self.get_tile(&(x, y)) {
                    Tile::Air => {
                        if (x, y) == self.origin {
                            print!("+");
                        } else {
                            print!(".")
                        }
                    }
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                }
            }
            println!("");
        }
    }

    fn drop_sand(&mut self) -> SimulationResult {
        let mut pos = self.origin.clone();

        if self.get_tile(&pos) != Tile::Air {
            return SimulationResult::SandBlocked;
        }

        while pos.1 < self.rect.1 .1 {
            pos.1 += 1;

            if self.get_tile(&pos) != Tile::Air {
                pos.0 -= 1;
                if self.get_tile(&pos) != Tile::Air {
                    pos.0 += 2;
                    if self.get_tile(&pos) != Tile::Air {
                        pos.1 -= 1;
                        pos.0 -= 1;
                        break;
                    }
                }
            }
        }
        if pos.1 == self.rect.1 .1 && self.floor_rule == SimulationFloorRule::HasNoFloor {
            return SimulationResult::SandFallsThrough;
        } else {
            self.set_tile(&pos, &Tile::Sand);
            return SimulationResult::SandRests;
        }
    }
}
