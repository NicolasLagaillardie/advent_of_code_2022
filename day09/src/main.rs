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

    // Starting cell of head and tail
    let mut head_cell = (0, 0);
    let mut node_01_cell = (0, 0);

    // Stores explored cells
    let mut explored_cells = vec![node_01_cell];

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
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

            let distance = (i32::abs(node_01_cell.0 - head_cell.0).pow(2)
                + i32::abs(node_01_cell.1 - head_cell.1).pow(2)) as f32;

            if distance > 2.0 {
                node_01_cell.0 = (head_cell.0 - node_01_cell.0)
                    / max(1, i32::abs(head_cell.0 - node_01_cell.0))
                    + node_01_cell.0;
                node_01_cell.1 = (head_cell.1 - node_01_cell.1)
                    / max(1, i32::abs(head_cell.1 - node_01_cell.1))
                    + node_01_cell.1;
            }

            println!("distance: {distance}");
            println!("head_cell: {:?}", head_cell);
            println!("node_01_cell: {:?}", node_01_cell);

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

    // Starting cell of head and tail
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
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
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
            let distance_01_head = (i32::abs(node_01_cell.0 - head_cell.0).pow(2)
                + i32::abs(node_01_cell.1 - head_cell.1).pow(2))
                as f32;

            if distance_01_head > 2.0 {
                node_01_cell.0 = (head_cell.0 - node_01_cell.0)
                    / max(1, i32::abs(head_cell.0 - node_01_cell.0))
                    + node_01_cell.0;
                node_01_cell.1 = (head_cell.1 - node_01_cell.1)
                    / max(1, i32::abs(head_cell.1 - node_01_cell.1))
                    + node_01_cell.1;
            }

            // Node 02 to node 01
            let distance_02_01 = (i32::abs(node_01_cell.0 - node_02_cell.0).pow(2)
                + i32::abs(node_01_cell.1 - node_02_cell.1).pow(2))
                as f32;

            if distance_02_01 > 2.0 {
                node_02_cell.0 = (node_01_cell.0 - node_02_cell.0)
                    / max(1, i32::abs(node_01_cell.0 - node_02_cell.0))
                    + node_02_cell.0;
                node_02_cell.1 = (node_01_cell.1 - node_02_cell.1)
                    / max(1, i32::abs(node_01_cell.1 - node_02_cell.1))
                    + node_02_cell.1;
            }

            // Node 03 to node 02
            let distance_03_02 = (i32::abs(node_02_cell.0 - node_03_cell.0).pow(2)
                + i32::abs(node_02_cell.1 - node_03_cell.1).pow(2))
                as f32;

            if distance_03_02 > 2.0 {
                node_03_cell.0 = (node_02_cell.0 - node_03_cell.0)
                    / max(1, i32::abs(node_02_cell.0 - node_03_cell.0))
                    + node_03_cell.0;
                node_03_cell.1 = (node_02_cell.1 - node_03_cell.1)
                    / max(1, i32::abs(node_02_cell.1 - node_03_cell.1))
                    + node_03_cell.1;
            }

            // Node 04 to node 03
            let distance_04_03 = (i32::abs(node_03_cell.0 - node_04_cell.0).pow(2)
                + i32::abs(node_03_cell.1 - node_04_cell.1).pow(2))
                as f32;

            if distance_04_03 > 2.0 {
                node_04_cell.0 = (node_03_cell.0 - node_04_cell.0)
                    / max(1, i32::abs(node_03_cell.0 - node_04_cell.0))
                    + node_04_cell.0;
                node_04_cell.1 = (node_03_cell.1 - node_04_cell.1)
                    / max(1, i32::abs(node_03_cell.1 - node_04_cell.1))
                    + node_04_cell.1;
            }

            // Node 05 to node 04
            let distance_05_04 = (i32::abs(node_04_cell.0 - node_05_cell.0).pow(2)
                + i32::abs(node_04_cell.1 - node_05_cell.1).pow(2))
                as f32;

            if distance_05_04 > 2.0 {
                node_05_cell.0 = (node_04_cell.0 - node_05_cell.0)
                    / max(1, i32::abs(node_04_cell.0 - node_05_cell.0))
                    + node_05_cell.0;
                node_05_cell.1 = (node_04_cell.1 - node_05_cell.1)
                    / max(1, i32::abs(node_04_cell.1 - node_05_cell.1))
                    + node_05_cell.1;
            }

            // Node 06 to node 05
            let distance_06_05 = (i32::abs(node_05_cell.0 - node_06_cell.0).pow(2)
                + i32::abs(node_05_cell.1 - node_06_cell.1).pow(2))
                as f32;

            if distance_06_05 > 2.0 {
                node_06_cell.0 = (node_05_cell.0 - node_06_cell.0)
                    / max(1, i32::abs(node_05_cell.0 - node_06_cell.0))
                    + node_06_cell.0;
                node_06_cell.1 = (node_05_cell.1 - node_06_cell.1)
                    / max(1, i32::abs(node_05_cell.1 - node_06_cell.1))
                    + node_06_cell.1;
            }

            // Node 07 to node 06
            let distance_07_06 = (i32::abs(node_06_cell.0 - node_07_cell.0).pow(2)
                + i32::abs(node_06_cell.1 - node_07_cell.1).pow(2))
                as f32;

            if distance_07_06 > 2.0 {
                node_07_cell.0 = (node_06_cell.0 - node_07_cell.0)
                    / max(1, i32::abs(node_06_cell.0 - node_07_cell.0))
                    + node_07_cell.0;
                node_07_cell.1 = (node_06_cell.1 - node_07_cell.1)
                    / max(1, i32::abs(node_06_cell.1 - node_07_cell.1))
                    + node_07_cell.1;
            }

            // Node 08 to node 07
            let distance_08_07 = (i32::abs(node_07_cell.0 - node_08_cell.0).pow(2)
                + i32::abs(node_07_cell.1 - node_08_cell.1).pow(2))
                as f32;

            if distance_08_07 > 2.0 {
                node_08_cell.0 = (node_07_cell.0 - node_08_cell.0)
                    / max(1, i32::abs(node_07_cell.0 - node_08_cell.0))
                    + node_08_cell.0;
                node_08_cell.1 = (node_07_cell.1 - node_08_cell.1)
                    / max(1, i32::abs(node_07_cell.1 - node_08_cell.1))
                    + node_08_cell.1;
            }

            // Node 09 to node 08
            let distance_09_08 = (i32::abs(node_08_cell.0 - node_09_cell.0).pow(2)
                + i32::abs(node_08_cell.1 - node_09_cell.1).pow(2))
                as f32;

            if distance_09_08 > 2.0 {
                node_09_cell.0 = (node_08_cell.0 - node_09_cell.0)
                    / max(1, i32::abs(node_08_cell.0 - node_09_cell.0))
                    + node_09_cell.0;
                node_09_cell.1 = (node_08_cell.1 - node_09_cell.1)
                    / max(1, i32::abs(node_08_cell.1 - node_09_cell.1))
                    + node_09_cell.1;
            }

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
