use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut calories = Vec::<i32>::new();

    let mut temp_calories: i32 = 0;

    // Read file line by line
    for (_index, line) in reader.lines().enumerate() {
        // Add sum of calories of each elves
        match line.unwrap().trim().parse::<i32>() {
            Ok(elt) => {
                temp_calories = temp_calories + elt;
            }
            Err(_) => {
                calories.push(temp_calories);
                temp_calories = 0;
            }
        };
    }
    
    calories.push(temp_calories);

    // Sort by descending order
    calories.sort_by(|a, b| b.cmp(a));

    calories[0]
}

// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut calories = Vec::<i32>::new();

    let mut temp_calories: i32 = 0;

    // Read file line by line
    for (_index, line) in reader.lines().enumerate() {
        // Add sum of calories of each elves
        match line.unwrap().trim().parse::<i32>() {
            Ok(elt) => {
                temp_calories = temp_calories + elt;
            }
            Err(_) => {
                calories.push(temp_calories);
                temp_calories = 0;
            }
        };
    }

    calories.push(temp_calories);

    // Sort by descending order
    calories.sort_by(|a, b| b.cmp(a));

    println!("{:?}", calories);

    calories[0] + calories[1] + calories[2]
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

    // Ask input of path
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

    // Display top one
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(aux_one(Path::new("input/test.txt")), 24000);
        assert_eq!(aux_two(Path::new("input/test.txt")), 45000);
    }
}
