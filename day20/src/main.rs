use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut list = Vec::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        list.push(line.parse::<i32>().unwrap());
    }

    println!("{list:?}");

    let mut index_moving_elt = 0;

    for _ in list.clone().iter() {
        let elt = list[index_moving_elt as usize];

        let index = if index_moving_elt + elt < 0 {
            let modulo = (index_moving_elt + elt).abs() % list.len() as i32;
            list.len() as i32 - modulo
        } else {
            index_moving_elt + elt % list.len() as i32
        };
        list.swap(index as usize, index_moving_elt as usize);

        index_moving_elt -= elt - 1;

        println!("{list:?}");
    }

    list[1000 % list.len()] + list[2000 % list.len()] + list[3000 % list.len()]
}

/// Function for part 02
fn aux_two(_file: &Path) -> i32 {
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 3);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 56 * 62);
    }
}
