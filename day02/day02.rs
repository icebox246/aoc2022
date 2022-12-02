use std::fs;

#[derive(Debug, PartialEq)]
enum Shapes {
    Rock,
    Paper,
    Scissors,
}

impl Shapes {
    fn from_oponent(character: char) -> Result<Shapes, String> {
        match character {
            'A' => Ok(Shapes::Rock),
            'B' => Ok(Shapes::Paper),
            'C' => Ok(Shapes::Scissors),
            _ => Err("unknown character".to_owned()),
        }
    }
    fn from_my(character: char) -> Result<Shapes, String> {
        match character {
            'X' => Ok(Shapes::Rock),
            'Y' => Ok(Shapes::Paper),
            'Z' => Ok(Shapes::Scissors),
            _ => Err("unknown character".to_owned()),
        }
    }

    fn get_better(&self) -> Shapes {
        match self {
            Shapes::Rock => Shapes::Paper,
            Shapes::Paper => Shapes::Scissors,
            Shapes::Scissors => Shapes::Rock,
        }
    }

    fn get_worse(&self) -> Shapes {
        match self {
            Shapes::Paper => Shapes::Rock,
            Shapes::Scissors => Shapes::Paper,
            Shapes::Rock => Shapes::Scissors,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shapes::Rock => 1,
            Shapes::Paper => 2,
            Shapes::Scissors => 3,
        }
    }
}

fn score_round(oponent: Shapes, my: Shapes) -> u32 {
    (my.score())
        + (if oponent == my.get_better() {
            0
        } else if oponent == my {
            3
        } else {
            6
        })
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();

    {
        let mut total_score = 0u32;

        for line in input.split('\n').filter(|line| !line.is_empty()) {
            let oponent_shape = Shapes::from_oponent(line.chars().next().unwrap()).unwrap();
            let my_shape = Shapes::from_my(line.chars().skip(2).next().unwrap()).unwrap();

            total_score += score_round(oponent_shape, my_shape);
        }

        println!("Part 1:");
        println!("{}", total_score);
    }

    {
        let mut total_score = 0u32;

        for line in input.split('\n').filter(|line| !line.is_empty()) {
            let oponent_shape = Shapes::from_oponent(line.chars().next().unwrap()).unwrap();
            let outcome = line.chars().skip(2).next().unwrap();

            total_score += if outcome == 'X' {
                oponent_shape.get_worse().score()
            } else if outcome == 'Y' {
                3 + oponent_shape.score()
            } else {
                6 + oponent_shape.get_better().score()
            };
        }

        println!("Part 2:");
        println!("{}", total_score);
    }
}
