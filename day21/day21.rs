use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let monkey_map: HashMap<MonkeyName, MathMonkey> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(MathMonkey::from_description(line))
            }
        })
        .collect();

    {
        let mut sim = Simulation::new(monkey_map.clone());

        let root_name = "root".parse().unwrap();

        let root_num = sim.ask_for(&root_name);
        let root_num = root_num.top.0[0] / root_num.bottom.0[0];

        println!("Part 1:");
        println!("{}", root_num);
    }

    {
        let root_name = "root".parse().unwrap();

        let mut monkey_map = monkey_map.clone();

        let humn_name: MonkeyName = "humn".parse().unwrap();

        monkey_map.insert(
            humn_name.clone(),
            MathMonkey::Constant(Polynomial::single(1)),
        );

        let a_name = monkey_map[&root_name].get_left();
        let b_name = monkey_map[&root_name].get_right();

        let mut sim = Simulation::new(monkey_map);

        let a = sim.ask_for(&a_name);
        let b = sim.ask_for(&b_name);

        let root = a - b;

        let equ = root.top;

        let solution = equ.solve0();

        println!("Part 2:");
        println!("{}", solution);
    }
}

#[derive(Clone, Debug)]
enum MathMonkey {
    Constant(Polynomial),
    Addition(MonkeyName, MonkeyName),
    Multiplication(MonkeyName, MonkeyName),
    Division(MonkeyName, MonkeyName),
    Subtraction(MonkeyName, MonkeyName),
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
struct MonkeyName(u32);

impl FromStr for MonkeyName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<u8> = s.bytes().collect();
        if chars.len() != 4 {
            return Err("Wrong amount of chars".to_owned());
        }

        let mut name = 0;

        for c in chars {
            name *= 0x100;
            name += c as u32;
        }

        Ok(MonkeyName(name))
    }
}

impl FromStr for MathMonkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ');
        use MathMonkey::*;
        match words.collect::<Vec<&str>>()[..] {
            [a, "+", b] => Ok(Addition(a.parse().unwrap(), b.parse().unwrap())),
            [a, "-", b] => Ok(Subtraction(a.parse().unwrap(), b.parse().unwrap())),
            [a, "/", b] => Ok(Division(a.parse().unwrap(), b.parse().unwrap())),
            [a, "*", b] => Ok(Multiplication(a.parse().unwrap(), b.parse().unwrap())),
            [a] => Ok(Constant(Polynomial::free(a.parse().unwrap()))),
            _ => Err(format!("Unknown expression {}", s)),
        }
    }
}

impl MathMonkey {
    fn from_description(s: &str) -> (MonkeyName, MathMonkey) {
        let (name, expr) = s.split_once(": ").unwrap();
        let name = name.parse().unwrap();
        let monkey = expr.parse().unwrap();

        (name, monkey)
    }

    fn get_left(&self) -> MonkeyName {
        match self {
            Self::Addition(l, _) => l.clone(),
            Self::Multiplication(l, _) => l.clone(),
            Self::Division(l, _) => l.clone(),
            Self::Subtraction(l, _) => l.clone(),
            Self::Constant(_) => panic!("not an expression"),
        }
    }
    fn get_right(&self) -> MonkeyName {
        match self {
            Self::Addition(_, r) => r.clone(),
            Self::Multiplication(_, r) => r.clone(),
            Self::Division(_, r) => r.clone(),
            Self::Subtraction(_, r) => r.clone(),
            Self::Constant(_) => panic!("not an expression"),
        }
    }
}

struct Simulation {
    values: HashMap<MonkeyName, PolynomialFraction>,
    monkey_map: HashMap<MonkeyName, MathMonkey>,
}

impl Simulation {
    fn new(monkey_map: HashMap<MonkeyName, MathMonkey>) -> Self {
        Self {
            values: HashMap::new(),
            monkey_map,
        }
    }

    fn ask_for(&mut self, name: &MonkeyName) -> PolynomialFraction {
        if self.values.contains_key(name) {
            return self.values[name].clone();
        }

        let monkey = self.monkey_map[name].clone();

        let result = match monkey {
            MathMonkey::Constant(num) => PolynomialFraction::from_poly(num),
            MathMonkey::Addition(a, b) => self.ask_for(&a) + self.ask_for(&b),
            MathMonkey::Subtraction(a, b) => self.ask_for(&a) - self.ask_for(&b),
            MathMonkey::Multiplication(a, b) => self.ask_for(&a) * self.ask_for(&b),
            MathMonkey::Division(a, b) => self.ask_for(&a) / self.ask_for(&b),
        };

        self.values.insert(*name, result.clone());

        result
    }
}

#[derive(Debug, Clone, Copy)]
struct Fract {
    top: i128,
    bottom: i128,
}

fn gcd(x: i128, y: i128) -> i128 {
    if x < 0 || y < 0 {
        return gcd(x.abs(), y.abs());
    } else if x < y {
        return gcd(y, x);
    } else if y == 0 {
        return x;
    } else {
        return gcd(y, x % y);
    }
}

