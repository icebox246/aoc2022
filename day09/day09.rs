use std::collections::HashSet;

type Point = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Result<Direction, ()> {
        use Direction::*;
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

struct Rope {
    head: Point,
    tail: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        Rope {
            head: (0, 0),
            tail: vec![(0, 0); n - 1],
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn applyn(&mut self, dir: &Direction, n: usize) {
        for _ in 0..n {
            self.apply(&dir);
            // self.print_state((-5,5), (-5,5));
        }
    }

    fn apply(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }

        for i in 0..(self.tail.len()) {
            let head = if i != 0 { self.tail[i - 1] } else { self.head };
            let tail = &mut self.tail[i];

            while (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                let dx = (head.0 - tail.0).signum();
                let dy = (head.1 - tail.1).signum();
                tail.0 += dx;
                tail.1 += dy;
            }
        }

        self.visited.insert(*self.tail.last().unwrap());
    }

    fn print_state(&self, x_range: (i32, i32), y_range: (i32, i32)) {
        for y in (y_range.0)..=(y_range.1) {
            for x in (x_range.0)..=(x_range.1) {
                if (x, y) == self.head {
                    print!("H");
                } else if let Some(idx) = self.tail.iter().position(|p| *p == (x, y)) {
                    print!("{}", idx + 1);
                } else if x == 0 && y == 0 {
                    print!("o");
                } else if self.visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    // let filename = "test.in";
    // let filename = "test2.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    {
        let mut rope = Rope::new(2);

        for line in input.split('\n').filter(|s| !s.is_empty()) {
            let (direction, count) = line.split_once(' ').unwrap();

            let direction = Direction::from_str(direction).unwrap();
            let count: usize = count.parse().unwrap();

            rope.applyn(&direction, count);
        }

        println!("Part 1:");
        println!("{}", rope.visited.len());
    }

    {
        let mut rope = Rope::new(10);

        for line in input.split('\n').filter(|s| !s.is_empty()) {
            let (direction, count) = line.split_once(' ').unwrap();

            let direction = Direction::from_str(direction).unwrap();
            let count: usize = count.parse().unwrap();

            rope.applyn(&direction, count);
            // rope.print_state((-20, 20), (-20, 20));
        }

        println!("Part 2:");
        println!("{}", rope.visited.len());
    }
}
