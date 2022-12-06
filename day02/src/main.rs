use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut score = 0;

    // Read file line by line, for part 01
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

    score
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut score = 0;

    // Read file line by line, for part 02
    for (_index, line) in reader.lines().enumerate() {
        // Sum scores
        match line.unwrap().trim().parse::<String>().unwrap().as_str() {
            // Cases when they play Rock ...
            // ... and I must lose
            "A X" => {
                score = score + 3 + 0;
            }
            // ... and I must draw
            "A Y" => {
                score = score + 1 + 3;
            }
            // ... and I must win
            "A Z" => {
                score = score + 2 + 6;
            }
            // Cases when they play Paper ...
            // ... and I must lose
            "B X" => {
                score = score + 1 + 0;
            }
            // ... and I must draw
            "B Y" => {
                score = score + 2 + 3;
            }
            // ... and I must win
            "B Z" => {
                score = score + 3 + 6;
            }
            // Cases when they play Scissors ...
            // ... and I must lose
            "C X" => {
                score = score + 2 + 0;
            }
            // ... and I must draw
            "C Y" => {
                score = score + 3 + 3;
            }
            // ... and I must win
            "C Z" => {
                score = score + 1 + 6;
            }
            elt => println!("Error with {elt}"),
        };
    }

    score
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

    let score = match choice.trim() {
        "1" => aux_one(file),
        "2" => aux_two(file),
        _ => panic!("Error, expecting 1 or 2"),
    };

    // Display total score
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(aux_one(Path::new("input/test.txt")), 15);
        assert_eq!(aux_two(Path::new("input/test.txt")), 12);
    }
}
