use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let filename = "test.in";
    // let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();
    let gas_streams: Vec<GasDirection> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| GasDirection::from_char(c))
        .collect();

    let rock_shapes: Vec<Rock> = [
        "####",
        ".#.\n\
         ###\n\
         .#.",
        "..#\n\
         ..#\n\
         ###",
        "#\n\
         #\n\
         #\n\
         #",
        "##\n\
         ##",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();

    let mut sim = Simulation::new(gas_streams, rock_shapes, 7);

    sim.simulate(100000);
    sim.clear();

    sim.simulate(2022);

    println!("Part 1:");
    println!("{}", sim.height);

    sim.clear();

    let cycle_start = sim.cycle_start.unwrap();
    let cycle_length = sim.cycle_length.unwrap();

    // let cycle_start = cycle_start + 3; // I really don't know why

    let mut simulations_to_do = 1000000000000;

    simulations_to_do -= cycle_start;
    sim.simulate(cycle_start);

    let runup_height = sim.height;

    let cycle_count = simulations_to_do / cycle_length;
    simulations_to_do %= cycle_length;
    sim.simulate(cycle_length);

    let cycle_height = sim.height - runup_height;

    if simulations_to_do > 0 {
        sim.simulate(simulations_to_do);
    }

    let rest_height = sim.height - cycle_height - runup_height;

    let total_height = runup_height + cycle_height * cycle_count + rest_height;

    println!("Part 2:");
    println!("{}", total_height);
}

#[derive(Debug, Clone)]
enum GasDirection {
    Left,
    Right,
}

impl GasDirection {
    fn from_char(c: char) -> Self {
        match c {
            '>' => GasDirection::Right,
            '<' => GasDirection::Left,
            c => panic!("Unexpected char: '{}'", c),
        }
    }

    fn as_int(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    tiles: Vec<(usize, usize)>,
}

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s
                .split('\n')
                .rev()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
                })
                .collect(),
        })
    }
}

impl Rock {
    fn moved(&self, direction: (i32, i32)) -> Self {
        Self {
            tiles: self
                .tiles
                .iter()
                .map(|&(x, y)| {
                    (
                        (x as i32 + direction.0) as usize,
                        (y as i32 + direction.1) as usize,
                    )
                })
                .collect(),
        }
    }
}

struct Simulation {
    tiles: Vec<Vec<usize>>,
    rock_count: usize,
    gas_streams: Vec<GasDirection>,
    cycle_start: Option<usize>,
    cycle_length: Option<usize>,

    gas_iter_offset: usize,
    rock_iter_offset: usize,
    rocks: Vec<Rock>,
    width: usize,
    height: usize,
}

impl Simulation {
    const HEADROOM: usize = 10;
    fn new(gas_streams: Vec<GasDirection>, rocks: Vec<Rock>, width: usize) -> Self {
        let mut new = Self {
            tiles: vec![],
            rock_count: 0,
            gas_streams,
            rocks,
            width,
            height: 0,
            cycle_start: None,
            cycle_length: None,
            gas_iter_offset: 0,
            rock_iter_offset: 0,
        };

        let mut floor = vec![];
        floor.resize_with(width + 2, || 1);
        new.tiles.push(floor);

        new.ensure_headroom();

        new
    }

    fn ensure_headroom(&mut self) {
        let mut new_row = vec![];
        new_row.push(1);
        new_row.resize_with(self.width + 1, || 0);
        new_row.push(1);

        self.tiles
            .resize_with(self.height + Self::HEADROOM, || new_row.clone());
    }

    fn clear(&mut self) {
        self.gas_iter_offset = 0;
        self.rock_iter_offset = 0;
        self.rock_count = 0;
        self.height = 0;
        self.tiles = vec![];

        let mut floor = vec![];
        floor.resize_with(self.width + 2, || 1);
        self.tiles.push(floor);

        self.ensure_headroom();
    }

