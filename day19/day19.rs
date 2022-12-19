use std::collections::HashSet;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let blueprints: Vec<Blueprint> = input
        .split('\n')
        .filter_map(|s| {
            if !s.is_empty() {
                Some(Blueprint::from_description(s))
            } else {
                None
            }
        })
        .collect();

    {
        let mut score = 0;
        let handles = blueprints
            .iter()
            .map(|b| {
                let blueprint = b.clone();
                std::thread::spawn(move || (blueprint.index, simulate(&blueprint, 24)))
            })
            .collect::<Vec<_>>();

        for handle in handles {
            let value = handle.join().unwrap();
            // println!("[Part 1]: {:?}", value);
            score += value.0 as usize * value.1 as usize;
        }

        println!("Part 1:");
        println!("{}", score);
    }

    {
        let mut score = 1;
        let handles = blueprints
            .iter()
            .take(3)
            .map(|b| {
                let blueprint = b.clone();
                std::thread::spawn(move || simulate(&blueprint, 32))
            })
            .collect::<Vec<_>>();

        for handle in handles {
            let value = handle.join().unwrap();
            // println!("[Part 2]: {}", value);
            score *= value as usize;
        }

        println!("Part 2:");
        println!("{}", score);
    }
}

#[derive(Debug, Clone)]
enum RobotRecipe {
    OreRobot { ore: u16 },
    ClayRobot { ore: u16 },
    ObsidianRobot { ore: u16, clay: u16 },
    GeodeRobot { ore: u16, obsidian: u16 },
}

#[derive(Debug, Clone)]
struct Blueprint {
    index: u16,
    recipes: Vec<RobotRecipe>,
}

impl Blueprint {
    fn from_description(s: &str) -> Self {
        let words = s.split(' ');

        let index = words.clone().nth(1).unwrap();
        let index = index.strip_suffix(':').unwrap().parse().unwrap();

        let ore_robot = {
            let ore = words.clone().nth(6).unwrap().parse().unwrap();
            RobotRecipe::OreRobot { ore }
        };

        let clay_robot = {
            let ore = words.clone().nth(12).unwrap().parse().unwrap();
            RobotRecipe::ClayRobot { ore }
        };

        let obsidian_robot = {
            let ore = words.clone().nth(18).unwrap().parse().unwrap();
            let clay = words.clone().nth(21).unwrap().parse().unwrap();
            RobotRecipe::ObsidianRobot { ore, clay }
        };

        let geode_robot = {
            let ore = words.clone().nth(27).unwrap().parse().unwrap();
            let obsidian = words.clone().nth(30).unwrap().parse().unwrap();
            RobotRecipe::GeodeRobot { ore, obsidian }
        };

        Self {
            index,
            recipes: vec![ore_robot, clay_robot, obsidian_robot, geode_robot],
        }
    }
}

fn simulate(blueprint: &Blueprint, iterations: usize) -> usize {
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct State {
        ore: u16,
        clay: u16,
        obsidian: u16,
        geode: u16,
        ore_robots: u16,
        clay_robots: u16,
        obsidian_robots: u16,
        geode_robots: u16,
    }

    impl State {
        fn apply_robots(&self) -> Self {
            Self {
                ore: self.ore + self.ore_robots,
                clay: self.clay + self.clay_robots,
                obsidian: self.obsidian + self.obsidian_robots,
                geode: self.geode + self.geode_robots,
                ore_robots: self.ore_robots,
                clay_robots: self.clay_robots,
                obsidian_robots: self.obsidian_robots,
                geode_robots: self.geode_robots,
            }
        }
    }

    let mut states = HashSet::new();

    states.insert(State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    });

    for i in 0..iterations {
        // println!("Simulation {}, Iteraton: {}", blueprint.index, i);
        states = blueprint
            .recipes
            .iter()
            .flat_map(|recipe| {
                use RobotRecipe::*;
                let recipe = recipe.clone();
                states.iter().map(move |state| match recipe {
                    OreRobot { ore } => {
                        let mut new_state = state.apply_robots();
                        if state.ore >= ore {
                            new_state.ore -= ore;
                            new_state.ore_robots += 1;
                        }
                        new_state
                    }

                    ClayRobot { ore } => {
                        let mut new_state = state.apply_robots();
                        if state.ore >= ore {
                            new_state.ore -= ore;
                            new_state.clay_robots += 1;
                        }
                        new_state
                    }

                    ObsidianRobot { ore, clay } => {
                        let mut new_state = state.apply_robots();
                        if state.ore >= ore && state.clay >= clay {
                            new_state.ore -= ore;
                            new_state.clay -= clay;
                            new_state.obsidian_robots += 1;
                        }
                        new_state
                    }

                    GeodeRobot { ore, obsidian } => {
                        let mut new_state = state.apply_robots();
                        if state.ore >= ore && state.obsidian >= obsidian {
                            new_state.ore -= ore;
                            new_state.obsidian -= obsidian;
                            new_state.geode_robots += 1;
                        }
                        new_state
                    }
                })
            })
            .collect();

        if i > iterations / 2 {
            let max_geodes = states.iter().map(|state| state.geode).max().unwrap();
            let treshold = i as u16 * max_geodes / (iterations + 1) as u16;

            states = states
                .iter()
                .filter(|state| state.geode >= treshold)
                .cloned()
                .collect();
        }
    }

    states.iter().map(|state| state.geode).max().unwrap() as usize
}
