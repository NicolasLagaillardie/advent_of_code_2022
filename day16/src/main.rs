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

// Get all permutations of the first k elements of list
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

    build_best_path_part_one(starting_valve, &valves, &built_paths, 30)
}

// Compute released pressure from given path
fn compute_released_pressure_part_two(
    valves: &HashMap<String, Valve>,
    matrix_paths: &HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>,
    tested_path_me: &Vec<String>,
    tested_path_elephant: &Vec<String>,
    mut elapsed_time_me: i32,
    mut elapsed_time_elephant: i32,
) -> i32 {
    let mut result = 0;

    for (index, name_end_valve) in tested_path_me[1..tested_path_me.len()].iter().enumerate() {
        let name_start_valve = tested_path_me[index].to_string();
        let (_, paths_from_start_to_all, _) = matrix_paths.get(&name_start_valve).unwrap();
        let (shortest_path, _) = match paths_from_start_to_all.get(name_end_valve) {
            Some(elt) => elt,
            None => {
                println!("name_start_valve: {name_start_valve}");
                println!("name_end_valve: {name_end_valve}");
                panic!("tested_path_me: {tested_path_me:?}")
            }
        };
        let pressure_end_valve = valves.get(name_end_valve).unwrap().flow_rate;

        if elapsed_time_me - shortest_path.len() as i32 > 0 {
            result += (elapsed_time_me - shortest_path.len() as i32) * pressure_end_valve;
            elapsed_time_me -= shortest_path.len() as i32;
        } else {
            break;
        }
    }

    for (index, name_end_valve) in tested_path_elephant[1..tested_path_elephant.len()]
        .iter()
        .enumerate()
    {
        let name_start_valve = tested_path_elephant[index].to_string();
        let (_, paths_from_start_to_all, _) = matrix_paths.get(&name_start_valve).unwrap();
        let (shortest_path, _) = match paths_from_start_to_all.get(name_end_valve) {
            Some(elt) => elt,
            None => {
                println!("name_start_valve: {name_start_valve}");
                println!("name_end_valve: {name_end_valve}");
                panic!("tested_path_me: {tested_path_me:?}")
            }
        };
        let pressure_end_valve = valves.get(name_end_valve).unwrap().flow_rate;

        if elapsed_time_elephant - shortest_path.len() as i32 > 0 {
            result += (elapsed_time_elephant - shortest_path.len() as i32) * pressure_end_valve;
            elapsed_time_elephant -= shortest_path.len() as i32;
        } else {
            return result;
        }
    }

    result
}

