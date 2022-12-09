use core::panic;
use std::cmp::max;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Number of visible trees
    let mut result = 0;

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line
            .unwrap()
    }

   result
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line.unwrap();
    }

    // Number of visible trees
    let mut result = 0;

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
        assert_eq!(aux_one(Path::new("input/test.txt")), 13);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 8);
    }
}
