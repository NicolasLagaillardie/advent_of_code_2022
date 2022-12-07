use core::panic;
use regex::Regex;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// Function for part 01
fn aux_one(file: &Path) -> String {
    // Store stacks
    let mut stacks: Vec<Vec<String>> = Vec::new();

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // If we are in the stack definition
        if line.contains('[') {
            let line = line.as_bytes();

            for (index_elt, elt) in line.iter().enumerate() {
                if index_elt % 4 == 1 {
                    if elt == &32 && stacks.len() <= (index_elt - 1) / 4 {
                        stacks.push(Vec::new());
                    } else if elt != &32 && stacks.len() <= (index_elt - 1) / 4 {
                        stacks.push(Vec::new());
                        stacks[(index_elt - 1) / 4].push(from_utf8(&[*elt]).unwrap().to_string());
                    } else if elt != &32 {
                        stacks[(index_elt - 1) / 4].push(from_utf8(&[*elt]).unwrap().to_string());
                    }
                }
            }
        } else if line.contains("move") {
            // Extract digits from instruction line
            let re = Regex::new(r" (\d+)").unwrap();
            let mut instructions = Vec::new();
            for cap in re.captures_iter(&line) {
                instructions.push(cap[0].to_string().trim().parse().unwrap());
            }

            // Move the elements
            for _i in 0..instructions[0] {
                let elt = stacks[instructions[1] - 1].remove(0);
                stacks[instructions[2] - 1].insert(0, elt);
            }
        }
    }

    // Get head of each stacks and concat them
    let mut heads = Vec::new();

    for elt in stacks.iter() {
        if !elt.is_empty() {
            heads.push(elt[0].to_string());
        }
    }

    let final_result = heads.iter().cloned().collect::<String>();

    final_result
}

/// Function for part 02
fn aux_two(file: &Path) -> String {
    // Store stacks
    let mut stacks: Vec<Vec<String>> = Vec::new();

    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // If we are in the stack definition
        if line.contains('[') {
            let line = line.as_bytes();

            for (index_elt, elt) in line.iter().enumerate() {
                if index_elt % 4 == 1 {
                    if elt == &32 && stacks.len() <= (index_elt - 1) / 4 {
                        stacks.push(Vec::new());
                    } else if elt != &32 && stacks.len() <= (index_elt - 1) / 4 {
                        stacks.push(Vec::new());
                        stacks[(index_elt - 1) / 4].push(from_utf8(&[*elt]).unwrap().to_string());
                    } else if elt != &32 {
                        stacks[(index_elt - 1) / 4].push(from_utf8(&[*elt]).unwrap().to_string());
                    }
                }
            }
        } else if line.contains("move") {
            // Extract digits from instruction line
            let re = Regex::new(r" (\d+)").unwrap();
            let mut instructions = Vec::new();
            for cap in re.captures_iter(&line) {
                instructions.push(cap[0].to_string().trim().parse().unwrap());
            }

            // Move the elements
            let mut temp_stack = Vec::new();
            for _i in 0..instructions[0] {
                temp_stack.push(stacks[instructions[1] - 1].remove(0));
            }
            temp_stack.reverse();
            for elt in temp_stack.iter() {
                stacks[instructions[2] - 1].insert(0, elt.to_string());
            }
        }
    }

    // Get head of each stacks and concat them
    let mut heads = Vec::new();

    for elt in stacks.iter() {
        if !elt.is_empty() {
            heads.push(elt[0].to_string());
        }
    }

    let final_result = heads.iter().cloned().collect::<String>();
    
    final_result
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
        assert_eq!(aux_one(Path::new("input/test.txt")), "CMZ");
        assert_eq!(aux_two(Path::new("input/test.txt")), "MCD");
    }
}
