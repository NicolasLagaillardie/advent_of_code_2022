use core::panic;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

struct Valve {
    name: String,
    flow_rate: i32,
    connected_valves: Vec<String>,
}

impl Valve {
    fn new() -> Self {
        Valve {
            name: "".to_string(),
            flow_rate: 0,
            connected_valves: Vec::new(),
        }
    }
}

/// Function for part 01
fn aux_one(file: &Path) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut valves = Vec::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let line = line.split("Valve ").collect::<Vec<&str>>()[1];

        let line = line.split(" has flow rate=").collect::<Vec<&str>>();
        let name = line[0].to_string();

        let line = line[1]
            .split("; tunnels lead to valves ")
            .collect::<Vec<&str>>();
        let flow_rate = line[0].parse::<i32>().unwrap();

        let connected_valves = line[1].split(",").collect::<Vec<&str>>();
        let connected_valves = connected_valves.iter().map(|elt| elt.to_string()).collect::<Vec<String>>();

        let mut valve = Valve::new();
        valve.name = name;
        valve.flow_rate = flow_rate;
        valve.connected_valves = connected_valves;

        valves.push(valve);
    }

    0
}

/// Function for part 02
fn aux_two(_file: &Path) -> i128 {
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 1651);
        // assert_eq!(aux_two(Path::new("input/test.txt"), true), 56000011);
    }
}
