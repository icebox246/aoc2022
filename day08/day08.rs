fn get_visible_matrix(grid: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut visible: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    for y in 0..height {
        let mut highest = -1;
        for x in 0..width {
            if highest < grid[y][x] as i8 {
                visible[y][x] = true;
                highest = grid[y][x] as i8;
            }
        }
        let mut highest = -1;
        for x in (0..width).rev() {
            if highest < grid[y][x] as i8 {
                visible[y][x] = true;
                highest = grid[y][x] as i8;
            }
        }
    }

    for x in 0..width {
        let mut highest = -1;
        for y in 0..height {
            if highest < grid[y][x] as i8 {
                visible[y][x] = true;
                highest = grid[y][x] as i8;
            }
        }
        let mut highest = -1;
        for y in (0..height).rev() {
            if highest < grid[y][x] as i8 {
                visible[y][x] = true;
                highest = grid[y][x] as i8;
            }
        }
    }

    visible
}

fn count_true_in_grid(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|s| **s).count())
        .sum()
}

fn get_scenic_scores(grid: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut score: Vec<Vec<usize>> = grid
        .iter()
        .map(|row| row.iter().map(|_| 1).collect())
        .collect();

    let mut running: Vec<u8> = vec![];

    for y in 0..height {
        running.clear();
        for x in 0..width {
            score[y][x] *= running
                .iter()
                .rev()
                .position(|h| (*h) >= grid[y][x])
                .map(|p| p + 1)
                .unwrap_or(running.len());
            running.push(grid[y][x]);
        }
        running.clear();
        for x in (0..width).rev() {
            score[y][x] *= running
                .iter()
                .rev()
                .position(|h| (*h) >= grid[y][x])
                .map(|p| p + 1)
                .unwrap_or(running.len());
            running.push(grid[y][x]);
        }
    }

    for x in 0..width {
        running.clear();
        for y in 0..height {
            score[y][x] *= running
                .iter()
                .rev()
                .position(|h| (*h) >= grid[y][x])
                .map(|p| p + 1)
                .unwrap_or(running.len());
            running.push(grid[y][x]);
        }
        running.clear();
        for y in (0..height).rev() {
            score[y][x] *= running
                .iter()
                .rev()
                .position(|h| (*h) >= grid[y][x])
                .map(|p| p + 1)
                .unwrap_or(running.len());
            running.push(grid[y][x]);
        }
    }

    score
}

fn max_in_grid(grid: &Vec<Vec<usize>>) -> usize {
    grid.iter()
        .map(|row| *row.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(0)
}

fn main() {
    // let filepath = "test.in";
    let filepath = "my.in";
    let input = std::fs::read_to_string(filepath).unwrap();

    let grid: Vec<Vec<u8>> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect();

    let visible = get_visible_matrix(&grid);

    println!("Part 1:");
    println!("{:?}", count_true_in_grid(&visible));

    let scenic_scores = get_scenic_scores(&grid);

    println!("Part 2:");
    println!("{:?}", max_in_grid(&scenic_scores));
}
