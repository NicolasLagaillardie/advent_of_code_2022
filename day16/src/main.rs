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
) -> HashMap<String, HashMap<String, (f64, Vec<String>)>> {
    let mut result = HashMap::<String, HashMap<String, (f64, Vec<String>)>>::new();

    for (starting_valve_name, starting_valve) in valves.clone().iter() {
        let mut temp_result = HashMap::<String, (f64, Vec<String>)>::new();

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

                    temp_result.insert(
                        ending_valve_name.clone(),
                        (ending_valve.flow_rate as f64 / temp.len() as f64, temp),
                    );
                }
            }
        }

        result.insert(starting_valve_name.to_string(), temp_result);
    }

    result
}

fn build_best_path(
    mut current_valve_name: String,
    valves: &HashMap<String, Valve>,
    paths: &HashMap<String, HashMap<String, (f64, Vec<String>)>>,
) -> Vec<String> {
    let mut result = vec![current_valve_name.to_string()];

    loop {
        // Get all available valves
        // With respect to already opened valves
        let mut available_valves = Vec::new();

        for (name, _valve) in valves.iter() {
            if !result.contains(name) {
                available_valves.push(name.to_string());
            }
        }

        let tested_paths = paths.get(&current_valve_name).unwrap();

        // Get maximum pressure
        // With respect to distance
        let mut pressure = (0.0, Vec::new());
        let mut target_valve = "";

        for valve in available_valves.iter() {
            if let Some(tested_path) = tested_paths.get(valve) {
                if tested_path.0 > pressure.0 {
                    pressure.0 = tested_path.0;
                    pressure.1 = tested_path.1.clone();
                    target_valve = valve;
                }
            }
        }

        // If all closed valve have a flow rate of 0
        if pressure.0 == 0.0 {
            break;
        } else {
            current_valve_name = target_valve.to_string();

            result.push(target_valve.to_string());
        }
    }

    result
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

    let mut starting_valve = "AA".to_string();

    let built_paths = build_paths(&valves);

    println!("built_paths done");

    let result_vec = build_best_path(starting_valve.clone(), &valves, &built_paths);

    println!("build_best_path done");

    println!("result_vec: {:?}", result_vec);

    // println!(
    //     "DD to BB: {:?}",
    //     built_paths.get("DD").unwrap().get("BB").unwrap()
    // );
    // println!(
    //     "DD to JJ: {:?}",
    //     built_paths.get("DD").unwrap().get("JJ").unwrap()
    // );

    // println!(
    //     "AA to BB: {:?}",
    //     built_paths.get("AA").unwrap().get("BB").unwrap()
    // );
    // println!(
    //     "AA to DD: {:?}",
    //     built_paths.get("AA").unwrap().get("DD").unwrap()
    // );

    let mut result = 0;

    let mut elapsed_time = 30;

    // Build back the path
    for elt in result_vec[1..].iter() {
        let paths_starting_valve = built_paths.get(&starting_valve).unwrap();

        let path = &paths_starting_valve.get(elt).unwrap().1;

        let pressure = valves.get(elt).unwrap().flow_rate;

        elapsed_time = elapsed_time - path.len() as i32;

        if elapsed_time >= 0 {
            result += elapsed_time * pressure;

            starting_valve = elt.to_string();
        } else {
            return result;
        }
    }

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
