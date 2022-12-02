use std::fs;

fn main() {
    let input = fs::read_to_string("my.in").expect("Must be able to read!");

    let mut current_sum = 0;
    let mut biggest_sum = 0;

    for line in input.split("\n") {
        if line.len() > 0 {
            current_sum += line
                .parse::<i32>()
                .expect("Each line must contain a proper number!");
        } else {
            biggest_sum = biggest_sum.max(current_sum);
            current_sum = 0;
        }
    }

    println!("Part 1:");
    println!("{}", biggest_sum);

    let mut sums: Vec<i32> = vec![];

    for line in input.split("\n") {
        if line.len() > 0 {
            current_sum += line
                .parse::<i32>()
                .expect("Each line must contain a proper number!");
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }

    sums.sort();
    let ans2: i32 = sums.iter().rev().take(3).sum();

    println!("Part 2:");
    println!("{}", ans2);
}
