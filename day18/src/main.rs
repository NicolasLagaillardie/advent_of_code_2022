use core::panic;
// use std::cmp::min;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut moves = Vec::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        moves = line.chars().collect();
    }

    // Starting coordinates of each rock
    // As well as its height and width
    let rock_patterns = vec![
        // The horizontal line
        (vec![(0, 3), (0, 4), (0, 5), (0, 6)], 1, 4),
        // The cross
        (vec![(0, 4), (1, 3), (1, 4), (1, 5), (2, 4)], 3, 3),
        // The reversed L
        (vec![(0, 5), (1, 5), (2, 3), (2, 4), (2, 5)], 3, 3),
        // The vertical line
        (vec![(0, 3), (1, 3), (2, 3), (3, 3)], 4, 1),
        // The square
        (vec![(0, 3), (0, 4), (1, 3), (1, 4)], 2, 2),
    ];

    let mut current_tower = vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1]];

    let mut current_move = 0;

    for rock in 0..2022 {
        let current_rock_compo = rock_patterns[rock % rock_patterns.len()].clone();
        let mut current_rock = current_rock_compo.0;

        for _ in 0..(current_rock_compo.1 + 3) {
            current_tower.insert(0, vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
        }

        'level_two: loop {
            if current_move % 2 == 0 {
                match moves[(current_move / 2) % moves.len()] {
                    '<' => {
                        let mut can_move = true;

                        'level_three: for cell in current_rock.clone() {
                            if current_tower[cell.0][cell.1 - 1] == 1 {
                                can_move = false;
                                break 'level_three;
                            }
                        }

                        if can_move {
                            current_rock =
                                current_rock.iter().map(|elt| (elt.0, elt.1 - 1)).collect();
                        }
                    }
                    '>' => {
                        let mut can_move = true;

                        'level_three: for cell in current_rock.clone() {
                            if current_tower[cell.0][cell.1 + 1] == 1 {
                                can_move = false;
                                break 'level_three;
                            }
                        }

                        if can_move {
                            current_rock =
                                current_rock.iter().map(|elt| (elt.0, elt.1 + 1)).collect();
                        }
                    }
                    elt => {
                        panic!("Error, expected < or >, found: {:?}", elt)
                    }
                }
            } else {
                let mut can_move = true;

                'level_three: for cell in current_rock.clone() {
                    if current_tower[cell.0 + 1][cell.1] == 1 {
                        can_move = false;
                        break 'level_three;
                    }
                }

                if can_move {
                    current_rock = current_rock.iter().map(|elt| (elt.0 + 1, elt.1)).collect();
                } else {
                    for cell in current_rock {
                        current_tower[cell.0][cell.1] = 1;
                    }
                    while current_tower[0] == vec![1, 0, 0, 0, 0, 0, 0, 0, 1] {
                        current_tower.remove(0);
                    }
                    break 'level_two;
                }
            }

            current_move += 1;
        }

        current_move += 1;
    }

    current_tower.len() as i128 - 1
}

