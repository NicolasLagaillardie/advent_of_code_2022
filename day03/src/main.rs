use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    // Read file line by line, for part 01
    for (_index, line) in reader.lines().enumerate() {
        // Sum scores
        match line.unwrap().trim().parse::<String>() {
            Ok(elt) => {
                let mut part_one: Vec<_> = elt.chars().collect();
                let part_two = part_one.split_off(part_one.len() / 2);
                // 'a' as u32 = 97
                // 'z' as u32 = 122
                // 'A' as u32 = 65
                // 'Z' as u32 = 90

                for elt_one in part_one {
                    for elt_two in part_two.iter() {
                        if &elt_one == elt_two {
                            let ascii_value = elt_one as u32;
                            // If uppercase
                            if ascii_value < 91 && ascii_value > 64 {
                                score = score + ascii_value - 65 + 27
                            } else if ascii_value < 123 && ascii_value > 96 {
                                score = score + ascii_value - 96
                            } else {
                                panic!("Expected latin letters, found {elt_one}");
                            }
                            break;
                        }
                    }
                }

                // score = score
                //     + part_one
                //         .iter()
                //         .map(|a| {
                //             let result: u32 = part_two
                //                 .iter()
                //                 .filter_map(|b| {
                //                     if *a == *b {
                //                         let ascii_value = *a as u32;
                //                         // If uppercase
                //                         if ascii_value < 91 && ascii_value > 64 {
                //                             Some(ascii_value - 65 + 27)
                //                         } else if ascii_value < 123 && ascii_value > 96 {
                //                             Some(ascii_value - 96)
                //                         } else {
                //                             None
                //                         }
                //                     } else {
                //                         None
                //                     }
                //                 })
                //                 .sum();
                //             result
                //         })
                //         .sum::<u32>();
            }
            Err(_) => {}
        }
    }

    println!("Score: {score}");

    0
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut score = 0;

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
        // assert_eq!(aux_one(Path::new("input/test.txt")), 157);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 12);
    }
}
