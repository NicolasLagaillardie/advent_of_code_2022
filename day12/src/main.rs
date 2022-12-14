use core::panic;
use std::cmp::min;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

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

    println!("current_cell: {:?}", current_cell);

    println!("ending_cell: {:?}", ending_cell);

    let length = matrix_heights.len() - 1;
    let height = matrix_heights[0].len() - 1;

    println!("size: {:?}", length * height);

    let mut explored_cells = vec![current_cell];

    matrix_paths[current_cell.0][current_cell.1] = 0;

    while matrix_paths[ending_cell.0][ending_cell.1] == -1 {
        let mut temp = Vec::new();
        for cell in explored_cells.clone().iter() {
            // Check on their right
            if cell.0 < length
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
            if cell.1 < height
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

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.split(' ').collect::<Vec<&str>>();
    }

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
        assert_eq!(aux_one(Path::new("input/test.txt")), 31);
        // No actual possible test, need to read letters on CRT
        // assert_eq!(aux_two(Path::new("input/test.txt")), 36);
    }
}