    fn simulate(&mut self, iterations: usize) {
        let rocks_vec = self.rocks.clone();
        let mut rocks = rocks_vec.iter().cycle().skip(self.rock_iter_offset);
        let gas_streams_vec = self.gas_streams.clone();
        let mut gas_streams = gas_streams_vec.iter().cycle().skip(self.gas_iter_offset);

        let mut gas_streams_used = 0;
        let mut rocks_used = 0;

        let mut gas_vs_rock_cycle_iteration = HashMap::new();
        let mut gas_vs_rock_cycle_last_iteration = HashMap::new();

        for i in 1..=iterations {
            let mut rock = rocks.next().unwrap().moved((3, self.height as i32 + 4));

            // self.print_state_with_rock(&rock);

            loop {
                let stream = gas_streams.next().unwrap();
                gas_streams_used += 1;
                let new_rock = rock.moved((stream.as_int(), 0));

                if !self.rock_collides(&new_rock) {
                    rock = new_rock;
                }

                let new_rock = rock.moved((0, -1));

                if !self.rock_collides(&new_rock) {
                    rock = new_rock;
                } else {
                    self.place_rock(&rock);
                    self.height = self
                        .height
                        .max(rock.tiles.iter().map(|p| p.1).max().unwrap());
                    self.ensure_headroom();
                    break;
                }
                // self.print_state_with_rock(&rock);
            }

            rocks_used += 1;

            if rocks_used % rocks_vec.len() == 0 {
                // println!(
                //     "Rock cycle on iteration: {} on iteration {}",
                //     gas_streams_used % gas_streams_vec.len(),
                //     i
                // );

                if let Some((prev, prev_height)) = gas_vs_rock_cycle_iteration
                    .insert(gas_streams_used % gas_streams_vec.len(), (i, self.height))
                {
                    if let Some((prev_prev, prev_prev_height)) = gas_vs_rock_cycle_last_iteration
                        .insert(
                            gas_streams_used % gas_streams_vec.len(),
                            (prev, prev_height),
                        )
                    {
                        if prev_height - prev_prev_height == self.height - prev_height {
                            if self.cycle_start.is_none() {
                                self.cycle_start = Some(prev_prev);
                                self.cycle_length = Some(prev - prev_prev);
                            } else {
                                assert_eq!(prev - prev_prev, self.cycle_length.unwrap());
                            }
                        }
                    }
                }
            }

            // self.print_state();
        }

        self.gas_iter_offset = gas_streams_used % gas_streams_vec.len();
        self.rock_iter_offset = rocks_used % rocks_vec.len();
    }

    fn rock_collides(&self, rock: &Rock) -> bool {
        rock.tiles
            .iter()
            .find(|(x, y)| self.tiles[*y][*x] != 0)
            .is_some()
    }

    fn place_rock(&mut self, rock: &Rock) {
        self.rock_count += 1;
        rock.tiles
            .iter()
            .for_each(|(x, y)| self.tiles[*y][*x] = self.rock_count);
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        self.tiles.iter().rev().for_each(|row| {
            row.iter().for_each(|&is_rock| {
                if is_rock != 0 {
                    match is_rock % 5 {
                        1 => print!("1"),
                        2 => print!("2"),
                        3 => print!("3"),
                        4 => print!("4"),
                        0 => print!("5"),
                        _ => unreachable!(),
                    }
                } else {
                    print!(" ");
                }
            });
            println!("");
        });
        println!("");
    }

    #[allow(dead_code)]
    fn print_state_with_rock(&self, rock: &Rock) {
        self.tiles.iter().enumerate().rev().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, &is_rock)| {
                if is_rock != 0 {
                    print!("#");
                } else if rock
                    .tiles
                    .iter()
                    .find(|(rx, ry)| *rx == x && *ry == y)
                    .is_some()
                {
                    print!("@");
                } else {
                    print!(" ");
                }
            });
            println!("");
        });
        println!("");
    }
}
