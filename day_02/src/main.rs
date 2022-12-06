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

    let mut score = 0;

    // Read file line by line
    for (_index, line) in reader.lines().enumerate() {
        // Sum scores
        match line.unwrap().trim().parse::<String>().unwrap().as_str() {
            // Cases when they play Rock ...
            // ... and I play Rock
            "A X" => {
                score = score + 1 + 3;
            }
            // ... and I play Paper
            "A Y" => {
                score = score + 2 + 6;
            }
            // ... and I play Scissors
            "A Z" => {
                score = score + 3 + 0;
            }
            // Cases when they play Paper ...
            // ... and I play Rock
            "B X" => {
                score = score + 1 + 0;
            }
            // ... and I play Paper
            "B Y" => {
                score = score + 2 + 3;
            }
            // ... and I play Scissors
            "B Z" => {
                score = score + 3 + 6;
            }
            // Cases when they play Scissors ...
            // ... and I play Rock
            "C X" => {
                score = score + 1 + 6;
            }
            // ... and I play Paper
            "C Y" => {
                score = score + 2 + 0;
            }
            // ... and I play Scissors
            "C Z" => {
                score = score + 3 + 3;
            }
            elt => println!("Error with {elt}"),
        };
    }

    // Display sum of top three
    println!("Sum: {}", score);
}
