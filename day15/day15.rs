use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let sensors: Vec<Sensor> = input
        .split('\n')
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.parse().unwrap())
            } else {
                None
            }
        })
        .collect();
    {
        let row_to_check = 10;
        // let row_to_check = 2000000;

        let mut ranges_in_rows: HashMap<i64, Vec<Range>> = HashMap::new();
        sensors
            .iter()
            .inspect(|sensor| println!("Processing {:?}", sensor))
            .for_each(|sensors| {
                sensors.mark_ranges(
                    &mut ranges_in_rows,
                    (Range(i64::MIN, i64::MAX), Range(row_to_check, row_to_check)),
                    false,
                );
            });

        let mut ranges = ranges_in_rows[&row_to_check].clone();
        ranges.sort();
        ranges = compress_row(&ranges);
        let blocked_spots_count: usize = ranges.iter().map(|r| r.size()).sum();

        println!("Part 1: (y = {})", row_to_check);
        println!("{}", blocked_spots_count);
    }

    {
        // let search_limit = 20;
        let search_limit = 4000000;

        let mut ranges_in_rows: HashMap<i64, Vec<Range>> = HashMap::new();
        sensors
            .iter()
            .inspect(|sensor| println!("Processing {:?}", sensor))
            .for_each(|sensors| {
                sensors.mark_ranges(
                    &mut ranges_in_rows,
                    (Range(0, search_limit), Range(0, search_limit)),
                    true,
                );
            });

        ranges_in_rows.iter_mut().for_each(|(_, row)| {
            row.sort();
            *row = compress_row(row);
        });

        let (sus_y, sus_ranges) = ranges_in_rows
            .iter()
            .inspect(|(y, _)| {
                if **y % 1000 == 0 {
                    println!("Inspected up to {}", **y - 1);
                }
            })
            .find(|(y, ranges)| {
                let blocked: usize = ranges.iter().map(|r| r.size()).sum();
                blocked <= search_limit as usize
            })
            .unwrap();

        let sus_x = if sus_ranges.len() == 2 {
            sus_ranges.first().unwrap().1 + 1
        } else if let Some(Range(0, d)) = sus_ranges.first() {
            d + 1
        } else {
            0
        };

        let tuning_frequency = sus_x * 4000000 + sus_y;

        println!("Part 2: (limit = {})", search_limit);
        println!("{}", tuning_frequency);
    }
}

#[derive(Debug)]
struct Sensor {
    location: (i64, i64),
    beacon: (i64, i64),
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_named_var(s: &str) -> i64 {
            let var = s.split_once('=').unwrap().1;
            var.parse::<i64>().unwrap()
        }
        fn parse_point(s: &str) -> (i64, i64) {
            let (x, y) = s.split_once(", ").unwrap();
            (parse_named_var(x), parse_named_var(y))
        }

        let first_part = s.split_once(':').unwrap().0;
        let location = first_part.strip_prefix("Sensor at ").unwrap();
        let location = parse_point(location);

        let beacon = s.rsplit_once("at ").unwrap().1;
        let beacon = parse_point(beacon);

        Ok(Self { location, beacon })
    }
}

impl Sensor {
    fn mark_ranges(
        &self,
        rows: &mut HashMap<i64, Vec<Range>>,
        bounds: (Range, Range),
        only_hidden: bool,
    ) {
        let radius =
            (self.location.0 - self.beacon.0).abs() + (self.location.1 - self.beacon.1).abs();

        for y in ((self.location.1 - radius).max(bounds.1 .0))
            ..=((self.location.1 + radius).min(bounds.1 .1))
        {
            let current_radius = radius - (self.location.1 - y).abs();
            if !rows.contains_key(&y) {
                rows.insert(y, vec![]);
            }
            if y == self.location.1 && !only_hidden {
                let range =
                    Range(self.location.0 - radius, self.location.0 - 1).intesection(&bounds.0);
                if let Some(range) = range {
                    rows.get_mut(&y).unwrap().push(range);
                }
                let range =
                    Range(self.location.0 + 1, self.location.0 + radius).intesection(&bounds.0);
                if let Some(range) = range {
                    rows.get_mut(&y).unwrap().push(range);
                }
            } else if y == self.beacon.1 && !only_hidden {
                if current_radius == 0 {
                    continue;
                }
                if self.beacon.0 > self.location.0 {
                    let range = Range(
                        self.location.0 - current_radius,
                        self.location.0 + current_radius - 1,
                    )
                    .intesection(&bounds.0);
                    if let Some(range) = range {
                        rows.get_mut(&y).unwrap().push(range);
                    }
                } else {
                    let range = Range(
                        self.location.0 - current_radius + 1,
                        self.location.0 + current_radius,
                    )
                    .intesection(&bounds.0);
                    if let Some(range) = range {
                        rows.get_mut(&y).unwrap().push(range);
                    }
                }
            } else {
                let range = Range(
                    self.location.0 - current_radius,
                    self.location.0 + current_radius,
                )
                .intesection(&bounds.0);
                if let Some(range) = range {
                    rows.get_mut(&y).unwrap().push(range);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Range(i64, i64);

impl Eq for Range {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Range {
    fn intesects(&self, other: &Self) -> bool {
        (self.0 <= other.0 && self.1 >= other.0)
            || (self.0 <= other.1 && self.1 >= other.1)
            || (other.0 <= self.0 && other.1 >= self.0)
            || (other.0 <= self.1 && other.1 >= self.1)
            || self.1 + 1 == other.0
            || other.1 + 1 == self.0
    }

    fn union(&self, other: &Self) -> Self {
        Range(self.0.min(other.0), self.1.max(other.1))
    }

    fn intesection(&self, other: &Self) -> Option<Self> {
        if self.intesects(other) {
            Some(Range(self.0.max(other.0), self.1.min(other.1)))
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }
}

fn compress_row(ranges: &Vec<Range>) -> Vec<Range> {
    let mut new_ranges = vec![];
    let mut current = ranges.first().unwrap().clone();

    for range in ranges {
        if range.intesects(&current) {
            current = range.union(&current);
        } else {
            new_ranges.push(current);
            current = range.clone();
        }
    }
    new_ranges.push(current);

    return new_ranges;
}
