use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

// Get all the stacks, one by one
fn get_stacks() -> Vec<char> {
    
}

/// Function for part 01
fn aux_one(file: &Path) -> &str {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Number of overlaps
    let mut overlaps = 0;

    // Read file line by line, for part 01
    for (_index, line) in reader.lines().enumerate() {
        // Sum scores
        match line.unwrap().trim().parse::<String>() {
            Ok(elt) => {
                let elves = elt.split(",").collect::<Vec<&str>>();
                let elf_one = bounds(elves[0]);
                let elf_two = bounds(elves[1]);

                if (elf_one.0 <= elf_two.0 && elf_one.1 >= elf_two.1)
                    || (elf_two.0 <= elf_one.0 && elf_two.1 >= elf_one.1)
                {
                    overlaps += 1;
                }
            }
            Err(_) => {}
        }
    }

    "CMZ"
}

/// Function for part 02
fn aux_two(file: &Path) -> &str {
    "CMZ"
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
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(aux_one(Path::new("input/test.txt")), "CMZ");
        // assert_eq!(aux_two(Path::new("input/test.txt")), 4);
    }
}
