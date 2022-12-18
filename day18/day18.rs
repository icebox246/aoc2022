use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let cubes: HashSet<Cube> = input
        .split("\n")
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse().unwrap())
            }
        })
        .collect();

    let area = calculate_area_1(&cubes);
    println!("Part 1:");
    println!("{}", area);

    let area = calculate_area_2(&cubes);
    println!("Part 2:");
    println!("{}", area);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube(i32, i32, i32);

impl FromStr for Cube {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',');
        let x = nums.next().unwrap().parse()?;
        let y = nums.next().unwrap().parse()?;
        let z = nums.next().unwrap().parse()?;

        Ok(Cube(x, y, z))
    }
}

impl Cube {
    fn moved(&self, offset: (i32, i32, i32)) -> Self {
        Cube(self.0 + offset.0, self.1 + offset.1, self.2 + offset.2)
    }
}

fn calculate_area_1(voxels: &HashSet<Cube>) -> usize {
    let possible_neighbour_offsets = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    voxels
        .iter()
        .map(|cube| {
            6 - possible_neighbour_offsets
                .iter()
                .filter(|&&offset| voxels.contains(&cube.moved(offset)))
                .count()
        })
        .sum()
}

fn calculate_area_2(voxels: &HashSet<Cube>) -> usize {
    let possible_neighbour_offsets = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let min_x = voxels.iter().map(|cube| cube.0).min().unwrap();
    let min_y = voxels.iter().map(|cube| cube.1).min().unwrap();
    let min_z = voxels.iter().map(|cube| cube.2).min().unwrap();

    let max_x = voxels.iter().map(|cube| cube.0).max().unwrap();
    let max_y = voxels.iter().map(|cube| cube.1).max().unwrap();
    let max_z = voxels.iter().map(|cube| cube.2).max().unwrap();

    let seed = Cube(min_x - 1, min_y - 1, min_z - 1);
    queue.push_back(seed.clone());
    visited.insert(seed.clone());

    let mut area = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        area += possible_neighbour_offsets
            .iter()
            .map(|&offset| current.moved(offset))
            .filter(|cube| voxels.contains(&cube))
            .count();

        let next_cubes: Vec<Cube> = possible_neighbour_offsets
            .iter()
            .map(|&offset| current.moved(offset))
            .filter(|cube| !visited.contains(&cube))
            .filter(|cube| !voxels.contains(&cube))
            .filter(|cube| {
                min_x - 1 <= cube.0
                    && cube.0 <= max_x + 1
                    && min_y - 1 <= cube.1
                    && cube.1 <= max_y + 1
                    && min_z - 1 <= cube.2
                    && cube.2 <= max_z + 1
            })
            .collect();

        for next_cube in next_cubes {
            visited.insert(next_cube.clone());
            queue.push_back(next_cube.clone());
        }
    }

    area
}
