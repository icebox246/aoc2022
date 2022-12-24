use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn main() {
    let filename = "test.in";
    // let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let map: Map = input.parse().unwrap();

    let start_end = map.find_shortest_path((1, 0), (map.width - 2, map.height - 1), 0);

    println!("Part 1:");
    println!("{}", start_end);

    let start_end_start =
        map.find_shortest_path((map.width - 2, map.height - 1), (1, 0), start_end);

    let start_end_start_end =
        map.find_shortest_path((1, 0), (map.width - 2, map.height - 1), start_end_start);

    println!("Part 1:");
    println!("{}", start_end_start_end);
}

#[derive(Debug)]
struct Blizzard {
    direction: i32, // with axis, against axis
    offset: i32,
}

#[derive(Debug)]
struct Map {
    horizontal_blizzards: Vec<Vec<Blizzard>>,
    vertical_blizzards: Vec<Vec<Blizzard>>,
    width: i32,
    height: i32,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: i32 = -1;
        let mut height: i32 = -1;

        let mut vertical_blizzards = vec![];
        let mut horizontal_blizzards = vec![];

        for (y, line) in s.split('\n').filter(|s| !s.is_empty()).enumerate() {
            horizontal_blizzards.push(vec![]);
            for (x, c) in line.chars().enumerate() {
                if vertical_blizzards.get(x).is_none() {
                    vertical_blizzards.push(vec![]);
                }

                match c {
                    '>' => horizontal_blizzards[y].push(Blizzard {
                        direction: 1,
                        offset: x as i32,
                    }),
                    '<' => horizontal_blizzards[y].push(Blizzard {
                        direction: -1,
                        offset: x as i32,
                    }),
                    'v' => vertical_blizzards[x].push(Blizzard {
                        direction: 1,
                        offset: y as i32,
                    }),
                    '^' => vertical_blizzards[x].push(Blizzard {
                        direction: -1,
                        offset: y as i32,
                    }),
                    '#' | '.' => {}
                    _ => {
                        return Err(format!("Unknown character: `{}`", c));
                    }
                }

                width = width.max(x as i32 + 1);
            }
            height = y as i32 + 1;
        }

        Ok(Self {
            horizontal_blizzards,
            vertical_blizzards,
            width,
            height,
        })
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    if x < y {
        gcd(y, x)
    } else if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

impl Map {
    fn find_shortest_path(&self, from: (i32, i32), to: (i32, i32), starting_time: usize) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let time_cycle =
            (self.width - 2) * (self.height - 2) / gcd(self.width - 2, self.height - 2);

        queue.push_back((from.0, from.1, starting_time as i32));
        visited.insert((from.0, from.1, starting_time as i32 % time_cycle));

        while !queue.is_empty() {
            let (x, y, t) = queue.pop_front().unwrap();

            if x == to.0 && y == to.1 {
                return t as usize;
            }

            if !self.is_occupied(x, y, t + 1) && !visited.contains(&(x, y, (t + 1) % time_cycle)) {
                queue.push_back((x, y, t + 1));
                visited.insert((x, y, (t + 1) % time_cycle));
            }
            if !self.is_occupied(x + 1, y, t + 1)
                && !visited.contains(&(x + 1, y, (t + 1) % time_cycle))
            {
                queue.push_back((x + 1, y, t + 1));
                visited.insert((x + 1, y, (t + 1) % time_cycle));
            }
            if !self.is_occupied(x, y + 1, t + 1)
                && !visited.contains(&(x, y + 1, (t + 1) % time_cycle))
            {
                queue.push_back((x, y + 1, t + 1));
                visited.insert((x, y + 1, (t + 1) % time_cycle));
            }
            if !self.is_occupied(x - 1, y, t + 1)
                && !visited.contains(&(x - 1, y, (t + 1) % time_cycle))
            {
                queue.push_back((x - 1, y, t + 1));
                visited.insert((x - 1, y, (t + 1) % time_cycle));
            }
            if !self.is_occupied(x, y - 1, t + 1)
                && !visited.contains(&(x, y - 1, (t + 1) % time_cycle))
            {
                queue.push_back((x, y - 1, t + 1));
                visited.insert((x, y - 1, (t + 1) % time_cycle));
            }
        }

        unreachable!()
    }

    fn is_occupied(&self, x: i32, y: i32, t: i32) -> bool {
        !(x == 1 && y == 0)
            && !(x == self.width - 2 && y == self.height - 1)
            && (x <= 0
                || y <= 0
                || x >= self.width - 1
                || y >= self.height - 1
                || self.horizontal_blizzards[y as usize].iter().any(|b| {
                    ((b.direction * t + b.offset - 1) % (self.width - 2) + (self.width - 2))
                        % (self.width - 2)
                        + 1
                        == x
                })
                || self.vertical_blizzards[x as usize].iter().any(|b| {
                    ((b.direction * t + b.offset - 1) % (self.height - 2) + (self.height - 2))
                        % (self.height - 2)
                        + 1
                        == y
                }))
    }
}
