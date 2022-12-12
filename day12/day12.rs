use std::collections::VecDeque;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let grid = Grid::from_lines(
        &input
            .split('\n')
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_owned())
                }
            })
            .collect(),
    )
    .unwrap();

    println!("Part 1:");
    println!("{}", grid.find_shortest_path_start_end());

    println!("Part 2:");
    println!("{}", grid.find_shortest_path_end_a());
}

#[derive(Debug, Clone, PartialEq)]
enum TileKind {
    Starting,
    Ending,
    Normal,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<(usize, TileKind)>>,
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Result<Self, String>
where {
        let mut tiles = vec![];
        let mut row = vec![];
        for line in lines {
            for c in line.chars() {
                row.push(match c {
                    'a'..='z' => Ok((((c as u8 - 'a' as u8) as usize), TileKind::Normal)),
                    'S' => Ok((0, TileKind::Starting)),
                    'E' => Ok((25, TileKind::Ending)),
                    c => Err(format!("Unknown character '{}'", c)),
                }?);
            }
            tiles.push(row.clone());
            row.clear();
        }

        Ok(Self { tiles })
    }

    fn find_starting(&self) -> Option<(usize, usize)> {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x].1 == TileKind::Starting {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn find_ending(&self) -> Option<(usize, usize)> {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x].1 == TileKind::Ending {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn get_elevation(&self, p: &(usize, usize)) -> Option<usize> {
        self.tiles
            .get(p.1)
            .map(|row| row.get(p.0))
            .map(|tile_opt| tile_opt.map(|tile| tile.0))
            .unwrap_or(None)
    }

    fn find_all_zero(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x].0 == 0 {
                    res.push((x, y));
                }
            }
        }
        res
    }

    fn find_distances(&self, starting: &Vec<(usize, usize)>) -> Vec<Vec<Option<usize>>> {
        let mut distance: Vec<Vec<Option<usize>>> = (0..self.tiles.len())
            .map(|_| (0..self.tiles[0].len()).map(|_| None).collect())
            .collect();

        let mut queue = VecDeque::new();
        queue.extend(starting.iter());
        starting.iter().for_each(|&(x, y)| distance[y][x] = Some(0));

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let current_elevation = self.get_elevation(&current).unwrap();
            let current_distance = distance[current.1][current.0].unwrap();

            let mut candidates = vec![(current.0 + 1, current.1), (current.0, current.1 + 1)];
            if current.0 > 0 {
                candidates.push((current.0 - 1, current.1))
            }
            if current.1 > 0 {
                candidates.push((current.0, current.1 - 1))
            }

            for cand in candidates {
                if let Some(cand_elevation) = self.get_elevation(&cand) {
                    if distance[cand.1][cand.0].is_none()
                        && cand_elevation as i32 - current_elevation as i32 <= 1
                    {
                        distance[cand.1][cand.0] = Some(current_distance + 1);
                        queue.push_back(cand);
                    }
                }
            }
        }

        distance
    }

    fn find_shortest_path_start_end(&self) -> usize {
        let starting = self.find_starting().unwrap();

        let distance = self.find_distances(&vec![starting]);

        let ending = self.find_ending().unwrap();
        distance[ending.1][ending.0].unwrap()
    }

    fn find_shortest_path_end_a(&self) -> usize {
        let starting = self.find_all_zero();

        let distance = self.find_distances(&starting);

        let ending = self.find_ending().unwrap();
        distance[ending.1][ending.0].unwrap()
    }
}
