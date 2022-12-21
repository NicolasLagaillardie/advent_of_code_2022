use core::panic;
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

// Recursive function for build_paths
fn aux_build_paths(
    starting_valve_name: String,
    ending_valve_name: String,
    valves: &HashMap<String, Valve>,
) -> Vec<String> {
    let mut paths = vec![vec![starting_valve_name.clone()]];

    // All nodes are connected in our case
    loop {
        let mut temp_result = Vec::new();

        for path in paths.clone().iter() {
            let connected_valves = &valves.get(&path[path.len() - 1]).unwrap().connected_valves;

            for conneected_valve in connected_valves.iter() {
                let mut temp_path = path.clone();

                if !path.contains(conneected_valve) {
                    temp_path.push(conneected_valve.to_string());
                }

                if temp_path.contains(&ending_valve_name) {
                    return temp_path;
                }

                temp_result.push(temp_path);
            }
        }

        paths = temp_result;
    }
}

// Build paths from each node to each other nodes
fn build_paths(
    valves: &HashMap<String, Valve>,
) -> HashMap<String, (Vec<String>, HashMap<String, Vec<String>>)> {
    let mut result = HashMap::<String, (Vec<String>, HashMap<String, Vec<String>>)>::new();

    for (starting_valve_name, starting_valve) in valves.clone().iter() {
        let mut temp_result = HashMap::<String, Vec<String>>::new();
        let mut valid_connected_valves = Vec::new();

        for (ending_valve_name, ending_valve) in valves.clone().iter() {
            // If starting and ending valve are different and we do not aim for an insignificant valve
            if starting_valve_name != ending_valve_name && ending_valve.flow_rate != 0 {
                // If start from AA or we start from a significant valve
                if starting_valve_name == &"AA".to_string() || starting_valve.flow_rate != 0 {
                    let temp = aux_build_paths(
                        starting_valve_name.clone(),
                        ending_valve_name.clone(),
                        valves,
                    );

                    temp_result.insert(ending_valve_name.clone(), temp);
                    valid_connected_valves.push(ending_valve_name.clone());
                }
            }
        }

        result.insert(
            starting_valve_name.to_string(),
            (valid_connected_valves, temp_result),
        );
    }

    result
}

fn build_best_path_part_one(
    current_valve_name: String,
    valves: &HashMap<String, Valve>,
    paths: &HashMap<String, (Vec<String>, HashMap<String, Vec<String>>)>,
    mut max_pressure: i32,
    elapsed_time: i32,
    current_path: &Vec<String>,
) -> i32 {
    // Get all closed valves
    let mut available_valves = Vec::new();

    // Removed already explored valves
    for (name_valve, valve) in valves.iter() {
        if !current_path.contains(name_valve) && valve.flow_rate != 0 {
            available_valves.push(name_valve.to_string());
        }
    }

    let save_max_pressure = max_pressure.clone();

    // Build paths
    for name_valve in available_valves.iter() {
        // Get the path from the current valve to the connected name_valve
        let path_starting_valve_to_valve = paths
            .get(&current_valve_name)
            .unwrap()
            .1
            .get(name_valve)
            .unwrap();

        // If within bounds of time limits
        if elapsed_time - path_starting_valve_to_valve.len() as i32 > 0 {
            let mut temp_result = current_path.clone();
            temp_result.push(name_valve.to_string());

            let valve = valves.get(name_valve).unwrap();

            let temp = build_best_path_part_one(
                name_valve.to_string(),
                valves,
                paths,
                save_max_pressure
                    + ((elapsed_time - path_starting_valve_to_valve.len() as i32)
                        * valve.flow_rate),
                elapsed_time - path_starting_valve_to_valve.len() as i32,
                &temp_result,
            );

            if temp > max_pressure {
                max_pressure = temp;
            }
        }
    }

    max_pressure
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

        let connected_valves = line[1].split(',').collect::<Vec<&str>>();
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

    let starting_valve = "AA".to_string();

    let built_paths = build_paths(&valves);

    println!("built_paths done");

    build_best_path_part_one(
        starting_valve.clone(),
        &valves,
        &built_paths,
        0,
        30,
        &vec![starting_valve.clone()],
    )
}

