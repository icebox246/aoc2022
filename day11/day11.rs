use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let line_count = input.split('\n').count();

    {
        let mut gang = {
            let mut lines = input.split('\n').take(line_count - 1).map(|s| s.to_owned());
            MonkeyGang::from_lines(&mut lines, &MagicPreferance::NoMagic).unwrap()
        };
        for _ in 0..20 {
            gang.do_round(&ReliefRule::FeelsRelief);
        }

        let mut counts = gang.inspection_counts.clone();
        counts.sort();

        let result: usize = counts.iter().rev().take(2).product();

        println!("Part 1:");
        println!("{}", result);
    }

    {
        let mut gang = {
            let mut lines = input.split('\n').take(line_count - 1).map(|s| s.to_owned());
            MonkeyGang::from_lines(&mut lines, &MagicPreferance::Magic).unwrap()
        };
        for _ in 0..10000 {
            gang.do_round(&ReliefRule::NoRelief);
        }

        let mut counts = gang.inspection_counts.clone();
        counts.sort();

        let result: usize = counts.iter().rev().take(2).product();

        println!("Part 2:");
        println!("{}", result);
    }
}

enum ReliefRule {
    FeelsRelief,
    NoRelief,
}

#[derive(Debug, Clone)]
struct MonkeyGang {
    monkeys: Vec<Monkey>,
    inspection_counts: Vec<usize>,
}

impl MonkeyGang {
    fn from_lines<I>(lines: &mut I, magic_pref: &MagicPreferance) -> Result<Self, ()>
    where
        I: Iterator<Item = String>,
    {
        let mut monkeys = vec![];

        loop {
            let monkey = Monkey::from_lines(lines, magic_pref)?;
            monkeys.push(monkey);

            if lines.next().is_none() {
                break;
            }
        }

        let gang = MonkeyGang {
            inspection_counts: vec![0; monkeys.len()],
            monkeys,
        };

        Ok(gang)
    }

    fn do_round(&mut self, relief_rule: &ReliefRule) {
        for i in 0..(self.monkeys.len()) {
            let throws = self.monkeys[i].turn(relief_rule);

            // monkey must have inspected all thrown items
            self.inspection_counts[i] += throws.len();

            for t in throws.iter() {
                self.monkeys[t.recipient].items.push(t.item.clone());
            }
        }

        // for (i, monkey) in self.monkeys.iter().enumerate() {
        //     println!("{}: {:?}", i, monkey.items);
        // }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<MagicInt>,
    inspect_operation: Operation,
    test_divisor: usize,
    true_result_monkey: usize,
    false_result_monkey: usize,
}

impl Monkey {
    fn from_lines<I>(lines: &mut I, magic_pref: &MagicPreferance) -> Result<Self, ()>
    where
        I: Iterator<Item = String>,
    {
        fn wrap_opt<T>(o: Option<T>) -> Result<T, ()> {
            o.map(|i| Ok(i)).unwrap_or(Err(()))
        }

        fn wrap_res<T, U>(o: Result<T, U>) -> Result<T, ()> {
            o.map(|i| Ok(i)).unwrap_or(Err(()))
        }
        // ignore monkey number line
        let _ = lines.next();

        // starting items line
        let line = wrap_opt(lines.next())?;
        let line = wrap_opt(line.strip_prefix("  Starting items: "))?;
        let items: Vec<MagicInt> = line
            .split(", ")
            .map(|s| MagicInt::new(s.parse::<usize>().unwrap(), magic_pref))
            .collect();

        // inspect operation line
        let line = wrap_opt(lines.next())?;
        let line = wrap_opt(line.strip_prefix("  Operation: new = "))?;
        let inspect_operation = line.parse::<Operation>()?;

        // test divisor line
        let line = wrap_opt(lines.next())?;
        let line = wrap_opt(line.strip_prefix("  Test: divisible by "))?;
        let test_divisor = wrap_res(line.parse::<usize>())?;

        // true result monkey line
        let line = wrap_opt(lines.next())?;
        let line = wrap_opt(line.strip_prefix("    If true: throw to monkey "))?;
        let true_result_monkey = wrap_res(line.parse::<usize>())?;

        // false result monkey line
        let line = wrap_opt(lines.next())?;
        let line = wrap_opt(line.strip_prefix("    If false: throw to monkey "))?;
        let false_result_monkey = wrap_res(line.parse::<usize>())?;

        Ok(Monkey {
            items,
            inspect_operation,
            test_divisor,
            true_result_monkey,
            false_result_monkey,
        })
    }

