use std::fmt::Display;
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let snafus: Vec<SNAFU> = input
        .split('\n')
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.parse::<SNAFU>().unwrap())
            } else {
                None
            }
        })
        .collect();

    let sum = SNAFU::new(snafus.iter().map(|s| s.num).sum());

    println!("Part 1:");
    println!("{}", sum);
}

#[derive(Debug)]
struct SNAFU {
    num: usize,
}

impl FromStr for SNAFU {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buff = 0i64;

        for c in s.chars() {
            buff *= 5;
            match c {
                '2' => buff += 2,
                '1' => buff += 1,
                '0' => buff += 0,
                '-' => buff += -1,
                '=' => buff += -2,
                _ => {
                    return Err(format!("Unknown character `{}`", c));
                }
            }
        }

        assert!(buff > 0);
        Ok(Self { num: buff as usize })
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // this algorithm asserts that the represented muber is positive and fits in 24 digits
        const MAX_DIGITS: usize = 24;
        let mut char_buffer = ['0'; MAX_DIGITS];

        let mut num = self.num + {
            let mut offset = 0;
            for _ in 0..MAX_DIGITS {
                offset *= 5;
                offset += 2;
            }
            offset
        };

        for i in (0..MAX_DIGITS).rev() {
            char_buffer[i] = match num % 5 {
                0 => '=',
                1 => '-',
                2 => '0',
                3 => '1',
                4 => '2',
                _ => unreachable!(),
            };
            num /= 5;
        }

        let first_non_zero = char_buffer.iter().position(|&c| c != '0').unwrap();

        let s: String = char_buffer[first_non_zero..].iter().collect();

        write!(f, "{}", s);

        Ok(())
    }
}

impl SNAFU {
    fn new(num: usize) -> Self {
        Self { num }
    }
}