fn build_best_path_part_two(
    starting_valve_name: String,
    valves: &HashMap<String, Valve>,
    matrix_paths: &HashMap<String, (Vec<String>, HashMap<String, (Vec<String>, f64)>, f64)>,
    elapsed_time: i32,
) -> i32 {
    let mut final_path_me = vec![starting_valve_name.clone()];
    let mut final_path_elephant = vec![starting_valve_name];

    let mut is_me = true;

    while final_path_me.len() + final_path_elephant.len() - 1 != matrix_paths.len() {
        let mut heaviest_normalised_pressure = 0.0;
        let mut next_valve = "".to_string();

        let name_start_valve = final_path_me[final_path_me.len() - 1].clone();
        let other_valves = &matrix_paths.get(&name_start_valve).unwrap().1;

        for (name_end_valve, info_end_valve) in other_valves.iter() {
            if !final_path_me.contains(name_end_valve)
                && !final_path_elephant.contains(name_end_valve)
                && info_end_valve.1 > heaviest_normalised_pressure
            {
                heaviest_normalised_pressure = info_end_valve.1;
                next_valve = name_end_valve.to_string();
            }
        }

        if is_me {
            final_path_me.push(next_valve.clone());
        } else {
            final_path_elephant.push(next_valve.clone());
        }

        is_me = !is_me;
    }

    let mut max_pressure = compute_released_pressure_part_two(
        valves,
        matrix_paths,
        &final_path_me,
        &final_path_elephant,
        elapsed_time,
        elapsed_time,
    );

    let mut target_k = 6;

    let mut modified = true;

    println!("Former path me: {final_path_me:?}");
    println!("Former path elephant: {final_path_elephant:?}");

    println!("");

    while modified {
        modified = false;

        let mut big_path = vec!["AA".to_string()];

        if final_path_elephant.len() <= final_path_me.len() {
            for index in 1..final_path_elephant.len() {
                if !big_path.contains(&final_path_me[index]) {
                    big_path.push(final_path_me[index].clone());
                }
                if !big_path.contains(&final_path_elephant[index]) {
                    big_path.push(final_path_elephant[index].clone());
                }
                big_path.push("".to_string());
                big_path.push("".to_string());
            }
            let mut rest_final_path_me = final_path_me[final_path_elephant.len()..].to_vec();
            big_path.append(&mut rest_final_path_me);
        } else {
            for index in 1..final_path_me.len() {
                if !big_path.contains(&final_path_me[index]) {
                    big_path.push(final_path_me[index].clone());
                }
                if !big_path.contains(&final_path_elephant[index]) {
                    big_path.push(final_path_elephant[index].clone());
                }
                big_path.push("".to_string());
                big_path.push("".to_string());
            }
            let mut rest_final_path_elephant = final_path_elephant[final_path_me.len()..].to_vec();
            big_path.append(&mut rest_final_path_elephant);
        }

        for index in 0..big_path.len() - target_k {
            let head_big_path = big_path[0..index + 1].to_vec();
            let tail_big_path = big_path[index + 1..big_path.len()].to_vec();

            let permutations_full = get_permutations(&mut tail_big_path.clone(), target_k);

            for mut permutation_full in permutations_full {
                let mut temp_final_path_me = vec!["AA".to_string()];
                let mut temp_final_path_elephant = vec!["AA".to_string()];

                let mut attempt_big_path = head_big_path.clone();
                attempt_big_path.append(&mut permutation_full);

                for i in 1..attempt_big_path.len() {
                    if i % 2 == 0 {
                        temp_final_path_me.push(attempt_big_path[i].clone());
                    } else {
                        temp_final_path_elephant.push(attempt_big_path[i].clone());
                    }
                }

                temp_final_path_me.retain(|x| x != "");
                temp_final_path_elephant.retain(|x| x != "");

                if temp_final_path_me.len() > target_k + 1
                    && temp_final_path_elephant.len() > target_k + 1
                {
                    // for index_rearrange in 0
                    for index_me in 0..temp_final_path_me.len() - target_k {
                        let head_me = temp_final_path_me[0..index_me + 1].to_vec();
                        let tail_me =
                            temp_final_path_me[index_me + 1..temp_final_path_me.len()].to_vec();

                        let permutations_me = get_permutations(&mut tail_me.clone(), target_k);

                        for mut permutation_me in permutations_me {
                            let mut attempt = head_me.clone();
                            attempt.append(&mut permutation_me);
                            let pressure_attempt = compute_released_pressure_part_two(
                                valves,
                                matrix_paths,
                                &temp_final_path_me,
                                &temp_final_path_elephant,
                                elapsed_time,
                                elapsed_time,
                            );

                            if pressure_attempt > max_pressure {
                                max_pressure = pressure_attempt;
                                final_path_me = attempt.clone();
                                final_path_elephant = temp_final_path_elephant.clone();
                                modified = true;
                            }
                        }
                    }

                    for index_elephant in 0..temp_final_path_elephant.len() - target_k {
                        let head_elephant =
                            temp_final_path_elephant[0..index_elephant + 1].to_vec();
                        let tail_elephant = temp_final_path_elephant
                            [index_elephant + 1..temp_final_path_elephant.len()]
                            .to_vec();

                        let permutations_elephant =
                            get_permutations(&mut tail_elephant.clone(), target_k);

                        for mut permutation_elephant in permutations_elephant {
                            let mut attempt = head_elephant.clone();
                            attempt.append(&mut permutation_elephant);
                            let pressure_attempt = compute_released_pressure_part_two(
                                valves,
                                matrix_paths,
                                &temp_final_path_me,
                                &temp_final_path_elephant,
                                elapsed_time,
                                elapsed_time,
                            );

                            if pressure_attempt > max_pressure {
                                max_pressure = pressure_attempt;
                                final_path_me = temp_final_path_me.clone();
                                final_path_elephant = attempt.clone();
                                modified = true;
                            }
                        }
                    }
                }
            }
        }

        target_k += 1;
    }

    println!("Final path me: {final_path_me:?}");
    println!("Final path elephant: {final_path_elephant:?}");
    println!("Final k: {target_k}");

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

// above 2580
