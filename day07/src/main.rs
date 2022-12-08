use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Extract all weights within a folder
fn extract_weights(directory: &str, directories_and_weights: &HashMap<String, Vec<String>>) -> i32 {
    let mut weight = 0;

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

    // Store links of child/parent: for each directory, we must know its parent directory
    let mut child_parent_link = HashMap::<String, String>::new();

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut current_dir = "".to_string();

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
                    current_dir = child_parent_link.get(&current_dir).unwrap().to_string();
                } else {
                    let vec_directory = line.split("cd ").collect::<Vec<&str>>();
                    let child = vec_directory[1].to_string();
                    if !child_parent_link.contains_key(&child) {
                        child_parent_link.insert(child.to_string(), current_dir.clone());
                    }
                    current_dir = child;
                    directories_and_weights.insert(current_dir.clone(), Vec::new());
                }
            } else if line == "$ ls" {
            } else {
                panic!("Error with command in line: {line}");
            }
        } else if !line.is_empty() {
            let vec_directory = line.split(" ").collect::<Vec<&str>>();
            if line[0..4] == "dir ".to_string() {
                let directory = vec_directory[1].to_string();
                let current_compo = directories_and_weights.get_mut(&current_dir).unwrap();
                current_compo.push(directory);
            } else {
                let directory = vec_directory[0].to_string();
                let current_compo = directories_and_weights.get_mut(&current_dir).unwrap();
                current_compo.push(directory);
            }
        }
    }

    println!("directories_and_weights: {:?}", directories_and_weights);
    println!("child_parent_link: {:?}", child_parent_link);

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
