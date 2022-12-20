type List = Vec<(i64, usize)>;

fn main() {
    let filename = "test.in";
    // let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let list: Vec<i64> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            } else {
                Some(line.parse().unwrap())
            }
        })
        .collect();

    let list: List = list
        .iter()
        .cloned()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect();

    {
        let mixed = mix_list(&list);

        let part1 = find_result(&mixed);

        println!("Part 1:");
        println!("{}", part1);
    }

    {
        let key = 811589153;
        let keyed: List = list.iter().map(|&(n, i)| (n * key, i)).collect();

        let mut mixed = keyed.clone();
        for i in 0..10 {
            mixed = mix_list(&mixed);
        }

        let part2 = find_result(&mixed);

        println!("Part 2:");
        println!("{}", part2);
    }
}

fn mix_list(list: &List) -> List {
    let len = list.len();
    let mut list = list.clone();

    for it in 0..len {
        let i = list.iter().position(|&p| p.1 == it).unwrap();

        let offset = list[i].0;

        let offset = (offset % (len - 1) as i64 + (len - 1) as i64) as usize % (len - 1);
        for j in i..(i + offset) {
            let a = (j as usize) % len;
            let b = (a + 1) % len;
            list.swap(a, b);
        }
    }

    list
}

fn find_result(list: &List) -> i64 {
    let zero = list.iter().position(|&n| n.0 == 0).unwrap();
    let len = list.len();

    list[(zero + 1000) % len].0 + list[(zero + 2000) % len].0 + list[(zero + 3000) % len].0
}
