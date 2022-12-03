use std::collections::HashSet;
use std::fs;

fn calculate_priority(name: &char) -> u32 {
    match name {
        'a'..='z' => (*name as u8 - 'a' as u8) as u32 + 1,
        'A'..='Z' => (*name as u8 - 'A' as u8) as u32 + 27,
        _ => 0,
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();

    {
        let mut total_priority = 0;

        for line in input.split('\n').filter(|l| !l.is_empty()) {
            let set1: HashSet<_> = line.get(..line.len() / 2).unwrap().chars().collect();
            let set2: HashSet<_> = line.get(line.len() / 2..).unwrap().chars().collect();

            let priority: u32 = set1
                .intersection(&set2)
                .into_iter()
                .map(|name| calculate_priority(name))
                .sum();

            total_priority += priority;
        }

        println!("Part 1:");
        println!("{}", total_priority);
    }

    {
        let mut total_priority = 0;

        let mut sets: Vec<HashSet<char>> = vec![];

        for line in input.split('\n').filter(|l| !l.is_empty()) {
            if sets.len() < 3 {
                sets.push(line.chars().collect());
            }
            if sets.len() == 3 {
                let mut common = sets[0].clone();
                for set in sets.iter().skip(1) {
                    common = common
                        .intersection(&set)
                        .map(|c| *c)
                        .collect::<HashSet<char>>();
                }

                total_priority += calculate_priority(
                    &common
                        .into_iter()
                        .next()
                        .expect("There must be a common item in a 3-group"),
                );

                sets.clear();
            }
        }

        println!("Part 2:");
        println!("{}", total_priority);
    }
}
