use core::panic;
// use std::cmp::min;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut cubes = Vec::new();

    let mut common_faces = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let coordinates = line.split(',').collect::<Vec<_>>();

        let x = coordinates[0].parse::<i32>().unwrap();
        let y = coordinates[1].parse::<i32>().unwrap();
        let z = coordinates[2].parse::<i32>().unwrap();

        if cubes.contains(&(x - 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x + 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y - 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y + 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z - 1)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z + 1)) {
            common_faces += 1;
        }

        cubes.push((x, y, z));
    }

    cubes.len() as i128 * 6 - common_faces * 2
}

/// Function for part 02
fn aux_two(_file: &Path) -> i128 {
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 64);
        assert_eq!(aux_two(Path::new("input/test.txt")), 58);
    }
}
