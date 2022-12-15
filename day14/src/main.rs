use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> u64 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Store composition of every monkey
    // Starting items = Vec<String>
    // Operation = String
    // (Test, True, False) = (String, String, String)
    let mut monkeys = HashMap::<u64, Monkey>::new();

    // Stores current parsed monkey
    let mut current_monkey = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
    }

    0
}

/// Function for part 02
fn aux_two(file: &Path) -> u64 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

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
        // assert_eq!(aux_one(Path::new("input/test.txt")), 10605);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 2713310158);
    }
}
