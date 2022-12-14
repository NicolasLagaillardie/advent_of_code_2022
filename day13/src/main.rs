use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Extract element from given list
fn extract_first_value_list(mut list: Vec<String>) -> (Vec<String>, Vec<String>) {
    if list.is_empty() {
        return (vec![], vec![]);
    }

    let mut extracted_list = Vec::new();

    for elt in list.clone().iter() {
        match &elt[..] {
            "[" => {
                extracted_list.push(list.remove(0));
            }
            "]" => {
                extracted_list.push(list.remove(0));
            }
            "," => {
                if extracted_list.iter().filter(|&elt| elt == &"[").count()
                    == extracted_list.iter().filter(|&elt| elt == &"]").count()
                {
                    list.remove(0);
                    return (extracted_list, list);
                } else {
                    extracted_list.push(list.remove(0));
                }
            }
            _ => {
                extracted_list.push(list.remove(0));
            }
        }
    }

    (extracted_list, list)
}

/// Remove first matching brackets
fn remove_matching_brackets(mut list: Vec<String>) -> Vec<String> {
    let mut starting_bracket = 0;
    let mut ending_bracket = 0;
    let mut group_bracket = 0;

    for (index, elt) in list.clone().iter().enumerate() {
        match &elt[..] {
            "[" => {
                if group_bracket == 0 {
                    starting_bracket = index;
                }

                group_bracket += 1;
            }
            "]" => {
                group_bracket -= 1;
                if group_bracket == 0 {
                    ending_bracket = index;
                }
            }
            _ => {}
        }
    }

    list.remove(ending_bracket);
    list.remove(starting_bracket);

    list
}

/// Compare both left and right
/// Returns index if they are in right order, 0 if not
fn compare(left: Vec<String>, right: Vec<String>, order: bool) -> bool {
    if left.is_empty() {
        return true;
    } else if !left.is_empty() && right.is_empty() {
        return false;
    }

    println!("left: {:?}", left);
    println!("right: {:?}", right);
    println!("");

    let (mut current_value_left, left) = extract_first_value_list(left);
    let (mut current_value_right, right) = extract_first_value_list(right);

    println!("current_value_left: {:?}", current_value_left);
    println!("left: {:?}", left);
    println!("current_value_right: {:?}", current_value_right);
    println!("right: {:?}", right);
    println!("");
    println!("");

    if current_value_left.is_empty() {
        return true && order && compare(left, right, order);
    } else if !current_value_left.is_empty() && current_value_right.is_empty() {
        return false && order && compare(left, right, order);
    }

    if current_value_left[0] != "[" {
        if current_value_right[0] != "[" {
            // If only integer(s)

            if current_value_left[0].parse::<i32>().unwrap()
                < current_value_right[0].parse::<i32>().unwrap()
            {
                return true;
            } else if current_value_left[0].parse::<i32>().unwrap()
                > current_value_right[0].parse::<i32>().unwrap()
            {
                return false;
            } else {
                // Remove first integer and possible following comma
                current_value_left.remove(0);
                if !current_value_left.is_empty() {
                    current_value_left.remove(0);
                }
                current_value_right.remove(0);
                if !current_value_right.is_empty() {
                    current_value_right.remove(0);
                }
                return order
                    && compare(current_value_left, current_value_right, order)
                    && compare(left, right, order);
            }
        } else {
            // If left integer(s) and right list(s)
            current_value_right = remove_matching_brackets(current_value_right);

            return order
                && compare(current_value_left, current_value_right, order)
                && compare(left, right, order);
        }
    } else {
        if current_value_right[0] != "[" {
            // If left list(s) and right integer(s)
            current_value_left = remove_matching_brackets(current_value_left);

            return order
                && compare(current_value_left, current_value_right, order)
                && compare(left, right, order);
        } else {
            // If both lists
            current_value_left = remove_matching_brackets(current_value_left);
            current_value_right = remove_matching_brackets(current_value_right);

            return order
                && compare(current_value_left, current_value_right, order)
                && compare(left, right, order);
        }
    }
}

/// Extract elements from line
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

    println!("matrix: {:?}", matrix);
    println!("matrix len: {:?}", matrix.len());

    matrix
        .iter()
        .enumerate()
        .map(|(index_pair, list)| {
            // Remove first and last [ ] for left
            let mut current_elt_left = list.0.clone();
            current_elt_left.remove(0);
            current_elt_left.remove(current_elt_left.len() - 1);

            // Remove first and last [ ] for right
            let mut current_elt_right = list.1.clone();
            current_elt_right.remove(0);
            current_elt_right.remove(current_elt_right.len() - 1);

            // Return result
            if compare(current_elt_left, current_elt_right, true) {
                println!("Is in order: {}", index_pair + 1);
                index_pair as i32 + 1
            } else {
                0
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
