use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Extract all weights within a folder
fn extract_weights(directory: &str, directories_and_weights: &HashMap<String, Vec<String>>) -> i32 {
    let mut weight = 0;

    println!("directory: {directory}");

    println!(
        "directories_and_weights.get(directory): {:?}",
        directories_and_weights.get(directory)
    );

    for elt in directories_and_weights.get(directory).unwrap() {
        match elt.parse::<i32>() {
            Ok(int) => {
                weight += int;
            }
            Err(_) => {
                weight += extract_weights(elt, directories_and_weights);
            }
        }
    }

    weight
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Store composition of every folder
    let mut directories_and_weights = HashMap::<String, Vec<String>>::new();

    // Stores how many time we explored each path
    let mut explored_paths = HashMap::<String, i32>::new();

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut current_path = "";

    let mut temp_string;

    // Read file line by line, for part 01
    // We get the composition of every explored directory
    for (_index_line, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim().to_string();

        // If we have a command
        if line.contains("$ ") {
            // If we move from one level to another
            if line.contains(" cd ") {
                // If we go back to parent or to child
                if line.contains(" ..") {
                    let mut temp_vec = current_path.split("/").collect::<Vec<&str>>();
                    temp_vec.pop();
                    temp_string = temp_vec.join("/");
                    if temp_string.is_empty() {
                        temp_string = "/".to_string();
                    }
                    current_path = &temp_string;
                } else {
                    // Retrieve child directory
                    let vec_directory = line.split("cd ").collect::<Vec<&str>>();
                    let child = vec_directory[1].to_string();

                    // Assign new absolute path as current one
                    if child == "/".to_string() {
                        current_path = "/";
                    } else if current_path == "/" {
                        temp_string = format!("/{child}");
                        current_path = &temp_string;
                    } else {
                        // Create new absolute path for child directory
                        let temp_vec = current_path.split("/").collect::<Vec<&str>>();
                        temp_string = temp_vec.join("/");

                        temp_string = format!("{temp_string}/{child}");

                        current_path = &temp_string;
                    }

                    directories_and_weights.insert(current_path.to_string(), Vec::new());
                }
            } else if line == "$ ls" {
                match explored_paths.get_mut(current_path) {
                    Some(index) => {
                        *index += 1;
                    }
                    None => {
                        explored_paths.insert(current_path.to_string(), 1);
                    }
                }
            } else {
                panic!("Error with command in line: {line}");
            }
        } else if !line.is_empty() {
            if explored_paths.get(current_path).unwrap() == &1 {
                let vec_directory = line.split(" ").collect::<Vec<&str>>();
                if line[0..4] == "dir ".to_string() {
                    let directory = vec_directory[1].to_string();
                    let current_compo = directories_and_weights.get_mut(current_path).unwrap();

                    if current_path == "/" {
                        current_compo.push(format!("/{directory}"));
                    } else {
                        // Create new absolute path for child directory
                        let temp_vec = current_path.split("/").collect::<Vec<&str>>();
                        let temp = temp_vec.join("/");

                        println!("temp_vec: {:?}", temp_vec);
                        println!("current_path: {:?}", current_path);
                        println!("directory: {:?}", directory);
                        println!("new directory: {:?}", format!("{temp}/{directory}"));

                        current_compo.push(format!("{temp}/{directory}"));
                    }
                } else {
                    let current_compo = directories_and_weights.get_mut(current_path).unwrap();

                    // Weight of integers
                    let directory = vec_directory[0].to_string();
                    current_compo.push(directory);
                }
            }
        }
    }

    println!("directories_and_weights: {:?}", directories_and_weights);

    // Extract all the actual weights of each directory
    let mut final_weights = HashMap::<String, i32>::new();

    for (key, _value) in &directories_and_weights {
        final_weights.insert(
            key.to_string(),
            extract_weights(&key, &directories_and_weights),
        );
    }

    println!("final_weights: {:?}", final_weights);

    // Sum all weights of at most 100000
    let mut result = 0;

    for (_key, value) in final_weights {
        if value < 100000 {
            result += value;
        }
    }

    result
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 95437);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 19);
    }
}
