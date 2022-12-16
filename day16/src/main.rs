use core::panic;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    connected_valves: Vec<String>,
}

impl Valve {
    fn new() -> Self {
        Valve {
            name: "".to_string(),
            flow_rate: 0,
            connected_valves: Vec::new(),
        }
    }
}

impl Clone for Valve {
    fn clone(&self) -> Self {
        Valve {
            name: self.name.clone(),
            flow_rate: self.flow_rate.clone(),
            connected_valves: self.connected_valves.clone(),
        }
    }
}

/// Explore valves
fn explore_valves(
    current_valve: &Valve,
    valves: &HashMap<String, Valve>,
    valves_with_pressure: &HashMap<String, Valve>,
    elapsed_time: i32,
    explored_paths: &Vec<(Vec<String>, i32)>,
    opened_valves: &Vec<String>,
    explored_valves: &Vec<String>,
    max_pressure: i32,
) -> Vec<(Vec<String>, i32)> {
    print!("{elapsed_time} / ");

    if elapsed_time >= 10 || opened_valves.len() == valves_with_pressure.len() {
        return explored_paths.to_vec();
    }

    let mut temp_explored_paths = Vec::new();

    // For each already explored path
    for cell in explored_paths.clone().iter() {
        let (path, pressure) = cell;

        // For each path to explore
        for next_valve_string in current_valve.clone().connected_valves.iter() {
            if opened_valves.contains(next_valve_string) {
                // Move to an opened valve
                if elapsed_time + 1 <= 10 {
                    let mut temp_path = path.clone();
                    temp_path.push(next_valve_string.to_string());

                    let next_valve = valves.get(next_valve_string).unwrap();

                    let mut temp_explored_valves = explored_valves.clone();
                    if !temp_explored_valves.contains(next_valve_string) {
                        temp_explored_valves.push(next_valve_string.to_string());
                    }

                    temp_explored_paths.append(&mut explore_valves(
                        next_valve,
                        valves,
                        valves_with_pressure,
                        elapsed_time + 1,
                        &vec![(temp_path.clone(), *pressure)],
                        opened_valves,
                        &temp_explored_valves,
                        max(max_pressure, *pressure),
                    ));
                }
            } else {
                // Move to a closed valve but don't open it
                if elapsed_time + 1 <= 10 {
                    let mut temp_path = path.clone();
                    temp_path.push(next_valve_string.to_string());

                    let next_valve = valves.get(next_valve_string).unwrap();

                    let mut temp_explored_valves = explored_valves.clone();
                    if !temp_explored_valves.contains(next_valve_string) {
                        temp_explored_valves.push(next_valve_string.to_string());
                    }

                    temp_explored_paths.append(&mut explore_valves(
                        next_valve,
                        valves,
                        valves_with_pressure,
                        elapsed_time + 1,
                        &vec![(temp_path.clone(), *pressure)],
                        opened_valves,
                        &temp_explored_valves,
                        max_pressure,
                    ));
                }

                // Move to a closed valve and open it
                if elapsed_time + 2 <= 10 {
                    let mut temp_path = path.clone();
                    temp_path.push(next_valve_string.to_string());

                    let next_valve = valves.get(next_valve_string).unwrap();

                    // If valve has impact on total pressure
                    if next_valve.flow_rate != 0 {
                        let mut temp_opened_valves = opened_valves.clone();
                        temp_opened_valves.push(next_valve_string.clone());

                        let mut temp_explored_valves = explored_valves.clone();
                        if !temp_explored_valves.contains(next_valve_string) {
                            temp_explored_valves.push(next_valve_string.to_string());
                        }

                        temp_explored_paths.append(&mut explore_valves(
                            next_valve,
                            valves,
                            valves_with_pressure,
                            elapsed_time + 2,
                            &vec![(
                                temp_path.clone(),
                                *pressure + (10 - elapsed_time) * current_valve.flow_rate,
                            )],
                            &temp_opened_valves,
                            &temp_explored_valves,
                            max(
                                max_pressure,
                                *pressure + (10 - elapsed_time) * current_valve.flow_rate,
                            ),
                        ));
                    }
                }
            }
        }
    }

    temp_explored_paths
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut valves = HashMap::<String, Valve>::new();
    let mut valves_with_pressure = HashMap::<String, Valve>::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let line = line.split("Valve ").collect::<Vec<&str>>()[1];

        let line = line.split(" has flow rate=").collect::<Vec<&str>>();
        let name = line[0].to_string();

        let line = if line[1].contains("; tunnel leads to valve ") {
            line[1]
                .split("; tunnel leads to valve ")
                .collect::<Vec<&str>>()
        } else if line[1].contains("; tunnels lead to valves ") {
            line[1]
                .split("; tunnels lead to valves ")
                .collect::<Vec<&str>>()
        } else {
            panic!("Error with line: {:?}", line);
        };

        let flow_rate = line[0].parse::<i32>().unwrap();

        let connected_valves = line[1].split(",").collect::<Vec<&str>>();
        let connected_valves = connected_valves
            .iter()
            .map(|elt| elt.to_string().trim().to_string())
            .collect::<Vec<String>>();

        let mut valve = Valve::new();
        valve.name = name.clone();
        valve.flow_rate = flow_rate;
        valve.connected_valves = connected_valves;

        if valve.flow_rate != 0 {
            valves_with_pressure.insert(name.clone(), valve.clone());
        }

        valves.insert(name, valve);
    }

    println!("Valves: {:?}", valves);

    let starting_valve = valves.get("AA").unwrap();

    let result_vec = explore_valves(
        starting_valve,
        &valves,
        &valves_with_pressure,
        0,
        &vec![(vec![starting_valve.name.clone()], 0)],
        &Vec::new(),
        &Vec::new(),
        0,
    );

    let mut result = 0;
    let mut test = Vec::new();

    for elt in result_vec.iter() {
        if elt.1 > result {
            test = elt.0.clone();
            result = elt.1;
        }
    }

    println!("test: {:?}", test);

    result
}

/// Function for part 02
fn aux_two(_file: &Path) -> i32 {
    0
}

/// Main function
fn main() {
    println!("Enter path to file");

    // Ask input of path
    let mut path = String::new();

    stdin().read_line(&mut path).expect("Failed to read input");

    // Remove end of input containing \n
    let path = path.trim();

    let file = Path::new(path);

    // Ask input of part
    println!("Which part do you want? (1/2)");

    let mut choice = String::new();

    stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let result = match choice.trim() {
        "1" => aux_one(file),
        "2" => aux_two(file),
        _ => panic!("Error, expecting 1 or 2"),
    };

    // Display total score
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(aux_one(Path::new("input/test.txt")), 1651);
        // assert_eq!(aux_two(Path::new("input/test.txt"), true), 56000011);
    }
}
