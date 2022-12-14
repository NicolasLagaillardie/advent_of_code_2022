use core::panic;
use std::cmp::min;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Process the matrix provided from the given cell
fn djikstra_process(
    current_cell: (usize, usize),
    mut matrix_paths: Vec<Vec<i32>>,
    matrix_heights: Vec<Vec<char>>,
    ending_cell: (usize, usize),
) -> i32 {
    let mut explored_cells = vec![current_cell];

    matrix_paths[current_cell.0][current_cell.1] = 0;

    while matrix_paths[ending_cell.0][ending_cell.1] == -1 {
        let mut temp = Vec::new();
        for cell in explored_cells.clone().iter() {
            // Check on their right
            if cell.0 < matrix_heights.len() - 1
                && (matrix_heights[cell.0 + 1][cell.1] as u32)
                    <= ((matrix_heights[cell.0][cell.1] as u32) + 1)
            {
                if matrix_paths[cell.0 + 1][cell.1] == -1 {
                    matrix_paths[cell.0 + 1][cell.1] = matrix_paths[cell.0][cell.1] + 1;
                } else {
                    matrix_paths[cell.0 + 1][cell.1] = min(
                        matrix_paths[cell.0 + 1][cell.1],
                        matrix_paths[cell.0][cell.1] + 1,
                    );
                }
                if !temp.contains(&(cell.0 + 1, cell.1)) {
                    temp.push((cell.0 + 1, cell.1));
                }
            }

            // Check on their left
            if cell.0 > 0
                && (matrix_heights[cell.0 - 1][cell.1] as u32)
                    <= ((matrix_heights[cell.0][cell.1] as u32) + 1)
            {
                if matrix_paths[cell.0 - 1][cell.1] == -1 {
                    matrix_paths[cell.0 - 1][cell.1] = matrix_paths[cell.0][cell.1] + 1;
                } else {
                    matrix_paths[cell.0 - 1][cell.1] = min(
                        matrix_paths[cell.0 - 1][cell.1],
                        matrix_paths[cell.0][cell.1] + 1,
                    );
                }
                if !temp.contains(&(cell.0 - 1, cell.1)) {
                    temp.push((cell.0 - 1, cell.1));
                }
            }

            // Check above
            if cell.1 < matrix_heights[0].len() - 1
                && (matrix_heights[cell.0][cell.1 + 1] as u32)
                    <= ((matrix_heights[cell.0][cell.1] as u32) + 1)
            {
                if matrix_paths[cell.0][cell.1 + 1] == -1 {
                    matrix_paths[cell.0][cell.1 + 1] = matrix_paths[cell.0][cell.1] + 1;
                } else {
                    matrix_paths[cell.0][cell.1 + 1] = min(
                        matrix_paths[cell.0][cell.1 + 1],
                        matrix_paths[cell.0][cell.1] + 1,
                    );
                }
                if !temp.contains(&(cell.0, cell.1 + 1)) {
                    temp.push((cell.0, cell.1 + 1));
                }
            }

            // Check down
            if cell.1 > 0
                && (matrix_heights[cell.0][cell.1 - 1] as u32)
                    <= ((matrix_heights[cell.0][cell.1] as u32) + 1)
            {
                if matrix_paths[cell.0][cell.1 - 1] == -1 {
                    matrix_paths[cell.0][cell.1 - 1] = matrix_paths[cell.0][cell.1] + 1;
                } else {
                    matrix_paths[cell.0][cell.1 - 1] = min(
                        matrix_paths[cell.0][cell.1 - 1],
                        matrix_paths[cell.0][cell.1] + 1,
                    );
                }
                if !temp.contains(&(cell.0, cell.1 - 1)) {
                    temp.push((cell.0, cell.1 - 1));
                }
            }
        }

        explored_cells = temp;
    }

    matrix_paths[ending_cell.0][ending_cell.1]
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut matrix_heights = Vec::new();

    let mut matrix_paths = Vec::new();

    let mut current_cell = (0, 0);

    let mut ending_cell = (0, 0);

    // Read file line by line, for part 01
    for (index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let mut line = line.chars().collect::<Vec<char>>();

        let weights = vec![-1; line.len()];

        if line.contains(&'S') {
            current_cell.0 = index_line;
            current_cell.1 = line.iter().position(|&r| r == 'S').unwrap();
            line[current_cell.1] = 'a';
        }

        if line.contains(&'E') {
            ending_cell.0 = index_line;
            ending_cell.1 = line.iter().position(|&r| r == 'E').unwrap();
            line[ending_cell.1] = 'z';
        }

        matrix_heights.push(line);

        matrix_paths.push(weights);
    }

    djikstra_process(current_cell, matrix_paths, matrix_heights, ending_cell)
}

/// Function for part 02
/// TODO: some issue with an offset of -1/+1
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut matrix_heights = Vec::new();

    let mut matrix_paths = Vec::new();

    let mut starting_cell = (0, 0);

    let mut ending_cell = (0, 0);

    // Read file line by line, for part 01
    for (index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let mut line = line.chars().collect::<Vec<char>>();

        let mut weights = vec![-1; line.len()];

        if line.contains(&'S') {
            starting_cell = (index_line, line.iter().position(|&r| r == 'S').unwrap());
            line[starting_cell.1] = 'a';
        }

        if line.contains(&'E') {
            ending_cell.0 = index_line;
            ending_cell.1 = line.iter().position(|&r| r == 'E').unwrap();
            line[ending_cell.1] = 'z';
        }

        if line.contains(&'a') {
            for (index, elt) in line.clone().iter().enumerate() {
                if elt == &'a' {
                    weights[index] = 0;
                }
            }
        }

        matrix_heights.push(line);

        matrix_paths.push(weights);
    }

    // -1 because some offset
    djikstra_process(starting_cell, matrix_paths, matrix_heights, ending_cell) - 1
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 31);
        assert_eq!(aux_two(Path::new("input/test.txt")), 29);
    }
}
