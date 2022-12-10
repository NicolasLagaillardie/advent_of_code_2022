use core::panic;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Update position of tail compared to head
fn update_cell_position(head_cell: &(i32, i32), tail_cell: &mut (i32, i32)) {
    if (head_cell.0 - tail_cell.0).abs().pow(2) + (head_cell.1 - tail_cell.1).abs().pow(2) > 2 {
        tail_cell.0 =
            (head_cell.0 - tail_cell.0) / max(1, (head_cell.0 - tail_cell.0).abs()) + tail_cell.0;
        tail_cell.1 =
            (head_cell.1 - tail_cell.1) / max(1, (head_cell.1 - tail_cell.1).abs()) + tail_cell.1;
    }
}

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Starting cell of head and tail
    let mut head_cell = (0, 0);
    let mut node_01_cell = (0, 0);

    // Stores explored cells
    let mut explored_cells = vec![node_01_cell];

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let instruction = line.split(' ').collect::<Vec<&str>>();

        let direction = instruction[0];
        let steps = instruction[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                // Move left
                "L" => {
                    head_cell.0 -= 1;
                }
                // Move right
                "R" => {
                    head_cell.0 += 1;
                }
                // Move up
                "U" => {
                    head_cell.1 += 1;
                }
                // Move down
                "D" => {
                    head_cell.1 -= 1;
                }

                err => panic!("Error, wrong direction, expected L, R, U or D, found {err}"),
            }

            update_cell_position(&head_cell, &mut node_01_cell);

            explored_cells.push(node_01_cell);
        }
    }

    println!("explored_cells: {:?}", explored_cells);

    // Keep only unique elements
    explored_cells
        .into_iter()
        .map(|elt| elt)
        .collect::<HashSet<_>>()
        .len()
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Starting cells
    let mut head_cell = (0, 0);
    let mut node_01_cell = (0, 0);
    let mut node_02_cell = (0, 0);
    let mut node_03_cell = (0, 0);
    let mut node_04_cell = (0, 0);
    let mut node_05_cell = (0, 0);
    let mut node_06_cell = (0, 0);
    let mut node_07_cell = (0, 0);
    let mut node_08_cell = (0, 0);
    let mut node_09_cell = (0, 0);

    // Stores explored cells
    let mut node_09_explored_cells = vec![node_09_cell];

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let instruction = line.split(' ').collect::<Vec<&str>>();

        let direction = instruction[0];
        let steps = instruction[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                // Move left
                "L" => {
                    head_cell.0 -= 1;
                }
                // Move right
                "R" => {
                    head_cell.0 += 1;
                }
                // Move up
                "U" => {
                    head_cell.1 += 1;
                }
                // Move down
                "D" => {
                    head_cell.1 -= 1;
                }

                err => panic!("Error, wrong direction, expected L, R, U or D, found {err}"),
            }

            // Head to node 01
            update_cell_position(&head_cell, &mut node_01_cell);

            // Node 02 to node 01
            update_cell_position(&node_01_cell, &mut node_02_cell);

            // Node 03 to node 02
            update_cell_position(&node_02_cell, &mut node_03_cell);

            // Node 04 to node 03
            update_cell_position(&node_03_cell, &mut node_04_cell);

            // Node 05 to node 04
            update_cell_position(&node_04_cell, &mut node_05_cell);

            // Node 06 to node 05
            update_cell_position(&node_05_cell, &mut node_06_cell);

            // Node 07 to node 06
            update_cell_position(&node_06_cell, &mut node_07_cell);

            // Node 08 to node 07
            update_cell_position(&node_07_cell, &mut node_08_cell);

            // Node 09 to node 08
            update_cell_position(&node_08_cell, &mut node_09_cell);

            node_09_explored_cells.push(node_09_cell);
        }
    }

    // Keep only unique elements
    node_09_explored_cells
        .into_iter()
        .map(|elt| elt)
        .collect::<HashSet<_>>()
        .len()
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
        assert_eq!(aux_one(Path::new("input/test01.txt")), 13);
        assert_eq!(aux_two(Path::new("input/test02.txt")), 36);
    }
}
