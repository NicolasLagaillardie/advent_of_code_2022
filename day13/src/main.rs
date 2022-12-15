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
fn compare(tail_left: Vec<String>, tail_right: Vec<String>) -> Result {
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
        return compare(tail_left, tail_right);
    }

    if head_left.len() == 1 {
        if head_right.len() == 1 {
            // If only integer(s)
            if head_left[0].parse::<i32>().unwrap() < head_right[0].parse::<i32>().unwrap() {
                return Result::True;
            } else if head_left[0].parse::<i32>().unwrap() > head_right[0].parse::<i32>().unwrap() {
                return Result::False;
            } else {
                match compare(tail_left, tail_right) {
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

            match compare(head_left, head_right) {
                Result::Unknown => return compare(tail_left, tail_right),
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

            match compare(head_left, head_right) {
                Result::Unknown => return compare(tail_left, tail_right),
                Result::True => {
                    return Result::True;
                }
                Result::False => return Result::False,
            };
        } else {
            match compare(head_left, head_right) {
                Result::Unknown => return compare(tail_left, tail_right),
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
fn extract_vec_from_str(line: String) -> Vec<String> {
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
            left = extract_vec_from_str(line);
        } else if index_line % 3 == 1 {
            let right = extract_vec_from_str(line);

            matrix.push((left.clone(), right.clone()));
        }
    }

    matrix
        .iter()
        .enumerate()
        .map(
            |(index_pair, list)| match compare(list.0.clone(), list.1.clone()) {
                Result::Unknown => panic!("Wrong output"),
                Result::True => index_pair as i32 + 1,
                Result::False => 0,
            },
        )
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn sort_signals(mut matrix: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut clone = matrix.clone();
    let mut in_order = false;
    while !in_order {
        clone = matrix.clone();
        in_order = true;

        for (index, _list) in matrix.iter().enumerate() {
            if index < matrix.len() - 1 {
                match compare(matrix[index].clone(), matrix[index + 1].clone()) {
                    Result::Unknown => panic!("Wrong output"),
                    Result::True => {}
                    Result::False => {
                        (clone[index], clone[index + 1]) =
                            (clone[index + 1].clone(), clone[index].clone());
                        in_order = false;
                    }
                }
            }
        }
        matrix = clone.clone();
    }

    return clone;
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut matrix = Vec::new();

    // Read file line by line, for part 01
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();

        let line = extract_vec_from_str(line);
        if !line.is_empty() {
            matrix.push(line);
        }
    }

    let first_divider = extract_vec_from_str("[[2]]".to_string());
    let second_divider = extract_vec_from_str("[[6]]".to_string());
    matrix.push(first_divider.clone());
    matrix.push(second_divider.clone());

    matrix = sort_signals(matrix);

    let mut index_first_divider = 0;
    let mut index_second_divider = 0;

    for (index, elt) in matrix.iter().enumerate() {
        if elt.iter().cloned().collect::<String>()
            == first_divider.iter().cloned().collect::<String>()
        {
            index_first_divider = index;
        }
        if elt.iter().cloned().collect::<String>()
            == second_divider.iter().cloned().collect::<String>()
        {
            index_second_divider = index;
        }
    }

    (index_first_divider as i32 + 1) * (index_second_divider as i32 + 1)
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
        assert_eq!(aux_two(Path::new("input/test.txt")), 140);
    }
}
