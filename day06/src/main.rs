use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Store index of first marker
    let mut index = 0;

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();

        // Start with an empty vec
        // It will contain each block of 4 consecutive chars
        let mut temp = Vec::new();

        // While we didn't reach the end of the vec line
        // and while the lenght of the vec temp reduced
        // its unique elements is not 4,
        // go to the following block of 4 chars
        while index + 4 < line.len()
            && temp
                .into_iter()
                .map(|elt| elt)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<char>>()
                .len()
                != 4
        {
            temp = line[index..(index + 4)].to_vec();
            index += 1;
        }
    }

    index - 1 + 4
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Store index of first marker
    let mut index = 0;

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();

        // Start with an empty vec
        // It will contain each block of 4 consecutive chars
        let mut temp = Vec::new();

        // While we didn't reach the end of the vec line
        // and while the lenght of the vec temp reduced
        // its unique elements is not 4,
        // go to the following block of 4 chars
        while index + 14 < line.len()
            && temp
                .into_iter()
                .map(|elt| elt)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<char>>()
                .len()
                != 14
        {
            temp = line[index..(index + 14)].to_vec();
            index += 1;
        }
    }

    index - 1 + 14
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 7);
        assert_eq!(aux_two(Path::new("input/test.txt")), 19);
    }
}
