use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<i32>,
    operation: (String, String, String),
    test: i32,
    if_true: i32,
    if_false: i32,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            starting_items: Vec::new(),
            operation: ("".to_string(), "".to_string(), "".to_string()),
            test: 0,
            if_true: 0,
            if_false: 0,
        }
    }
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Monkey {
            starting_items: self.starting_items.clone(),
            operation: self.operation.clone(),
            test: self.test.clone(),
            if_true: self.if_true.clone(),
            if_false: self.if_false.clone(),
        }
    }
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    // Store composition of every monkey
    // Starting items = Vec<String>
    // Operation = String
    // (Test, True, False) = (String, String, String)
    let mut monkeys = HashMap::<i32, Monkey>::new();

    // Stores current parsed monkey
    let mut current_monkey = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for (_index_line, line) in reader.lines().enumerate() {
        // Split line into direction and steps
        let line = line.unwrap();

        if line.contains("Monkey") {
            let line = line.split("Monkey ").collect::<Vec<&str>>();
            let line = line.iter().cloned().collect::<String>();
            let line = line.split(":").collect::<Vec<&str>>();

            current_monkey = line[0].parse::<i32>().unwrap();

            let monkey = Monkey::new();

            monkeys.insert(line[0].parse::<i32>().unwrap(), monkey);
        } else if line.contains("Starting items: ") {
            let line = line.split("Starting items: ").collect::<Vec<&str>>();
            let line = line.iter().cloned().collect::<String>();
            let line = line.split(' ').collect::<Vec<&str>>();
            let line = line.iter().cloned().collect::<String>();
            let line = line.split(',').collect::<Vec<&str>>();

            let monkey = monkeys.get_mut(&current_monkey).unwrap();

            for elt in line {
                monkey.starting_items.push(elt.parse::<i32>().unwrap());
            }
        } else if line.contains("Operation: new = ") {
            let line = line.split("Operation: new = ").collect::<Vec<&str>>();
            let line = line.iter().cloned().collect::<String>();
            let line = line.trim();

            let line = line.split(' ').collect::<Vec<&str>>();

            let mut monkey = monkeys.get_mut(&current_monkey).unwrap();

            monkey.operation = (
                line[0].to_string(),
                line[1].to_string(),
                line[2].to_string(),
            );
        } else if line.contains("Test: divisible by ") {
            let line = line.split("Test: divisible by ").collect::<Vec<&str>>();

            let mut monkey = monkeys.get_mut(&current_monkey).unwrap();

            monkey.test = line[1].parse::<i32>().unwrap();
        } else if line.contains("If true: throw to monkey ") {
            let line = line
                .split("If true: throw to monkey ")
                .collect::<Vec<&str>>();

            let mut monkey = monkeys.get_mut(&current_monkey).unwrap();

            monkey.if_true = line[1].parse::<i32>().unwrap();
        } else if line.contains("If false: ") {
            let line = line
                .split("If false: throw to monkey ")
                .collect::<Vec<&str>>();

            let mut monkey = monkeys.get_mut(&current_monkey).unwrap();

            monkey.if_false = line[1].parse::<i32>().unwrap();
        }
    }

    println!("monkeys: {:?}", monkeys);

    let mut holded_objects = vec![0; monkeys.len()];

    // For each round of the 20 rounds
    for index in 0..20 {
        // For each monkey, in the ascending order
        for monkey_index in 0..monkeys.len() {
            let mut clone_monkey = monkeys.clone();

            let monkey = monkeys.get_mut(&(monkey_index as i32)).unwrap();

            if monkey.operation.1.contains('+') {
                if monkey.operation.0.contains("old") {
                    if monkey.operation.2.contains("old") {
                        for elt in &monkey.starting_items {
                            let test = (elt + elt) / 3;

                            holded_objects[monkey_index] += 1;

                            if test % monkey.test == 0 {
                                let monkey_receive = clone_monkey.get_mut(&monkey.if_true).unwrap();

                                monkey_receive.starting_items.push(test);
                            } else {
                                let monkey_receive =
                                    clone_monkey.get_mut(&monkey.if_false).unwrap();

                                monkey_receive.starting_items.push(test);
                            }
                        }
                    } else {
                        let extract_right = monkey.operation.2.parse::<i32>().unwrap();
                        for elt in &monkey.starting_items {
                            let test = (elt + extract_right) / 3;

                            holded_objects[monkey_index] += 1;

                            if test % monkey.test == 0 {
                                let monkey_receive = clone_monkey.get_mut(&monkey.if_true).unwrap();

                                monkey_receive.starting_items.push(test);
                            } else {
                                let monkey_receive =
                                    clone_monkey.get_mut(&monkey.if_false).unwrap();

                                monkey_receive.starting_items.push(test);
                            }
                        }
                    }
                }
            } else if monkey.operation.1.contains('*') {
                if monkey.operation.0.contains("old") {
                    if monkey.operation.2.contains("old") {
                        for elt in &monkey.starting_items {
                            let test = (elt * elt) / 3;

                            holded_objects[monkey_index] += 1;

                            if test % monkey.test == 0 {
                                let monkey_receive = clone_monkey.get_mut(&monkey.if_true).unwrap();

                                monkey_receive.starting_items.push(test);
                            } else {
                                let monkey_receive =
                                    clone_monkey.get_mut(&monkey.if_false).unwrap();

                                monkey_receive.starting_items.push(test);
                            }
                        }
                    } else {
                        let extract_right = monkey.operation.2.parse::<i32>().unwrap();
                        for elt in &monkey.starting_items {
                            let test = (elt * extract_right) / 3;

                            holded_objects[monkey_index] += 1;

                            if test % monkey.test == 0 {
                                let monkey_receive = clone_monkey.get_mut(&monkey.if_true).unwrap();

                                monkey_receive.starting_items.push(test);
                            } else {
                                let monkey_receive =
                                    clone_monkey.get_mut(&monkey.if_false).unwrap();

                                monkey_receive.starting_items.push(test);
                            }
                        }
                    }
                }
            } else {
                panic!("Error, expected + or *, found {}", monkey.operation.1);
            }

            let monkey = clone_monkey.get_mut(&(monkey_index as i32)).unwrap();

            monkey.starting_items = Vec::new();

            monkeys = clone_monkey;
        }

        println!("monkeys at index {index}: {:?}", monkeys);
        println!("");
        println!("");
    }

    println!("monkeys at the end: {:?}", monkeys);

    holded_objects.sort();

    println!(
        "holded_objects at the end: {:?}",
        holded_objects.iter().rev()
    );

    let maxi_0 = holded_objects.pop().unwrap();
    let maxi_1 = holded_objects.pop().unwrap();

    println!("result: {:?}", maxi_0 * maxi_1);

    maxi_0 * maxi_1
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 10605);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 36);
    }
}
