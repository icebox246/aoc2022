use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    begin: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin_str, end_str) = s.split_once('-').unwrap();

        let begin: u32 = begin_str.parse()?;
        let end: u32 = end_str.parse()?;

        Ok(Range { begin, end })
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (other.begin <= self.begin && self.begin <= other.end)
            || (other.begin <= self.end && self.end <= other.end)
            || self.contains(other)
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();

    let mut containing_count = 0;
    let mut overlappping_count = 0;

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let (range1_str, range2_str) = line.split_once(',').unwrap();

        let range1: Range = range1_str.parse().unwrap();
        let range2: Range = range2_str.parse().unwrap();

        if range1.contains(&range2) || range2.contains(&range1) {
            containing_count += 1;
        }

        if range1.overlaps(&range2) {
            overlappping_count += 1;
        }
    }

    println!("Part 1:");
    println!("{}", containing_count);

    println!("Part 2:");
    println!("{}", overlappping_count);
}
