use std::fs;

fn has_only_unique_chars(s: &str) -> bool {
    s.char_indices()
        .all(|(i1, c1)| s.char_indices().all(|(i2, c2)| c2 != c1 || i2 == i1))
}

fn index_of_unique_char_sequence(transmission: &str, search_len: usize) -> Result<usize, String> {
    let res = (0..transmission.len() - 4).find(|i| {
        let seq: String = transmission.chars().skip(*i).take(search_len).collect();
        has_only_unique_chars(&seq)
    });

    if let Some(idx) = res {
        Ok(idx)
    } else {
        Err("Could not find unique sequence".to_owned())
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();
    let transmission = input.trim();

    let start_of_packet_idx = index_of_unique_char_sequence(&transmission, 4).unwrap();
    let chars_that_were_checked_to_find_sop = start_of_packet_idx + 4;

    println!("Part 1:");
    println!("{}", chars_that_were_checked_to_find_sop);

    let start_of_message_idx = index_of_unique_char_sequence(&transmission, 14).unwrap();
    let chars_that_were_checked_to_find_som = start_of_message_idx + 14;

    println!("Part 2:");
    println!("{}", chars_that_were_checked_to_find_som);
}
