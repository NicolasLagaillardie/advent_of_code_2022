use core::panic;
use std::cmp::max;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Set decimal base
    const RADIX: u32 = 10;

    let mut composition = Vec::new();

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap())
            .collect::<Vec<u32>>();

        composition.push(line);
    }

    // Number of visible trees
    let mut result = 0;

    // If tested tree is visible
    let mut visible_from_top;
    let mut visible_from_bottom;
    let mut visible_from_left;
    let mut visible_from_right;

    // Actually count visible trees
    for (y_index, y_axis) in composition.iter().enumerate() {
        for (x_index, x_axis) in y_axis.iter().enumerate() {
            // If we are not on the border
            if y_index != 0
                && y_index != composition.len() - 1
                && x_index != 0
                && x_index != y_axis.len() - 1
            {
                visible_from_top = true;
                visible_from_bottom = true;
                visible_from_left = true;
                visible_from_right = true;

                // We check trees from top border to tree
                for y_tree in 0..y_index {
                    if &composition[y_tree][x_index] >= x_axis {
                        visible_from_top = false;
                    }
                }
                // We check trees from tree to bottom border
                for y_tree in y_index + 1..composition.len() {
                    if &composition[y_tree][x_index] >= x_axis {
                        visible_from_bottom = false;
                    }
                }
                // We check trees from left border to tree
                for x_tree in 0..x_index {
                    if &composition[y_index][x_tree] >= x_axis {
                        visible_from_left = false;
                    }
                }
                // We check trees from tree to right border
                for x_tree in x_index + 1..composition[0].len() {
                    if &composition[y_index][x_tree] >= x_axis {
                        visible_from_right = false;
                    }
                }

                // If tree entirely visible
                if visible_from_top
                    || visible_from_bottom
                    || visible_from_left
                    || visible_from_right
                {
                    result += 1;
                }
            }
        }
    }

    2 * composition.len() + 2 * composition[0].len() - 4 + result
}

/// Function for part 02
fn aux_two(file: &Path) -> usize {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Set decimal base
    const RADIX: u32 = 10;

    // Readable composition of the map
    let mut composition = Vec::new();

    // Read file line by line, for part 01
    // Retrieve all integers in vec of vec
    for (_index_line, line) in reader.lines().enumerate() {
        // Split all integers and parse them into u32
        let line = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap())
            .collect::<Vec<u32>>();

        composition.push(line);
    }

    // Number of visible trees
    let mut result = 0;

    // If tested tree is visible
    let mut visible_from_top;
    let mut visible_from_bottom;
    let mut visible_from_left;
    let mut visible_from_right;

    // Actually count visible trees
    for (y_index, y_axis) in composition.iter().enumerate() {
        for (x_index, x_axis) in y_axis.iter().enumerate() {
            // If we are not on the border
            if y_index != 0
                && y_index != composition.len() - 1
                && x_index != 0
                && x_index != y_axis.len() - 1
            {
                visible_from_top = 0;
                visible_from_bottom = 0;
                visible_from_left = 0;
                visible_from_right = 0;

                // We check trees from tree to top border
                for y_tree in (0..y_index).rev() {
                    if &composition[y_tree][x_index] < x_axis {
                        visible_from_top += 1;
                    } else {
                        visible_from_top += 1;
                        break;
                    }
                }
                // We check trees from tree to bottom border
                for y_tree in y_index + 1..composition.len() {
                    if &composition[y_tree][x_index] < x_axis {
                        visible_from_bottom += 1;
                    } else {
                        visible_from_bottom += 1;
                        break;
                    }
                }
                // We check trees from tree to left border
                for x_tree in (0..x_index).rev() {
                    if &composition[y_index][x_tree] < x_axis {
                        visible_from_left += 1;
                    } else {
                        visible_from_left += 1;
                        break;
                    }
                }
                // We check trees from tree to right border
                for x_tree in x_index + 1..composition[0].len() {
                    if &composition[y_index][x_tree] < x_axis {
                        visible_from_right += 1;
                    } else {
                        visible_from_right += 1;
                        break;
                    }
                }

                if max(1, visible_from_top)
                    * max(1, visible_from_bottom)
                    * max(1, visible_from_left)
                    * max(1, visible_from_right)
                    > result
                {
                    result = max(1, visible_from_top)
                        * max(1, visible_from_bottom)
                        * max(1, visible_from_left)
                        * max(1, visible_from_right);
                }
            }
        }
    }

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
        assert_eq!(aux_one(Path::new("input/test.txt")), 21);
        assert_eq!(aux_two(Path::new("input/test.txt")), 8);
    }
}
