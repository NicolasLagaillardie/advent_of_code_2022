use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Main function
fn main() {
    println!("Enter path to file");

    // Ask input of path
    let mut path = String::new();

    stdin().read_line(&mut path).expect("Failed to read input");

    // Remove end of input containing \n
    let path = path.trim();

    let file = Path::new(path);

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut calories = Vec::<i32>::new();

    let mut temp_calories: i32 = 0;

    // Read file line by line
    for (_index, line) in reader.lines().enumerate() {
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

    println!("Sum: {}", calories[0] + calories[1] + calories[2]);
}
