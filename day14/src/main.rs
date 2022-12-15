use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut coordinates_lines_sand = Vec::new();

    let mut threshold = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        if !line.is_empty() {
            let line = line
                .split(" -> ")
                .map(|elt| elt.to_string())
                .collect::<Vec<String>>();

            let mut coordinates = Vec::new();

            let mut temp_previous_coordinates = (-1, -1);

            for elt in line.iter() {
                // There are only two integers
                let temp_elt = elt
                    .split(",")
                    .map(|elt| elt.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if temp_previous_coordinates == (-1, -1) {
                    temp_previous_coordinates = (temp_elt[0], temp_elt[1]);
                } else {
                    // If vertical or horizontal
                    if temp_previous_coordinates.0 == temp_elt[0] {
                        if temp_previous_coordinates.1 < temp_elt[1] {
                            for index in temp_previous_coordinates.1..=temp_elt[1] {
                                if threshold < index {
                                    threshold = index;
                                }
                                coordinates.push((temp_previous_coordinates.0, index));
                            }
                        } else {
                            for index in (temp_elt[1]..=temp_previous_coordinates.1).rev() {
                                if threshold < index {
                                    threshold = index;
                                }
                                coordinates.push((temp_previous_coordinates.0, index));
                            }
                        }
                    } else if temp_previous_coordinates.1 == temp_elt[1] {
                        if temp_previous_coordinates.0 < temp_elt[0] {
                            for index in temp_previous_coordinates.0..=temp_elt[0] {
                                coordinates.push((index, temp_previous_coordinates.1));
                            }
                        } else {
                            for index in (temp_elt[0]..=temp_previous_coordinates.0).rev() {
                                coordinates.push((index, temp_previous_coordinates.1));
                            }
                        }
                    } else {
                        panic!(
                            "Wrong coordinates between {:?} and {:?}",
                            temp_previous_coordinates, temp_elt
                        );
                    }

                    temp_previous_coordinates = (temp_elt[0], temp_elt[1]);
                }
            }

            coordinates_lines_sand.append(&mut coordinates);
        }
    }

    let mut coordinates_rocks = Vec::new();

    loop {
        let mut current_rock = (500, 0);

        let mut rest = false;

        while current_rock.1 < threshold && !rest {
            if !coordinates_lines_sand.contains(&(current_rock.0, current_rock.1 + 1)) {
                current_rock.1 = current_rock.1 + 1;
            } else if !coordinates_lines_sand.contains(&(current_rock.0 - 1, current_rock.1 + 1)) {
                current_rock.0 = current_rock.0 - 1;
                current_rock.1 = current_rock.1 + 1;
            } else if !coordinates_lines_sand.contains(&(current_rock.0 + 1, current_rock.1 + 1)) {
                current_rock.0 = current_rock.0 + 1;
                current_rock.1 = current_rock.1 + 1;
            } else {
                rest = true;
            }
        }

        if !rest {
            return coordinates_rocks.len();
        } else {
            coordinates_lines_sand.push(current_rock.clone());
            coordinates_rocks.push(current_rock.clone());
        }
    }
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut coordinates_lines_sand = Vec::new();

    let mut threshold = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        if !line.is_empty() {
            let line = line
                .split(" -> ")
                .map(|elt| elt.to_string())
                .collect::<Vec<String>>();

            let mut coordinates = Vec::new();

            let mut temp_previous_coordinates = (-1, -1);

            for elt in line.iter() {
                // There are only two integers
                let temp_elt = elt
                    .split(",")
                    .map(|elt| elt.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if temp_previous_coordinates == (-1, -1) {
                    temp_previous_coordinates = (temp_elt[0], temp_elt[1]);
                } else {
                    // If vertical or horizontal
                    if temp_previous_coordinates.0 == temp_elt[0] {
                        if temp_previous_coordinates.1 < temp_elt[1] {
                            for index in temp_previous_coordinates.1..=temp_elt[1] {
                                if threshold < index {
                                    threshold = index;
                                }
                                coordinates.push((temp_previous_coordinates.0, index));
                            }
                        } else {
                            for index in (temp_elt[1]..=temp_previous_coordinates.1).rev() {
                                if threshold < index {
                                    threshold = index;
                                }
                                coordinates.push((temp_previous_coordinates.0, index));
                            }
                        }
                    } else if temp_previous_coordinates.1 == temp_elt[1] {
                        if temp_previous_coordinates.0 < temp_elt[0] {
                            for index in temp_previous_coordinates.0..=temp_elt[0] {
                                coordinates.push((index, temp_previous_coordinates.1));
                            }
                        } else {
                            for index in (temp_elt[0]..=temp_previous_coordinates.0).rev() {
                                coordinates.push((index, temp_previous_coordinates.1));
                            }
                        }
                    } else {
                        panic!(
                            "Wrong coordinates between {:?} and {:?}",
                            temp_previous_coordinates, temp_elt
                        );
                    }

                    temp_previous_coordinates = (temp_elt[0], temp_elt[1]);
                }
            }

            coordinates_lines_sand.append(&mut coordinates);
        }
    }

    let mut coordinates_rocks = Vec::new();

    loop {
        let mut current_rock = (500, 0);

        let mut rest = false;

        while current_rock.1 < threshold + 1 && !rest {
            if !coordinates_lines_sand.contains(&(current_rock.0, current_rock.1 + 1)) {
                current_rock.1 = current_rock.1 + 1;
            } else if !coordinates_lines_sand.contains(&(current_rock.0 - 1, current_rock.1 + 1)) {
                current_rock.0 = current_rock.0 - 1;
                current_rock.1 = current_rock.1 + 1;
            } else if !coordinates_lines_sand.contains(&(current_rock.0 + 1, current_rock.1 + 1)) {
                current_rock.0 = current_rock.0 + 1;
                current_rock.1 = current_rock.1 + 1;
            } else {
                rest = true;
            }
        }

        println!("current rocks: {:?}", coordinates_rocks.len());

        if current_rock == (500, 0) {
            coordinates_lines_sand.push(current_rock.clone());
            coordinates_rocks.push(current_rock.clone());
            return coordinates_rocks.len();
        } else {
            coordinates_lines_sand.push(current_rock.clone());
            coordinates_rocks.push(current_rock.clone());
        }
    }
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 24);
        assert_eq!(aux_two(Path::new("input/test.txt")), 93);
    }
}
