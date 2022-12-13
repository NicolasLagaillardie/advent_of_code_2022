use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut x = 1;

    let mut result = 0;

    let mut current_cycle = 0;

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.split(' ').collect::<Vec<&str>>();

        let instruction = line[0];

        match instruction {
            "noop" => {
                current_cycle += 1;

                if current_cycle == 20
                    || current_cycle == 60
                    || current_cycle == 100
                    || current_cycle == 140
                    || current_cycle == 180
                    || current_cycle == 220
                {
                    result += current_cycle * x;
                }
            }

            "addx" => {
                let steps = line[1].parse::<i32>().unwrap();

                current_cycle += 1;

                if current_cycle == 20
                    || current_cycle == 60
                    || current_cycle == 100
                    || current_cycle == 140
                    || current_cycle == 180
                    || current_cycle == 220
                {
                    result += current_cycle * x;
                }

                current_cycle += 1;

                if current_cycle == 20
                    || current_cycle == 60
                    || current_cycle == 100
                    || current_cycle == 140
                    || current_cycle == 180
                    || current_cycle == 220
                {
                    result += current_cycle * x;
                }

                x += steps;
            }

            elt => panic!("Error, expected noop or addx, found {elt}"),
        }
    }

    result
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut crt = vec![vec!['.'; 40]; 6];

    let mut current_cycle = 0;

    let mut sprite_position: i32 = 0;

    // Read file line by line, for part 01
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.split(' ').collect::<Vec<&str>>();

        let instruction = line[0];

        println!("sprite_position: {sprite_position}");

        match instruction {
            "noop" => {
                if sprite_position <= (current_cycle % 40)
                    && (current_cycle % 40) <= sprite_position + 2
                {
                    crt[(current_cycle / 40) as usize][(current_cycle % 40) as usize] = '#';
                }

                current_cycle += 1;
            }

            "addx" => {
                let steps = line[1].parse::<i32>().unwrap();

                if sprite_position <= (current_cycle % 40)
                    && (current_cycle % 40) <= sprite_position + 2
                {
                    crt[(current_cycle / 40) as usize][(current_cycle % 40) as usize] = '#';
                }

                current_cycle += 1;

                if sprite_position <= (current_cycle % 40)
                    && (current_cycle % 40) <= sprite_position + 2
                {
                    crt[(current_cycle / 40) as usize][(current_cycle % 40) as usize] = '#';
                }

                current_cycle += 1;

                sprite_position += steps;

                println!("steps: {steps}");
            }

            elt => panic!("Error, expected noop or addx, found {elt}"),
        }
    }

    println!("Line 0: {:?}", crt[0].iter().collect::<String>());
    println!("Line 1: {:?}", crt[1].iter().collect::<String>());
    println!("Line 2: {:?}", crt[2].iter().collect::<String>());
    println!("Line 3: {:?}", crt[3].iter().collect::<String>());
    println!("Line 4: {:?}", crt[4].iter().collect::<String>());
    println!("Line 5: {:?}", crt[5].iter().collect::<String>());

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
        assert_eq!(aux_one(Path::new("input/test.txt")), 13140);
        // No actual possible test, need to read letters on CRT
        // assert_eq!(aux_two(Path::new("input/test.txt")), 36);
    }
}
