use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let filename = "test.in";
    // let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let (map_part, instruction_part) = input.split_once("\n\n").unwrap();

    let map: Map = map_part.parse().unwrap();

    // map.cube_warps
    //     .iter()
    //     .for_each(|(a, b)| println!("{:?} -> {:?}", a, b));

    let instructions = Instruction::from_list_string(instruction_part);

    let sim = Simulation::new(map, instructions);

    let answer = sim.simulate();

    println!("Part 1:");
    println!("{}", answer);

    let answer = sim.simulate_cube();

    println!("Part 2:");
    println!("{}", answer);
}

#[derive(Debug)]
struct Map {
    row_limits: Vec<(i64, i64)>,
    column_limits: Vec<(i64, i64)>,
    obstacles: HashSet<(i64, i64)>,
    cube_warps: HashMap<(i64, i64), ((i64, i64), i64)>,
}

fn map(x: i64, a: (i64, i64), b: (i64, i64)) -> i64 {
    (x - a.0) * (b.1 - b.0) / (a.1 - a.0) + b.0
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n');
        let height = lines.clone().count() as i64 + 1;
        let width = lines.clone().map(|line| line.len()).max().unwrap() as i64 + 1;

        let mut row_limits = vec![(width, 0); height as usize];
        let mut column_limits = vec![(height, 0); width as usize];
        let mut obstacles = HashSet::new();

        lines.enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let x = x as i64;
                let y = y as i64;
                if c == '.' || c == '#' {
                    row_limits[y as usize + 1].0 = row_limits[y as usize + 1].0.min(x + 1);
                    row_limits[y as usize + 1].1 = row_limits[y as usize + 1].1.max(x + 1);

                    column_limits[x as usize + 1].0 = column_limits[x as usize + 1].0.min(y + 1);
                    column_limits[x as usize + 1].1 = column_limits[x as usize + 1].1.max(y + 1);
                }

                if c == '#' {
                    obstacles.insert((x + 1, y + 1));
                }
            })
        });

        let mut cube_warps = HashMap::new();

        // NOTE: this condition determines whether to user edge generation for test.in or my.in
        if true {
            // test case
            {
                let y = 1;
                for x in 9..=12 {
                    let target = (4 - (x - 8) + 1, 5);
                    cube_warps.insert((x, y), (target, 2));
                    cube_warps.insert(target, ((x, y), 2));
                }
            }
            {
                let x = 12;
                for y in 1..=4 {
                    let target = (12, 8 + 4 - y + 1);
                    cube_warps.insert((x, y), (target, 2));
                    cube_warps.insert(target, ((x, y), 2));
                }
            }
            {
                let x = 12;
                for y in 5..=8 {
                    let target = (y - 4 + 1 + 12, 9);
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
            {
                let y = 12;
                for x in 13..=16 {
                    let target = (1, x - 12 + 4);
                    cube_warps.insert((x, y), (target, 3));
                    cube_warps.insert(target, ((x, y), 1));
                }
            }
            {
                let y = 12;
                for x in 9..=12 {
                    let target = (4 - (x - 8) + 1, 8);
                    cube_warps.insert((x, y), (target, 2));
                    cube_warps.insert(target, ((x, y), 2));
                }
            }
            {
                let x = 9;
                for y in 9..=12 {
                    let target = (9 - (y - 8), 8);
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }

            {
                let y = 5;
                for x in 5..=8 {
                    let target = (9, x - 4);
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
        } else {
            // my case

            // 1
            {
                let y = 1;
                for x in 51..=100 {
                    let target = (1, map(x, (51, 100), (151, 200)));
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
            // 2
            {
                let y = 1;
                for x in 101..=150 {
                    let target = (map(x, (101, 150), (1, 50)), 200);
                    cube_warps.insert((x, y), (target, 4));
                    cube_warps.insert(target, ((x, y), 4));
                }
            }
            // 3
            {
                let x = 150;
                for y in 1..=50 {
                    let target = (100, map(y, (1, 50), (150, 101)));
                    cube_warps.insert((x, y), (target, 2));
                    cube_warps.insert(target, ((x, y), 2));
                }
            }
            // 4
            {
                let y = 50;
                for x in 101..=150 {
                    let target = (100, map(x, (101, 150), (51, 100)));
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
            // 5
            {
                let y = 150;
                for x in 51..=100 {
                    let target = (50, map(x, (51, 100), (151, 200)));
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
            // 6
            {
                let x = 1;
                for y in 101..=150 {
                    let target = (51, map(y, (101, 150), (50, 1)));
                    cube_warps.insert((x, y), (target, 2));
                    cube_warps.insert(target, ((x, y), 2));
                }
            }
            // 7
            {
                let y = 101;
                for x in 1..=50 {
                    let target = (51, map(x, (1, 50), (51, 100)));
                    cube_warps.insert((x, y), (target, 1));
                    cube_warps.insert(target, ((x, y), 3));
                }
            }
        }

        let map = Map {
            row_limits,
            column_limits,
            obstacles,
            cube_warps,
        };

        Ok(map)
    }
}

impl Map {
    fn is_on_map(&self, point: &(i64, i64)) -> bool {
        if point.1 < 0 || point.1 >= self.row_limits.len() as i64 {
            false
        } else {
            self.row_limits[point.1 as usize].0 <= point.0
                && self.row_limits[point.1 as usize].1 >= point.0
        }
    }
}

#[derive(Debug)]
enum Instruction {
    GoForward(i64),
    TurnRight,
    TurnLeft,
}

impl Instruction {
    fn from_list_string(s: &str) -> Vec<Instruction> {
        let mut list = vec![];
        let mut buf = None;

        for c in s.chars() {
            if c.is_digit(10) {
                buf = Some(buf.unwrap_or(0) * 10 + (c as i64 - '0' as i64))
            } else {
                if buf.is_some() {
                    list.push(Instruction::GoForward(buf.unwrap()));
                    buf = None;
                }

                if c == 'L' {
                    list.push(Instruction::TurnLeft);
                } else if c == 'R' {
                    list.push(Instruction::TurnRight);
                }
            }
        }

        list
    }
}

#[derive(Clone, Copy, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn rotate_left(&self) -> Self {
        use Facing::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn rotate_right(&self) -> Self {
        use Facing::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn as_int(&self) -> i64 {
        use Facing::*;
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

struct Simulation {
    map: Map,
    instructions: Vec<Instruction>,
}

impl Simulation {
    fn new(map: Map, instructions: Vec<Instruction>) -> Self {
        Self { map, instructions }
    }

    fn simulate(&self) -> i64 {
        let mut position = (self.map.row_limits[1].0, 1);
        let mut facing = Facing::Right;

        for instruction in &self.instructions {
            match instruction {
                Instruction::TurnLeft => facing = facing.rotate_left(),
                Instruction::TurnRight => facing = facing.rotate_right(),
                Instruction::GoForward(steps) => {
                    let steps = *steps;
                    for _ in 0..steps {
                        let new_position = match facing {
                            Facing::Right => {
                                if position.0 == self.map.row_limits[position.1 as usize].1 {
                                    (self.map.row_limits[position.1 as usize].0, position.1)
                                } else {
                                    (position.0 + 1, position.1)
                                }
                            }
                            Facing::Left => {
                                if position.0 == self.map.row_limits[position.1 as usize].0 {
                                    (self.map.row_limits[position.1 as usize].1, position.1)
                                } else {
                                    (position.0 - 1, position.1)
                                }
                            }
                            Facing::Down => {
                                if position.1 == self.map.column_limits[position.0 as usize].1 {
                                    (position.0, self.map.column_limits[position.0 as usize].0)
                                } else {
                                    (position.0, position.1 + 1)
                                }
                            }
                            Facing::Up => {
                                if position.1 == self.map.column_limits[position.0 as usize].0 {
                                    (position.0, self.map.column_limits[position.0 as usize].1)
                                } else {
                                    (position.0, position.1 - 1)
                                }
                            }
                        };

                        if self.map.obstacles.contains(&new_position) {
                            break;
                        }

                        position = new_position;
                    }
                }
            }
        }

        // println!("{:?} {:?}", position, facing);

        position.1 * 1000 + position.0 * 4 + facing.as_int()
    }

    fn simulate_cube(&self) -> i64 {
        let mut position = (self.map.row_limits[1].0, 1);
        let mut facing = Facing::Right;

        let mut path = HashMap::new();

        for instruction in &self.instructions {
            match instruction {
                Instruction::TurnLeft => facing = facing.rotate_left(),
                Instruction::TurnRight => facing = facing.rotate_right(),
                Instruction::GoForward(steps) => {
                    let steps = *steps;
                    for _ in 0..steps {
                        // println!("{:?}", position);
                        let (new_position, new_facing) = match facing {
                            Facing::Right => {
                                if position.0 == self.map.row_limits[position.1 as usize].1 {
                                    let (new_position, rot_cnt) = self.map.cube_warps[&position];
                                    let mut new_facing = facing;
                                    for _ in 0..rot_cnt {
                                        new_facing = new_facing.rotate_right();
                                    }
                                    (new_position, new_facing)
                                } else {
                                    ((position.0 + 1, position.1), facing)
                                }
                            }
                            Facing::Left => {
                                if position.0 == self.map.row_limits[position.1 as usize].0 {
                                    let (new_position, rot_cnt) = self.map.cube_warps[&position];
                                    let mut new_facing = facing;
                                    for _ in 0..rot_cnt {
                                        new_facing = new_facing.rotate_right();
                                    }
                                    (new_position, new_facing)
                                } else {
                                    ((position.0 - 1, position.1), facing)
                                }
                            }
                            Facing::Down => {
                                if position.1 == self.map.column_limits[position.0 as usize].1 {
                                    let (new_position, rot_cnt) = self.map.cube_warps[&position];
                                    let mut new_facing = facing;
                                    for _ in 0..rot_cnt {
                                        new_facing = new_facing.rotate_right();
                                    }
                                    (new_position, new_facing)
                                } else {
                                    ((position.0, position.1 + 1), facing)
                                }
                            }
                            Facing::Up => {
                                if position.1 == self.map.column_limits[position.0 as usize].0 {
                                    let (new_position, rot_cnt) = self.map.cube_warps[&position];
                                    let mut new_facing = facing;
                                    for _ in 0..rot_cnt {
                                        new_facing = new_facing.rotate_right();
                                    }
                                    (new_position, new_facing)
                                } else {
                                    ((position.0, position.1 - 1), facing)
                                }
                            }
                        };

                        if self.map.obstacles.contains(&new_position) {
                            break;
                        }

                        path.insert(position, facing);

                        position = new_position;
                        facing = new_facing;
                        // println!("{:?} {:?}", position, facing);
                    }
                }
            };
            path.insert(position, facing);

            assert!(self.map.is_on_map(&position));
        }

        for y in 1..=20 {
            for x in 1..=20 {
                if path.contains_key(&(x, y)) {
                    match path[&(x, y)] {
                        Facing::Up => {
                            print!("^");
                        }
                        Facing::Right => {
                            print!(">");
                        }
                        Facing::Down => {
                            print!("v");
                        }
                        Facing::Left => {
                            print!("<");
                        }
                    }
                } else if self.map.is_on_map(&(x, y)) {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }

        position.1 * 1000 + position.0 * 4 + facing.as_int()
    }
}
