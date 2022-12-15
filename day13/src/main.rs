use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

enum Result {
    True,
    False,
    Unknown,
}

/// Split head and tail from list
fn head_tail(mut list: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut head = Vec::new();
    let mut group_bracket = 0;

    for elt in list.clone().iter() {
        match &elt[..] {
            "[" => {
                if group_bracket > 0 {
                    head.push(list.remove(0));
                } else {
                    list.remove(0);
                }

                group_bracket += 1;
            }
            "]" => {
                if group_bracket == 1 {
                    // If we didn't reach the end of the list
                    if !list.is_empty() && list[0] == "," {
                        list.remove(0);
                    }
                    // Close the tail
                    list.insert(0, "[".to_string());

                    return (head, list);
                } else if group_bracket > 1 {
                    head.push(list.remove(0));
                }
                group_bracket -= 1;
            }
            "," => {
                if head.iter().filter(|&elt| elt == &"[").count()
                    == head.iter().filter(|&elt| elt == &"]").count()
                {
                    // If we didn't reach the end of the list
                    if list[0] == "," {
                        list.remove(0);
                    }
                    // Close the tail
                    list.insert(0, "[".to_string());

                    return (head, list);
                } else {
                    head.push(list.remove(0));
                }
            }
            _ => {
                head.push(list.remove(0));
            }
        }
    }

    return (head, list);
}

/// Compare both left and right
/// Returns index if they are in right order, 0 if not
fn compare(tail_left: Vec<String>, tail_right: Vec<String>, index: usize) -> Result {
    if tail_left.is_empty() {
        if tail_right.is_empty() {
            return Result::Unknown;
        } else {
            return Result::True;
        }
    } else if !tail_left.is_empty() && tail_right.is_empty() {
        return Result::False;
    } else if tail_left == tail_right {
        return Result::Unknown;
    }

    let (mut head_left, tail_left) = head_tail(tail_left);
    let (mut head_right, tail_right) = head_tail(tail_right);

    if head_left == head_right {
        return compare(tail_left, tail_right, index);
    }

    if head_left.len() == 1 {
        if head_right.len() == 1 {
            // If only integer(s)
            if head_left[0].parse::<i32>().unwrap() < head_right[0].parse::<i32>().unwrap() {
                return Result::True;
            } else if head_left[0].parse::<i32>().unwrap() > head_right[0].parse::<i32>().unwrap() {
                return Result::False;
            } else {
                match compare(tail_left.clone(), tail_right.clone(), index) {
                    Result::Unknown => panic!("Wrong output for compare"),
                    Result::True => {
                        return Result::True;
                    }
                    Result::False => return Result::False,
                };
            }
        } else {
            // If head_left integer and head_right list(s)
            head_left.insert(0, "[".to_string());
            head_left.push("]".to_string());

            match compare(head_left.clone(), head_right.clone(), index) {
                Result::Unknown => return compare(tail_left, tail_right, index),
                Result::True => {
                    return Result::True;
                }
                Result::False => return Result::False,
            };
        }
    } else {
        if head_right.len() == 1 {
            // If head_left list(s) and head_right integer
            head_right.insert(0, "[".to_string());
            head_right.push("]".to_string());

            match compare(head_left.clone(), head_right.clone(), index) {
                Result::Unknown => return compare(tail_left, tail_right, index),
                Result::True => {
                    return Result::True;
                }
                Result::False => return Result::False,
            };
        } else {
            match compare(head_left.clone(), head_right.clone(), index) {
                Result::Unknown => return compare(tail_left, tail_right, index),
                Result::True => {
                    return Result::True;
                }
                Result::False => return Result::False,
            };
        }
    }
}

/// Extract elements from line
/// Done because integers > 9
fn extract_left_right(line: String) -> Vec<String> {
    let mut result = Vec::new();

    let mut temp_elt = "".to_string();

    for elt in line.chars().into_iter() {
        match elt {
            '[' => {
                if !temp_elt.is_empty() {
                    result.push(temp_elt);
                }
                result.push("[".to_string());
                temp_elt = "".to_string();
            }
            ']' => {
                if !temp_elt.is_empty() {
                    result.push(temp_elt);
                }
                result.push("]".to_string());
                temp_elt = "".to_string();
            }
            ',' => {
                if !temp_elt.is_empty() {
                    result.push(temp_elt);
                }
                result.push(",".to_string());
                temp_elt = "".to_string();
            }
            _ => {
                temp_elt = format!("{temp_elt}{elt}");
            }
        }
    }

    result
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut matrix = Vec::new();

    let mut left = Vec::new();

    // Read file line by line, for part 01
    for (index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();

        if index_line % 3 == 0 {
            left = extract_left_right(line);
        } else if index_line % 3 == 1 {
            let right = extract_left_right(line);

            matrix.push((left.clone(), right.clone()));
        }
    }

    matrix
        .iter()
        .enumerate()
        .map(|(index_pair, list)| {
            // Remove first and last [ ] for left
            let current_elt_left = list.0.clone();

            // Remove first and last [ ] for right
            let current_elt_right = list.1.clone();

            match compare(
                current_elt_left.clone(),
                current_elt_right.clone(),
                index_pair,
            ) {
                Result::Unknown => panic!("Wrong output"),
                Result::True => index_pair as i32 + 1,
                Result::False => 0,
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    println!("file: {:?}", file);

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
        assert_eq!(aux_one(Path::new("input/test.txt")), 13);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 29);
    }
}
