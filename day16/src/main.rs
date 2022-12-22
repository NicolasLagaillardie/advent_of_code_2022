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
) -> HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)> {
    let mut result =
        HashMap::<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>::new();

    for (starting_valve_name, starting_valve) in valves.clone().iter() {
        let mut temp_result = HashMap::<String, (Vec<String>, f64)>::new();
        let mut valid_connected_valves = Vec::new();

        let mut temp_sum_pressure = 0.0;

        for (ending_valve_name, ending_valve) in valves.clone().iter() {
            // If starting and ending valve are different and we do not aim for an insignificant valve
            if starting_valve_name != ending_valve_name && ending_valve.flow_rate > 0 {
                // If start from AA or we start from a significant valve
                if starting_valve_name == &"AA".to_string() || starting_valve.flow_rate > 0 {
                    // from starting_valve_name to ending_valve_name
                    let temp_shorted_path = aux_build_paths(
                        starting_valve_name.clone(),
                        ending_valve_name.clone(),
                        valves,
                    );

                    let temp_normalised_pressure =
                        ending_valve.flow_rate as f64 / temp_shorted_path.len() as f64;

                    temp_sum_pressure += temp_normalised_pressure;

                    temp_result.insert(
                        ending_valve_name.clone(),
                        (temp_shorted_path, temp_normalised_pressure),
                    );
                    valid_connected_valves.push(ending_valve_name.clone());
                }

                if (!ending_valve.connected_valves.contains(starting_valve_name)
                    && starting_valve.connected_valves.contains(ending_valve_name))
                    || (ending_valve.connected_valves.contains(starting_valve_name)
                        && !starting_valve.connected_valves.contains(ending_valve_name))
                {
                    panic!("Error with {ending_valve_name} / {starting_valve_name}");
                }
            }
        }

        if starting_valve_name == &"AA".to_string() || starting_valve.flow_rate > 0 {
            result.insert(
                starting_valve_name.to_string(),
                (valid_connected_valves, temp_result, temp_sum_pressure),
            );
        }
    }

    result
}

// Compute released pressure from given path
fn compute_released_pressure(
    valves: &HashMap<String, Valve>,
    matrix_paths: &HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>,
    tested_path: &Vec<String>,
    mut elapsed_time: i32,
) -> i32 {
    let mut result = 0;

    for (index, name_end_valve) in tested_path[1..tested_path.len()].iter().enumerate() {
        let name_start_valve = tested_path[index].to_string();
        let (_, paths_from_start_to_all, _) = matrix_paths.get(&name_start_valve).unwrap();
        let (shortest_path, _) = paths_from_start_to_all.get(name_end_valve).unwrap();
        let pressure_end_valve = valves.get(name_end_valve).unwrap().flow_rate;

        if elapsed_time - shortest_path.len() as i32 > 0 {
            result += (elapsed_time - shortest_path.len() as i32) * pressure_end_valve;
            elapsed_time -= shortest_path.len() as i32;
        } else {
            return result;
        }
    }

    result
}

fn get_permutations(list: &mut Vec<String>, k: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    if k == 1 {
        return vec![list.to_vec()];
    }

    for i in 0..k {
        let mut temp = get_permutations(list, k - 1);

        if k % 2 == 1 {
            list.swap(0, k - 1);
        } else {
            list.swap(i, k - 1);
        }

        result.append(&mut temp);
    }

    result
}

fn build_best_path_part_one(
    starting_valve_name: String,
    valves: &HashMap<String, Valve>,
    matrix_paths: &HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>,
    elapsed_time: i32,
) -> i32 {
    let mut final_path = vec![starting_valve_name];

    while final_path.len() != matrix_paths.len() {
        let mut heaviest_normalised_pressure = 0.0;
        let mut next_valve = "".to_string();

        let name_start_valve = final_path[final_path.len() - 1].clone();
        let other_valves = &matrix_paths.get(&name_start_valve).unwrap().1;

        for (name_end_valve, info_end_valve) in other_valves.iter() {
            if !final_path.contains(name_end_valve)
                && info_end_valve.1 > heaviest_normalised_pressure
            {
                heaviest_normalised_pressure = info_end_valve.1;
                next_valve = name_end_valve.to_string();
            }
        }

        final_path.push(next_valve.clone());
    }

    let mut max_pressure =
        compute_released_pressure(valves, matrix_paths, &final_path, elapsed_time);

    let mut target_k = 5;

    let mut modified = true;

    while modified {
        modified = false;

        for index in 0..final_path.len() - target_k {
            let head = final_path[0..index + 1].to_vec();
            let tail = final_path[index + 1..final_path.len()].to_vec();

            let permutations = get_permutations(&mut tail.clone(), target_k);

            for mut permutation in permutations {
                let mut attempt = head.clone();
                attempt.append(&mut permutation);
                let pressure_attempt =
                    compute_released_pressure(valves, matrix_paths, &attempt, elapsed_time);

                if pressure_attempt > max_pressure {
                    max_pressure = pressure_attempt;
                    final_path = attempt.clone();
                    modified = true;
                }
            }
        }

        target_k += 1;
    }

    println!("Final path : {final_path:?}");

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

    println!("built_paths done:");

    for (k, v) in built_paths.iter() {
        println!("{:?} : {:?}", k, v);
        println!("");
    }

    build_best_path_part_one(starting_valve, &valves, &built_paths, 30)
}

