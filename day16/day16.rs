use std::collections::{HashMap, VecDeque};

const ALONE_TIME: usize = 31;
const ELEPHANT_TIME: usize = 27;

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = std::fs::read_to_string(filename).unwrap();

    let valves: Vec<Valve> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Valve::from_description(line))
            }
        })
        .collect();

    let valve_map: HashMap<ValveName, Valve> = valves
        .iter()
        .map(|valve| valve.name.clone())
        .zip(valves.iter().cloned())
        .collect();

    // print_graphviz(&valve_map);

    let simplified_valve_map = simplify_graph(&valve_map, &valve_name_from_str("AA"));

    let mut simplified_valve_vec = vec![None; 26 * 26];

    for (name, valve) in simplified_valve_map {
        simplified_valve_vec[name] = Some(valve.clone());
    }

    // print_graphviz(&simplifier_valve_map);

    println!("Part 1:");
    let best_score = traverse(
        &simplified_valve_vec,
        &valve_name_from_str("AA"),
        ALONE_TIME,
        false,
    );
    println!("{}", best_score);

    println!("Part 2:");
    let best_score = traverse(
        &simplified_valve_vec,
        &valve_name_from_str("AA"),
        ELEPHANT_TIME,
        true,
    );
    println!("{}", best_score);
}

type ValveName = usize;

fn valve_name_from_str(s: &str) -> ValveName {
    (s.chars().next().unwrap() as u8 - 'A' as u8) as usize
        + ((s.chars().skip(1).next().unwrap() as u8 - 'A' as u8) as usize) * 26
}

#[derive(Debug, Clone)]
struct Valve {
    name: ValveName,
    flow_rate: usize,
    tunnels: Vec<(usize, ValveName)>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.flow_rate == other.flow_rate
            && self.tunnels == other.tunnels
    }
}

impl Eq for Valve {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Valve {
    fn from_description(s: &str) -> Valve {
        let name = valve_name_from_str(&s[6..8]);

        let tunnels = s
            .split_once("; ")
            .unwrap()
            .1
            .split(' ')
            .skip(4)
            .map(|name| {
                (
                    1,
                    valve_name_from_str(if name.ends_with(',') {
                        &name[..(name.len() - 1)]
                    } else {
                        name
                    }),
                )
            })
            .collect();

        let flow_rate = s
            .split_once(';')
            .unwrap()
            .0
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        Valve {
            name,
            flow_rate,
            tunnels,
        }
    }
}

type Graph = Vec<Option<Valve>>;

fn traverse(
    graph: &Graph,
    starting_valve: &ValveName,
    time_to_go: usize,
    has_elephant: bool,
) -> usize {
    let mut visited = vec![false; 26 * 26];
    visited[*starting_valve as usize] = true;

    fn trav(
        graph: &Graph,
        visited: &mut Vec<bool>,
        visited_count: usize,
        current_value: &ValveName,
        time_to_go: usize,
        has_elephant: bool,
    ) -> usize {
        let valve = graph[*current_value].as_ref().unwrap();

        let mut score = 0;

        if visited_count > graph.len() / 2 && time_to_go > 0 {
            score = score.max(valve.flow_rate * (time_to_go - 1));
        }

        if has_elephant {
            score += trav(
                graph,
                visited,
                0,
                &valve_name_from_str("AA"),
                ELEPHANT_TIME,
                false,
            );
        }

        // still has time to go somewhere else
        for (dist, next_valve) in &valve.tunnels {
            if visited[*next_valve as usize] || dist + 1 > time_to_go {
                continue;
            }

            visited[(*next_valve as usize)] = true;
            score = score.max(
                (valve.flow_rate * (time_to_go - 1))
                    + trav(
                        graph,
                        visited,
                        visited_count + 1,
                        &next_valve,
                        time_to_go - dist - 1,
                        has_elephant,
                    ),
            );
            visited[(*next_valve as usize)] = false;
        }

        score
    }

    trav(
        graph,
        &mut visited,
        0,
        starting_valve,
        time_to_go,
        has_elephant,
    )
}

#[allow(dead_code)]
fn print_graphviz(valve_map: &HashMap<ValveName, Valve>) {
    println!("digraph {{");

    for (name, valve) in valve_map {
        println!(
            "  {}{} [xlabel={}];",
            (name % 256) as u8 as char,
            (name / 256) as u8 as char,
            valve.flow_rate
        );
        for (dist, next_name) in &valve.tunnels {
            println!(
                "  {}{} -> {}{} [label={}];",
                (name % 256) as u8 as char,
                (name / 256) as u8 as char,
                (next_name % 256) as u8 as char,
                (next_name / 256) as u8 as char,
                dist
            );
        }
    }

    println!("}}");
}

fn simplify_graph(
    valve_map: &HashMap<ValveName, Valve>,
    starting_valve: &ValveName,
) -> HashMap<ValveName, Valve> {
    fn calculate_distances(
        valve_map: &HashMap<ValveName, Valve>,
        starting_valve: &ValveName,
    ) -> Vec<(usize, ValveName)> {
        let mut queue: VecDeque<(usize, ValveName)> = VecDeque::new();
        let mut distances: HashMap<ValveName, usize> = HashMap::new();

        queue.push_back((0usize, starting_valve.clone()));

        while !queue.is_empty() {
            let (current_dist, current) = queue.pop_front().unwrap();

            let current = &valve_map[&current];

            for (dist, next) in &current.tunnels {
                if !distances.contains_key(&next) {
                    distances.insert(next.clone(), current_dist + dist);
                    queue.push_back((current_dist + dist, next.clone()));
                }
            }
        }

        distances
            .iter()
            .filter(|(name, _)| valve_map[&name].flow_rate != 0)
            .map(|(name, dist)| (*dist, name.clone()))
            .collect()
    }

    valve_map
        .iter()
        .filter(|(name, valve)| valve.flow_rate != 0 || *name == starting_valve)
        .map(|(name, valve)| {
            (
                name.clone(),
                Valve {
                    name: name.clone(),
                    flow_rate: valve.flow_rate,
                    tunnels: calculate_distances(valve_map, name),
                },
            )
        })
        .collect()
}