impl Fract {
    fn from_int(x: i64) -> Self {
        Self {
            top: x as i128,
            bottom: 1,
        }
    }

    fn simplify(&self) -> Self {
        let g = gcd(self.top, self.bottom);
        let mut res = Self {
            top: self.top / g,
            bottom: self.bottom / g,
        };

        if res.top < 0 && res.bottom < 0 {
            res.top *= -1;
            res.bottom *= -1;
        }

        res
    }
}

impl Display for Fract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bottom != 1 {
            write!(f, "({} / {})", self.top, self.bottom)
        } else {
            write!(f, "{}", self.top)
        }
    }
}

impl Add for Fract {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res = Self {
            top: self.top * rhs.bottom + rhs.top * self.bottom,
            bottom: self.bottom * rhs.bottom,
        }
        .simplify();

        res
    }
}

impl Sub for Fract {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = Self {
            top: self.top * rhs.bottom - rhs.top * self.bottom,
            bottom: self.bottom * rhs.bottom,
        }
        .simplify();

        res
    }
}

impl Mul for Fract {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = Self {
            top: self.top * rhs.top,
            bottom: self.bottom * rhs.bottom,
        }
        .simplify();

        res
    }
}

impl Div for Fract {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let res = Self {
            top: self.top * rhs.bottom,
            bottom: self.bottom * rhs.top,
        }
        .simplify();

        res
    }
}

#[derive(Debug, Clone)]
struct Polynomial(Vec<Fract>);

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(self.0.len()) {
            let a = self.0[i];
            if i != 0 {
                write!(f, " + ")?;
                write!(f, "{}x", a)?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            } else {
                write!(f, "{}", a)?;
            }
        }

        Ok(())
    }
}

impl Polynomial {
    fn free(x: i64) -> Self {
        Self(vec![Fract::from_int(x)])
    }

    fn single(pow: usize) -> Self {
        let mut res = vec![];

        res.extend((0..pow).map(|_| Fract::from_int(0)));
        res.push(Fract::from_int(1));

        Self(res)
    }

    fn gcd(&self) -> i128 {
        let mut g = 0;
        for a in &self.0 {
            g = gcd(a.top, g);
        }
        g
    }

    fn force_divide(&self, x: i128) -> Self {
        Self(
            self.0
                .iter()
                .map(|&a| a / Fract::from_int(x as i64))
                .collect(),
        )
    }

    fn solve0(&self) -> Fract {
        if self.0.len() == 1 {
            self.0[0].clone()
        } else {
            (self.0[0] / self.0[1]) * Fract::from_int(-1)
        }
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.0.len().max(rhs.0.len());

        let mut res = vec![];

        for i in 0..len {
            res.push(
                *self.0.get(i).unwrap_or(&Fract::from_int(0))
                    + *rhs.0.get(i).unwrap_or(&Fract::from_int(0)),
            );
        }

        Self(res)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.0.len().max(rhs.0.len());

        let mut res = vec![];

        for i in 0..len {
            res.push(
                *self.0.get(i).unwrap_or(&Fract::from_int(0))
                    - *rhs.0.get(i).unwrap_or(&Fract::from_int(0)),
            );
        }

        Self(res)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.0.len() + rhs.0.len() - 1;

        let mut res = vec![Fract::from_int(0); len];

        for i in 0..self.0.len() {
            for j in 0..rhs.0.len() {
                res[i + j] = res[i + j] + self.0[i] * rhs.0[j];
            }
        }

        Self(res)
    }
}

#[derive(Clone)]
struct PolynomialFraction {
    top: Polynomial,
    bottom: Polynomial,
}

impl Display for PolynomialFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} ) / ( {} )", self.top, self.bottom)?;
        Ok(())
    }
}

impl PolynomialFraction {
    fn from_poly(p: Polynomial) -> Self {
        Self {
            top: p,
            bottom: Polynomial::free(1),
        }
    }

    fn simplify(&self) -> Self {
        let g = gcd(self.top.gcd(), self.bottom.gcd());
        Self {
            top: self.top.force_divide(g),
            bottom: self.bottom.force_divide(g),
        }
    }
}

impl Add for PolynomialFraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top * rhs.bottom.clone() + rhs.top * self.bottom.clone(),
            bottom: self.bottom * rhs.bottom,
        }
        .simplify()
    }
}

impl Sub for PolynomialFraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top * rhs.bottom.clone() - rhs.top * self.bottom.clone(),
            bottom: self.bottom * rhs.bottom,
        }
        .simplify()
    }
}

impl Mul for PolynomialFraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top * rhs.top,
            bottom: self.bottom * rhs.bottom,
        }
        .simplify()
    }
}

impl Div for PolynomialFraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top * rhs.bottom,
            bottom: self.bottom * rhs.top,
        }
        .simplify()
    }
}