fn build_best_path_part_two(
    starting_valve_name: String,
    valves: &HashMap<String, Valve>,
    paths: &HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>,
    elapsed_time: i32,
) -> i32 {
    let mut tested_paths = Vec::new();

    let mut max_pressure = 0;

    // First, build all starting path
    for (name_valve, valve) in valves.iter() {
        if valve.flow_rate > 0 {
            // Get the path from the current valve to the connected name_valve
            let path_starting_valve_to_valve = paths
                .get(&starting_valve_name)
                .unwrap()
                .1
                .get(name_valve)
                .unwrap();

            let new_elapsed_time = elapsed_time - path_starting_valve_to_valve.0.len() as i32;

            tested_paths.push((
                vec![starting_valve_name.clone(), name_valve.to_string()], // New path me
                vec![starting_valve_name.clone()],                         // New path elephant
                new_elapsed_time * valve.flow_rate,                        // Released pressure
                new_elapsed_time,                                          // Elapsed time me
                elapsed_time,                                              // Elapsed time elephant
            ));

            if new_elapsed_time * valve.flow_rate > max_pressure {
                max_pressure = new_elapsed_time * valve.flow_rate;
            }
        }
    }

    let mut modified = true;

    let mut final_path = (Vec::new(), Vec::new());

    while modified {
        modified = false;

        let mut next_tested_paths = Vec::new();

        // For each paths to test
        for (
            tested_path_me,
            tested_path_elephant,
            released_pressure_tested_path,
            elapsed_time_tested_path_me,
            elapsed_time_tested_path_elephant,
        ) in tested_paths.iter()
        {
            // Last opened valve = starting valve
            let name_start_valve_me = tested_path_me[tested_path_me.len() - 1].to_string();
            let name_start_valve_elephant =
                tested_path_elephant[tested_path_elephant.len() - 1].to_string();

            // For each other valves with positive pressure
            for (name_end_valve, end_valve) in valves.iter() {
                if end_valve.flow_rate > 0
                    && !tested_path_me.contains(name_end_valve)
                    && !tested_path_elephant.contains(name_end_valve)
                {
                    // Get the path from the current valve to the connected name_valve
                    let path_starting_valve_to_valve = paths
                        .get(&name_start_valve_me)
                        .unwrap()
                        .1
                        .get(name_end_valve)
                        .unwrap();

                    let new_elapsed_time_tested_path_me =
                        elapsed_time_tested_path_me - path_starting_valve_to_valve.0.len() as i32;

                    // If adding this new valve is not too far w.r.t the remaing time
                    if new_elapsed_time_tested_path_me > 0 {
                        let mut temp_tested_path_me = tested_path_me.clone();
                        temp_tested_path_me.push(name_end_valve.clone());

                        let new_released_pressure = released_pressure_tested_path
                            + new_elapsed_time_tested_path_me * end_valve.flow_rate;

                        next_tested_paths.push((
                            temp_tested_path_me.clone(),
                            tested_path_elephant.clone(),
                            released_pressure_tested_path
                                + new_elapsed_time_tested_path_me * end_valve.flow_rate,
                            new_elapsed_time_tested_path_me,
                            *elapsed_time_tested_path_elephant,
                        ));

                        if new_released_pressure > max_pressure {
                            max_pressure = new_released_pressure;
                            final_path.0 = temp_tested_path_me.clone();
                        }

                        modified = true;
                    }

                    // Get the path from the current valve to the connected name_valve
                    let path_starting_valve_to_valve = paths
                        .get(&name_start_valve_elephant)
                        .unwrap()
                        .1
                        .get(name_end_valve)
                        .unwrap();

                    let new_elapsed_time_tested_path_elephant = elapsed_time_tested_path_elephant
                        - path_starting_valve_to_valve.0.len() as i32;

                    // If adding this new valve is not too far w.r.t the remaing time
                    if new_elapsed_time_tested_path_elephant > 0 {
                        let mut temp_tested_path_elephant = tested_path_elephant.clone();
                        temp_tested_path_elephant.push(name_end_valve.clone());

                        let new_released_pressure = released_pressure_tested_path
                            + new_elapsed_time_tested_path_elephant * end_valve.flow_rate;

                        next_tested_paths.push((
                            tested_path_me.clone(),
                            temp_tested_path_elephant.clone(),
                            released_pressure_tested_path
                                + new_elapsed_time_tested_path_elephant * end_valve.flow_rate,
                            *elapsed_time_tested_path_me,
                            new_elapsed_time_tested_path_elephant,
                        ));

                        if new_released_pressure > max_pressure {
                            max_pressure = new_released_pressure;
                            final_path.1 = temp_tested_path_elephant.clone();
                        }

                        modified = true;
                    }
                }
            }
        }

        tested_paths = next_tested_paths;
    }

    max_pressure
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

    build_best_path_part_two(starting_valve, &valves, &built_paths, 26)
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
    }
}

// 2580
