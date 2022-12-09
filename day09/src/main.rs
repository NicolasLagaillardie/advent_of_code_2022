use core::panic;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Stores explored cells and prevent to count twice the same cell
    let mut explored_cells = vec![(0, 0)];

    // Starting cell of head and tail
    let mut head_cell = (0, 0);
    let mut tail_cell = (0, 0);

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line.unwrap();
        let instruction = line.split(' ').collect::<Vec<&str>>();

        let direction = instruction[0];
        let steps = instruction[1].parse::<i32>().unwrap();

        if index_line == 0 {
            // Init head cell
            head_cell = match direction {
                "L" => {
                    for _ in 0..max(0, steps - 1) {
                        tail_cell.0 -= 1;
                        explored_cells.push(tail_cell);
                    }
                    (-steps, 0)
                }
                "R" => {
                    for _ in 0..max(0, steps - 1) {
                        tail_cell.0 += 1;
                        explored_cells.push(tail_cell);
                    }
                    (steps, 0)
                }
                "U" => {
                    for _ in 0..max(0, steps - 1) {
                        tail_cell.1 += 1;
                        explored_cells.push(tail_cell);
                    }
                    (0, steps)
                }
                "D" => {
                    for _ in 0..max(0, steps - 1) {
                        tail_cell.1 -= 1;
                        explored_cells.push(tail_cell);
                    }
                    (0, -steps)
                }
                err => panic!("Error, wrong direction, expected L, R, U or D, found {err}"),
            };
        } else {
            match direction {
                // Move left
                "L" => {
                    for _ in 0..steps {
                        head_cell.0 -= 1;

                        if ((head_cell.0 - tail_cell.0).abs().pow(2)
                            + (head_cell.1 - tail_cell.1).abs().pow(2))
                            > 2
                        {
                            tail_cell.0 -= 1;
                            tail_cell.1 = head_cell.1;
                        }

                        explored_cells.push(tail_cell);
                    }
                }
                // Move right
                "R" => {
                    for _ in 0..steps {
                        head_cell.0 += 1;

                        if ((head_cell.0 - tail_cell.0).abs().pow(2)
                            + (head_cell.1 - tail_cell.1).abs().pow(2))
                            > 2
                        {
                            tail_cell.0 += 1;
                            tail_cell.1 = head_cell.1;
                        }

                        explored_cells.push(tail_cell);
                    }
                }
                // Move up
                "U" => {
                    for _ in 0..steps {
                        head_cell.1 += 1;

                        if ((head_cell.0 - tail_cell.0).abs().pow(2)
                            + (head_cell.1 - tail_cell.1).abs().pow(2))
                            > 2
                        {
                            tail_cell.1 += 1;
                            tail_cell.0 = head_cell.0;
                        }

                        explored_cells.push(tail_cell);
                    }
                }
                // Move down
                "D" => {
                    for _ in 0..steps {
                        head_cell.1 -= 1;

                        if ((head_cell.0 - tail_cell.0).abs().pow(2)
                            + (head_cell.1 - tail_cell.1).abs().pow(2))
                            > 2
                        {
                            tail_cell.1 -= 1;
                            tail_cell.0 = head_cell.0;
                        }

                        explored_cells.push(tail_cell);
                    }
                }

                err => panic!("Error, wrong direction, expected L, R, U or D, found {err}"),
            }
        }
    }

    explored_cells
        .into_iter()
        .map(|elt| elt)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<(i32, i32)>>()
        .len()
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line.unwrap();
    }

    // Number of visible trees
    let mut result = 0;

    result
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
        // assert_eq!(aux_two(Path::new("input/test.txt")), 8);
    }
}
