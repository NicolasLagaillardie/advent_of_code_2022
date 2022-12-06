use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

// Function to find duplicate chars in vec of chars
fn find_duplicate_between_lists(
    list_one: Vec<char>,
    list_two: Vec<char>,
    mut list_of_duplicates: Vec<char>,
) -> Vec<char> {
    // Check duplicate letters in one part compared to the other
    for elt_one in list_one {
        for elt_two in list_two.iter() {
            if &elt_one == elt_two {
                list_of_duplicates.push(elt_one);
            }
        }
    }

    list_of_duplicates
}

/// Function for part 01
fn aux_one(file: &Path) -> u32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Final result
    let mut score: u32 = 0;

    // Vec with only the duplicated letter
    let mut duplicated_letters = Vec::new();

    // Checks if duplicate letter found
    // let mut found = false;

    // Read file line by line, for part 01
    for (_index, line) in reader.lines().enumerate() {
        // Sum scores
        match line.unwrap().trim().parse::<String>() {
            Ok(elt) => {
                // Split line in two
                let mut part_one: Vec<_> = elt.chars().collect();
                let part_two = part_one.split_off(part_one.len() / 2);

                // Check duplicate letters in one part compared to the other
                'outer: for elt_one in part_one {
                    for elt_two in part_two.iter() {
                        if &elt_one == elt_two {
                            duplicated_letters.push(elt_one);
                            break 'outer;
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    // 'a' as u32 = 97
    // 'z' as u32 = 122
    // 'A' as u32 = 65
    // 'Z' as u32 = 90
    score = score
        + duplicated_letters
            .iter()
            .filter_map(|a: &char| {
                let ascii_value = *a as u32;
                // If uppercase or lowercase latin letter
                if ascii_value < 91 && ascii_value > 64 {
                    Some(ascii_value - 65 + 27)
                } else if ascii_value < 123 && ascii_value > 96 {
                    Some(ascii_value - 96)
                } else {
                    None
                }
            })
            .sum::<u32>();

    score
}

/// Function for part 02
fn aux_two(file: &Path) -> u32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Final result
    let mut score = 0;

    // Temporaly stores rucksacks three by three
    let mut rucksacks_groups: Vec<Vec<char>> = Vec::new();

    // Stores duplicated letters among every group of three rucksacks
    let mut duplicated_letters: Vec<char> = Vec::new();

    // Read file line by line, for part 02
    for (index, line) in reader.lines().enumerate() {
        // If within a group of three lines, starting from line 0, add chars to rucksacks_groups,
        // else,
        // find dulplicated letters and add them to duplicated_letters
        if index == 0 || index % 3 != 0 {
            // Add all 3 rucksacks to rucksacks_groups
            match line.unwrap().trim().parse::<String>() {
                Ok(elt) => {
                    // Split line in two
                    rucksacks_groups.push(elt.chars().collect());
                }
                Err(_) => {}
            }
        } else {
            let temp_one_two = find_duplicate_between_lists(
                rucksacks_groups[0].clone(),
                rucksacks_groups[1].clone(),
                Vec::new(),
            );

            // Check duplicate letters in one part compared to the other
            'outer: for elt_one in temp_one_two {
                for elt_two in rucksacks_groups[2].iter() {
                    if &elt_one == elt_two {
                        duplicated_letters.push(elt_one);
                        break 'outer;
                    }
                }
            }

            rucksacks_groups = Vec::new();

            // Add all 3 rucksacks to rucksacks_groups
            match line.unwrap().trim().parse::<String>() {
                Ok(elt) => {
                    // Split line in two
                    rucksacks_groups.push(elt.chars().collect());
                }
                Err(_) => {}
            }
        }
    }

    // Add last group of three lines
    let temp_one_two = find_duplicate_between_lists(
        rucksacks_groups[0].clone(),
        rucksacks_groups[1].clone(),
        Vec::new(),
    );

    // Check duplicate letters in one part compared to the other
    'outer: for elt_one in temp_one_two {
        for elt_two in rucksacks_groups[2].iter() {
            if &elt_one == elt_two {
                duplicated_letters.push(elt_one);
                break 'outer;
            }
        }
    }

    // 'a' as u32 = 97
    // 'z' as u32 = 122
    // 'A' as u32 = 65
    // 'Z' as u32 = 90
    score = score
        + duplicated_letters
            .iter()
            .filter_map(|a: &char| {
                let ascii_value = *a as u32;
                // If uppercase or lowercase latin letter
                if ascii_value < 91 && ascii_value > 64 {
                    Some(ascii_value - 65 + 27)
                } else if ascii_value < 123 && ascii_value > 96 {
                    Some(ascii_value - 96)
                } else {
                    None
                }
            })
            .sum::<u32>();

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

    // Ask input of part
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 157);
        assert_eq!(aux_two(Path::new("input/test.txt")), 70);
    }
}
