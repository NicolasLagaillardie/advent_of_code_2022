use core::panic;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut result = 0;

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let instruction = line.split(' ').collect::<Vec<&str>>();

        let direction = instruction[0];
        let steps = instruction[1].parse::<i32>().unwrap();
        

    }

    result
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut result = 0

    // Stores explored cells
    let mut node_09_explored_cells = vec![node_09_cell];

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let instruction = line.split(' ').collect::<Vec<&str>>();

        let direction = instruction[0];
        let steps = instruction[1].parse::<i32>().unwrap();
    }

    result
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
        assert_eq!(aux_one(Path::new("input/test01.txt")), 13);
        assert_eq!(aux_two(Path::new("input/test02.txt")), 36);
    }
}