fn build_best_path_part_two(
    current_valve_name_me: String,
    current_valve_name_elephant: String,
    valves: &HashMap<String, Valve>,
    paths: &HashMap<String, (Vec<String>, HashMap<String, Vec<String>>)>,
    mut max_pressure: i32,
    elapsed_time: i32,
    remaining_time_blocked_me: i32,
    remaining_time_blocked_elephant: i32,
    current_path: Vec<String>,
    mut attempted_me: Vec<(String, String)>,
    mut attempted_elephant: Vec<(String, String)>,
) -> (i32, Vec<(String, String)>, Vec<(String, String)>) {
    // Get all closed valves
    let mut available_valves = Vec::new();

    // Save max pressure
    let save_max_pressure = max_pressure.clone();

    // Removed already explored valves
    for (name_valve, valve) in valves.iter() {
        if !current_path.contains(name_valve) && valve.flow_rate != 0 {
            available_valves.push(name_valve.to_string());
        }
    }

    // Move one then the other
    for name_valve in available_valves.iter() {
        // If I can move
        if remaining_time_blocked_me == 0 {
            // Get the path from the current valve to the connected name_valve for me
            let path_starting_valve_to_valve_me = paths
                .get(&current_valve_name_me)
                .unwrap()
                .1
                .get(name_valve)
                .unwrap();

            // If within bounds of time limits for me
            if elapsed_time - path_starting_valve_to_valve_me.len() as i32 > 0
                && !attempted_elephant
                    .contains(&(current_valve_name_me.to_string(), name_valve.to_string()))
            {
                attempted_me.push((current_valve_name_me.to_string(), name_valve.to_string()));

                let compute_time_elephant =
                    remaining_time_blocked_elephant - path_starting_valve_to_valve_me.len() as i32;

                let mut temp_result = current_path.clone();
                temp_result.push(name_valve.to_string());

                let valve = valves.get(name_valve).unwrap();

                // If moving to the next valve takes longer than the elephant for moving to its next valve
                if compute_time_elephant < 0 {
                    let temp = build_best_path_part_two(
                        name_valve.to_string(),
                        current_valve_name_elephant.clone(),
                        valves,
                        paths,
                        save_max_pressure
                            + ((elapsed_time - path_starting_valve_to_valve_me.len() as i32)
                                * valve.flow_rate),
                        elapsed_time - remaining_time_blocked_elephant,
                        -compute_time_elephant,
                        0,
                        temp_result.clone(),
                        attempted_me.clone(),
                        attempted_elephant.clone(),
                    );

                    if temp.0 > max_pressure {
                        max_pressure = temp.0;
                    }
                } else {
                    let temp = build_best_path_part_two(
                        name_valve.to_string(),
                        current_valve_name_elephant.clone(),
                        valves,
                        paths,
                        save_max_pressure
                            + ((elapsed_time - path_starting_valve_to_valve_me.len() as i32)
                                * valve.flow_rate),
                        elapsed_time - path_starting_valve_to_valve_me.len() as i32,
                        0,
                        compute_time_elephant,
                        temp_result.clone(),
                        attempted_me.clone(),
                        attempted_elephant.clone(),
                    );

                    if temp.0 > max_pressure {
                        max_pressure = temp.0;
                    }
                }
            }
        } else if remaining_time_blocked_elephant == 0 {
            // If the elephant can move
            // Get the path from the current valve to the connected name_valve for me
            let path_starting_valve_to_valve_elephant = paths
                .get(&current_valve_name_elephant)
                .unwrap()
                .1
                .get(name_valve)
                .unwrap();

            // If within bounds of time limits for elephant
            if elapsed_time - path_starting_valve_to_valve_elephant.len() as i32 > 0
                && !attempted_me.contains(&(
                    current_valve_name_elephant.to_string(),
                    name_valve.to_string(),
                ))
            {
                attempted_elephant.push((
                    current_valve_name_elephant.to_string(),
                    name_valve.to_string(),
                ));

                let compute_time_me =
                    remaining_time_blocked_me - path_starting_valve_to_valve_elephant.len() as i32;

                let mut temp_result = current_path.clone();
                temp_result.push(name_valve.to_string());

                let valve = valves.get(name_valve).unwrap();

                if compute_time_me < 0 {
                    // If moving to the next valve takes longer than me for moving to my next valve
                    let temp = build_best_path_part_two(
                        current_valve_name_me.clone(),
                        name_valve.to_string(),
                        valves,
                        paths,
                        save_max_pressure
                            + ((elapsed_time - path_starting_valve_to_valve_elephant.len() as i32)
                                * valve.flow_rate),
                        elapsed_time - remaining_time_blocked_me,
                        0,
                        -compute_time_me,
                        temp_result.clone(),
                        attempted_me.clone(),
                        attempted_elephant.clone(),
                    );

                    if temp.0 > max_pressure {
                        max_pressure = temp.0;
                    }
                } else {
                    let temp = build_best_path_part_two(
                        current_valve_name_me.clone(),
                        name_valve.to_string(),
                        valves,
                        paths,
                        save_max_pressure
                            + ((elapsed_time - path_starting_valve_to_valve_elephant.len() as i32)
                                * valve.flow_rate),
                        elapsed_time - path_starting_valve_to_valve_elephant.len() as i32,
                        compute_time_me,
                        0,
                        temp_result.clone(),
                        attempted_me.clone(),
                        attempted_elephant.clone(),
                    );

                    if temp.0 > max_pressure {
                        max_pressure = temp.0;
                    }
                }
            }
        }
    }

    (
        max_pressure,
        attempted_me.to_vec(),
        attempted_elephant.to_vec(),
    )
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
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

        let connected_valves = line[1].split(',').collect::<Vec<&str>>();
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

    let starting_valve = "AA".to_string();

    let built_paths = build_paths(&valves);

    println!("built_paths done");

    build_best_path_part_two(
        starting_valve.clone(),
        starting_valve.clone(),
        &valves,
        &built_paths,
        0,
        26,
        0,
        0,
        vec![starting_valve.clone()],
        vec![],
        vec![],
    )
    .0
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
        assert_eq!(aux_two(Path::new("input/test.txt")), 1707);
        assert_eq!(aux_two(Path::new("input/01.txt")), 2580);
    }
}