/// Function for part 02
fn aux_two(file: &Path) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut moves = Vec::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        moves = line.chars().collect();
    }

    // Starting coordinates of each rock
    // As well as its height and width
    let rock_patterns = vec![
        // The horizontal line
        (vec![(0, 3), (0, 4), (0, 5), (0, 6)], 1, 4),
        // The cross
        (vec![(0, 4), (1, 3), (1, 4), (1, 5), (2, 4)], 3, 3),
        // The reversed L
        (vec![(0, 5), (1, 5), (2, 3), (2, 4), (2, 5)], 3, 3),
        // The vertical line
        (vec![(0, 3), (1, 3), (2, 3), (3, 3)], 4, 1),
        // The square
        (vec![(0, 3), (0, 4), (1, 3), (1, 4)], 2, 2),
    ];

    let mut current_tower = vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1]];

    let mut current_move = 0;

    // let mut min_depth = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];

    // let mut offset = 0;

    let mut current_max_depth_cycle = 1;

    for rock in 0..1000000000000 {
        let current_rock_compo = rock_patterns[rock % rock_patterns.len()].clone();
        let mut current_rock = current_rock_compo.0.clone();

        for _ in 0..(current_rock_compo.1 + 3) {
            current_tower.insert(0, vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
            // min_depth = min_depth.iter().map(|elt| elt + 1).collect();
        }

        'level_two: loop {
            if current_move % 2 == 0 {
                match moves[(current_move / 2) % moves.len()] {
                    '<' => {
                        let mut can_move = true;

                        'level_three: for cell in current_rock.clone() {
                            if current_tower[cell.0][cell.1 - 1] == 1 {
                                can_move = false;
                                break 'level_three;
                            }
                        }

                        if can_move {
                            current_rock =
                                current_rock.iter().map(|elt| (elt.0, elt.1 - 1)).collect();
                        }
                    }
                    '>' => {
                        let mut can_move = true;

                        'level_three: for cell in current_rock.clone() {
                            if current_tower[cell.0][cell.1 + 1] == 1 {
                                can_move = false;
                                break 'level_three;
                            }
                        }

                        if can_move {
                            current_rock =
                                current_rock.iter().map(|elt| (elt.0, elt.1 + 1)).collect();
                        }
                    }
                    elt => {
                        panic!("Error, expected < or >, found: {:?}", elt)
                    }
                }
            } else {
                let mut can_move = true;

                'level_three: for cell in current_rock.clone() {
                    if current_tower[cell.0 + 1][cell.1] == 1 {
                        can_move = false;
                        break 'level_three;
                    }
                }

                if can_move {
                    current_rock = current_rock.iter().map(|elt| (elt.0 + 1, elt.1)).collect();
                } else {
                    for cell in current_rock {
                        current_tower[cell.0][cell.1] = 1;
                        // min_depth[cell.1] = min(min_depth[cell.1], cell.0);
                    }
                    while current_tower[0] == vec![1, 0, 0, 0, 0, 0, 0, 0, 1] {
                        current_tower.remove(0);
                        // min_depth = min_depth.iter().map(|elt| elt - 1).collect();
                    }
                    break 'level_two;
                }
            }

            current_move += 1;
        }

        current_move += 1;

        // let inaccessible = min_depth[1..(min_depth.len() - 1)].iter().max().unwrap();

        // if inaccessible < &current_tower.len() {
        //     let to_remove = &current_tower.len() - inaccessible - 1;
        //     offset += to_remove;

        //     println!("previous current_tower:");
        //     for level in current_tower.iter() {
        //         println!("{:?}", level);
        //     }

        //     current_tower = current_tower[0..(current_tower.len() - to_remove)].to_vec();
        //     min_depth[0] -= to_remove;
        //     min_depth[8] -= to_remove;

        //     println!("current_tower:");
        //     for level in current_tower.iter() {
        //         println!("{:?}", level);
        //     }
        //     println!("min_depth: {:?}", min_depth);
        // }

        if current_tower.len() > current_max_depth_cycle + 1
            && current_tower[..current_tower.len() - current_max_depth_cycle]
                .contains(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1])
        {
            // for level in current_tower.iter() {
            //     println!("{level:?}");
            // }

            println!("Current rock: {:?}", current_rock_compo.0);
            println!("Line at: {:?}", current_tower.len());
            current_max_depth_cycle = current_tower.len();

            // Square in 15 and in 220 and in 675
            // Horizontal line in 257 and in 270 and in 373
        }

        if rock > 10000 {
            panic!("Stop");
        }

        // if current_tower.len() > 10 {
        //     if current_tower.len() % 2 == 1 {
        //         let up = &current_tower[0..((current_tower.len() - 1) / 2)];
        //         let down = &current_tower[((current_tower.len() - 1) / 2)..current_tower.len() - 1];

        //         if up == down {
        //             panic!("Found the culprit for size {}", current_tower.len());
        //         }
        //     }
        // }
    }

    // current_tower.len() as i128 - 1 + offset as i128
    current_tower.len() as i128 - 1
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 64);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 1514285714288);
    }
}