    fn turn(&mut self, relief_rule: &ReliefRule) -> Vec<MonkeyThrow> {
        let mut throws = vec![];

        self.items.iter().for_each(|item| {
            let new = self.inspect_operation.eval(item);
            let new = match relief_rule {
                ReliefRule::FeelsRelief => new.divide_by(&3),
                ReliefRule::NoRelief => new,
            };
            if new.is_divisible_by(&self.test_divisor) {
                throws.push(MonkeyThrow {
                    item: new,
                    recipient: self.true_result_monkey,
                });
            } else {
                throws.push(MonkeyThrow {
                    item: new,
                    recipient: self.false_result_monkey,
                });
            }
        });

        self.items.clear();

        throws
    }
}

#[derive(Debug)]
struct MonkeyThrow {
    item: MagicInt,
    recipient: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn eval(&self, x: &MagicInt) -> MagicInt {
        use Operation::*;
        match self {
            Multiply(y) => x.mul(*y),
            Add(y) => x.add(*y),
            Square => x.square(),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        match s.split(' ').collect::<Vec<&str>>()[..] {
            ["old", "*", "old"] => Ok(Square),
            ["old", "*", num] => Ok(Multiply(num.parse::<usize>().unwrap())),
            ["old", "+", num] => Ok(Add(num.parse::<usize>().unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum MagicInt {
    Magic { rests: HashMap<usize, usize> },
    Normal(usize),
}

enum MagicPreferance {
    Magic,
    NoMagic,
}

impl MagicInt {
    const MAX_DIVISOR: usize = 31;

    fn new(x: usize, pref: &MagicPreferance) -> Self {
        match pref {
            MagicPreferance::Magic => {
                let mut rests = HashMap::new();

                'loop_over_divisors: for i in 2..=MagicInt::MAX_DIVISOR {
                    for j in 2..i {
                        if i % j == 0 {
                            continue 'loop_over_divisors;
                        }
                    }

                    // i is prime
                    rests.insert(i, x % i);
                }

                MagicInt::Magic { rests }
            }
            MagicPreferance::NoMagic => MagicInt::Normal(x),
        }
    }

    fn mul(&self, rhs: usize) -> Self {
        match self {
            Self::Magic { rests } => Self::Magic {
                rests: rests.iter().map(|(d, r)| (*d, r * rhs % d)).collect(),
            },
            Self::Normal(x) => Self::Normal(x * rhs),
        }
    }

    fn add(&self, rhs: usize) -> Self {
        match self {
            Self::Magic { rests } => Self::Magic {
                rests: rests.iter().map(|(d, r)| (*d, (r + rhs) % d)).collect(),
            },
            Self::Normal(x) => Self::Normal(x + rhs),
        }
    }

    fn square(&self) -> Self {
        match self {
            Self::Magic { rests } => Self::Magic {
                rests: rests.iter().map(|(d, r)| (*d, (r * r) % d)).collect(),
            },
            Self::Normal(x) => Self::Normal(x * x),
        }
    }

    fn is_divisible_by(&self, x: &usize) -> bool {
        match self {
            Self::Magic { rests } => {
                assert!(rests.contains_key(x));
                rests[x] == 0
            }
            Self::Normal(y) => y % x == 0,
        }
    }

    fn divide_by(&self, x: &usize) -> Self {
        match self {
            Self::Magic { rests: _ } => todo!("Magic divide no worky"),
            Self::Normal(y) => Self::Normal(y / x),
        }
    }
}
