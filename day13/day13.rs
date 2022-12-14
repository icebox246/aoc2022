use std::cmp::Ord;
use std::cmp::Ordering;
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let mut lines = input.split('\n').peekable();

    let mut pairs = vec![];

    while let Some(_) = lines.peek() {
        let pack1: Packet = lines.next().unwrap().parse().unwrap();
        let pack2: Packet = lines.next().unwrap().parse().unwrap();

        pairs.push((pack1, pack2));

        lines.next();
    }

    let sum_of_correct_idxs: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
        .sum();

    println!("Part 1:");
    println!("{}", sum_of_correct_idxs);

    let mut packets: Vec<Packet> = pairs
        .iter()
        .flat_map(|(p1, p2)| [p1.clone(), p2.clone()])
        .collect();

    packets.push(Packet::double_wrap(2));
    packets.push(Packet::double_wrap(6));

    packets.sort();

    let position_1 = packets
        .iter()
        .position(|p| *p == Packet::double_wrap(2))
        .unwrap()
        + 1;
    let position_2 = packets
        .iter()
        .position(|p| *p == Packet::double_wrap(6))
        .unwrap()
        + 1;

    println!("Part 2:");
    println!("{}", position_1 * position_2);
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Single(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn double_wrap(x: u32) -> Self {
        Self::List(vec![Self::List(vec![Self::Single(x)])])
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];

        let mut num_buf = None;

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push(Vec::new());
                }
                ']' => {
                    if let Some(v) = num_buf {
                        stack.last_mut().unwrap().push(Self::Single(v));
                        num_buf = None;
                    }
                    let popped = stack.pop().unwrap();

                    if let Some(list) = stack.last_mut() {
                        list.push(Self::List(popped));
                    } else {
                        return Ok(Self::List(popped));
                    }
                }
                '0'..='9' => {
                    num_buf = Some(num_buf.unwrap_or(0) * 10 + (c as u8 - '0' as u8) as u32);
                }
                ',' => {
                    if let Some(v) = num_buf {
                        stack.last_mut().unwrap().push(Self::Single(v));
                        num_buf = None;
                    }
                }
                _ => {
                    return Err(format!("Unknown char {}", c));
                }
            }
        }

        unreachable!()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;
        match (self, other) {
            (Single(x), Single(y)) => Some(x.cmp(y)),
            (List(xs), List(ys)) => {
                let res = xs.iter().zip(ys.iter()).find_map(|(x, y)| {
                    let res = x.cmp(y);
                    if res != Ordering::Equal {
                        Some(res)
                    } else {
                        None
                    }
                });
                if let Some(res) = res {
                    Some(res)
                } else {
                    Some(xs.len().cmp(&ys.len()))
                }
            }
            (List(_), Single(_)) => Some(self.cmp(&List(vec![other.clone()]))),
            (Single(_), List(_)) => Some(List(vec![self.clone()]).cmp(other)),
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
