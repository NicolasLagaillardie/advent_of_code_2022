use core::panic;
// use std::cmp::min;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut cubes = Vec::new();

    let mut common_faces = 0;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let coordinates = line.split(',').collect::<Vec<_>>();

        let x = coordinates[0].parse::<i32>().unwrap();
        let y = coordinates[1].parse::<i32>().unwrap();
        let z = coordinates[2].parse::<i32>().unwrap();

        if cubes.contains(&(x - 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x + 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y - 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y + 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z - 1)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z + 1)) {
            common_faces += 1;
        }

        cubes.push((x, y, z));
    }

    cubes.len() as i32 * 6 - common_faces * 2
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut cubes = Vec::new();

    let mut common_faces = 0;

    let mut potential_air_bubbles = Vec::new();

    let mut borders_x: Option<(i32, i32)> = None;
    let mut borders_y: Option<(i32, i32)> = None;
    let mut borders_z: Option<(i32, i32)> = None;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let coordinates = line.split(',').collect::<Vec<_>>();

        let x = coordinates[0].parse::<i32>().unwrap();
        let y = coordinates[1].parse::<i32>().unwrap();
        let z = coordinates[2].parse::<i32>().unwrap();

        // Update borders
        if let Some(coord_x) = borders_x {
            if x < coord_x.0 {
                borders_x = Some((x, coord_x.1));
            }

            if x > coord_x.1 {
                borders_x = Some((coord_x.0, x));
            }
        } else {
            borders_x = Some((x - 1, x + 1));
        }

        if let Some(coord_y) = borders_y {
            if y < coord_y.0 {
                borders_y = Some((y, coord_y.1));
            }

            if y > coord_y.1 {
                borders_y = Some((coord_y.0, y));
            }
        } else {
            borders_y = Some((y - 1, y + 1));
        }

        if let Some(coord_z) = borders_z {
            if z < coord_z.0 {
                borders_z = Some((z, coord_z.1));
            }

            if z > coord_z.1 {
                borders_z = Some((coord_z.0, z));
            }
        } else {
            borders_z = Some((z - 1, z + 1));
        }

        // Get common faces
        if cubes.contains(&(x - 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x + 1, y, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y - 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y + 1, z)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z - 1)) {
            common_faces += 1;
        }

        if cubes.contains(&(x, y, z + 1)) {
            common_faces += 1;
        }

        // Get corner of bubble of air
        // For air cube at (x, y - 1, z)
        if (cubes.contains(&(x - 1, y - 1, z)) && cubes.contains(&(x, y - 1, z - 1)))
            || (cubes.contains(&(x - 1, y - 1, z)) && cubes.contains(&(x, y - 1, z + 1)))
            || (cubes.contains(&(x + 1, y - 1, z)) && cubes.contains(&(x, y - 1, z - 1)))
            || (cubes.contains(&(x + 1, y - 1, z)) && cubes.contains(&(x, y - 1, z + 1)))
        {
            if !potential_air_bubbles.contains(&(x, y - 1, z)) {
                potential_air_bubbles.push((x, y - 1, z));
            }
        }

        // For air cube at (x, y + 1, z)
        if (cubes.contains(&(x - 1, y + 1, z)) && cubes.contains(&(x, y + 1, z - 1)))
            || (cubes.contains(&(x - 1, y + 1, z)) && cubes.contains(&(x, y + 1, z + 1)))
            || (cubes.contains(&(x + 1, y + 1, z)) && cubes.contains(&(x, y + 1, z - 1)))
            || (cubes.contains(&(x + 1, y + 1, z)) && cubes.contains(&(x, y + 1, z + 1)))
        {
            if !potential_air_bubbles.contains(&(x, y + 1, z)) {
                potential_air_bubbles.push((x, y + 1, z));
            }
        }

        // For air cube at (x - 1, y , z)
        if (cubes.contains(&(x - 1, y - 1, z)) && cubes.contains(&(x - 1, y, z - 1)))
            || (cubes.contains(&(x - 1, y - 1, z)) && cubes.contains(&(x - 1, y, z + 1)))
            || (cubes.contains(&(x - 1, y + 1, z)) && cubes.contains(&(x - 1, y, z - 1)))
            || (cubes.contains(&(x - 1, y + 1, z)) && cubes.contains(&(x - 1, y, z + 1)))
        {
            if !potential_air_bubbles.contains(&(x - 1, y, z)) {
                potential_air_bubbles.push((x - 1, y, z));
            }
        }

        // For air cube at (x + 1, y , z)
        if (cubes.contains(&(x + 1, y - 1, z)) && cubes.contains(&(x + 1, y, z - 1)))
            || (cubes.contains(&(x + 1, y - 1, z)) && cubes.contains(&(x + 1, y, z + 1)))
            || (cubes.contains(&(x + 1, y + 1, z)) && cubes.contains(&(x + 1, y, z - 1)))
            || (cubes.contains(&(x + 1, y + 1, z)) && cubes.contains(&(x + 1, y, z + 1)))
        {
            if !potential_air_bubbles.contains(&(x + 1, y, z)) {
                potential_air_bubbles.push((x + 1, y, z));
            }
        }

        // For air cube at (x, y , z - 1)
        if (cubes.contains(&(x - 1, y, z - 1)) && cubes.contains(&(x, y - 1, z - 1)))
            || (cubes.contains(&(x - 1, y, z - 1)) && cubes.contains(&(x, y + 1, z - 1)))
            || (cubes.contains(&(x + 1, y, z - 1)) && cubes.contains(&(x, y - 1, z - 1)))
            || (cubes.contains(&(x + 1, y, z - 1)) && cubes.contains(&(x, y + 1, z - 1)))
        {
            if !potential_air_bubbles.contains(&(x, y, z - 1)) {
                potential_air_bubbles.push((x, y, z - 1));
            }
        }

        // For air cube at (x, y , z + 1)
        if (cubes.contains(&(x - 1, y, z + 1)) && cubes.contains(&(x, y - 1, z + 1)))
            || (cubes.contains(&(x - 1, y, z + 1)) && cubes.contains(&(x, y + 1, z + 1)))
            || (cubes.contains(&(x + 1, y, z + 1)) && cubes.contains(&(x, y - 1, z + 1)))
            || (cubes.contains(&(x + 1, y, z + 1)) && cubes.contains(&(x, y + 1, z + 1)))
        {
            if !potential_air_bubbles.contains(&(x, y, z + 1)) {
                potential_air_bubbles.push((x, y, z + 1));
            }
        }

        cubes.push((x, y, z));
    }

    potential_air_bubbles.retain(|elt| !cubes.contains(elt));

    // Create all outer cubes
    let mut coords_x = borders_x.unwrap();
    let mut coords_y = borders_y.unwrap();
    let mut coords_z = borders_z.unwrap();

    coords_x = (coords_x.0 - 1, coords_x.1 + 1);
    coords_y = (coords_y.0 - 1, coords_y.1 + 1);
    coords_z = (coords_z.0 - 1, coords_z.1 + 1);

    let mut outer_cubes = vec![(coords_x.0, coords_y.0, coords_z.0)];

    let mut to_explore_outer_cubes = vec![(coords_x.0, coords_y.0, coords_z.0)];

    while !to_explore_outer_cubes.is_empty() {
        let mut temp_to_explore_outer_cubes = Vec::new();

        for (x, y, z) in to_explore_outer_cubes.iter() {
            // if x >= coords_x.0 && x <=
            if x - 1 > coords_x.0 {
                if !cubes.contains(&(*x - 1, *y, *z))
                    && !outer_cubes.contains(&(*x - 1, *y, *z))
                    && !temp_to_explore_outer_cubes.contains(&(*x - 1, *y, *z))
                {
                    temp_to_explore_outer_cubes.push((*x - 1, *y, *z));
                }
            }

            if x + 1 < coords_x.1 {
                if !cubes.contains(&(*x + 1, *y, *z))
                    && !outer_cubes.contains(&(*x + 1, *y, *z))
                    && !temp_to_explore_outer_cubes.contains(&(*x + 1, *y, *z))
                {
                    temp_to_explore_outer_cubes.push((*x + 1, *y, *z));
                }
            }

            if y - 1 > coords_y.0 {
                if !cubes.contains(&(*x, *y - 1, *z))
                    && !outer_cubes.contains(&(*x, *y - 1, *z))
                    && !temp_to_explore_outer_cubes.contains(&(*x, *y - 1, *z))
                {
                    temp_to_explore_outer_cubes.push((*x, *y - 1, *z));
                }
            }

            if y + 1 < coords_y.1 {
                if !cubes.contains(&(*x, *y + 1, *z))
                    && !outer_cubes.contains(&(*x, *y + 1, *z))
                    && !temp_to_explore_outer_cubes.contains(&(*x, *y + 1, *z))
                {
                    temp_to_explore_outer_cubes.push((*x, *y + 1, *z));
                }
            }

            if z - 1 > coords_z.0 {
                if !cubes.contains(&(*x, *y, *z - 1))
                    && !outer_cubes.contains(&(*x, *y, *z - 1))
                    && !temp_to_explore_outer_cubes.contains(&(*x, *y, *z - 1))
                {
                    temp_to_explore_outer_cubes.push((*x, *y, *z - 1));
                }
            }

            if z + 1 < coords_z.1 {
                if !cubes.contains(&(*x, *y, *z + 1))
                    && !outer_cubes.contains(&(*x, *y, *z + 1))
                    && !temp_to_explore_outer_cubes.contains(&(*x, *y, *z + 1))
                {
                    temp_to_explore_outer_cubes.push((*x, *y, *z + 1));
                }
            }
        }

        outer_cubes.append(&mut temp_to_explore_outer_cubes.clone());

        to_explore_outer_cubes = temp_to_explore_outer_cubes;
    }

    let mut hidden_faces = 0;

    for x in coords_x.0 + 1..coords_x.1 {
        for y in coords_y.0 + 1..coords_y.1 {
            for z in coords_z.0 + 1..coords_z.1 {
                if !outer_cubes.contains(&(x, y, z)) && !cubes.contains(&(x, y, z)) {
                    // Get common faces
                    if cubes.contains(&(x - 1, y, z)) {
                        hidden_faces += 1;
                    }

                    if cubes.contains(&(x + 1, y, z)) {
                        hidden_faces += 1;
                    }

                    if cubes.contains(&(x, y - 1, z)) {
                        hidden_faces += 1;
                    }

                    if cubes.contains(&(x, y + 1, z)) {
                        hidden_faces += 1;
                    }

                    if cubes.contains(&(x, y, z - 1)) {
                        hidden_faces += 1;
                    }

                    if cubes.contains(&(x, y, z + 1)) {
                        hidden_faces += 1;
                    }
                }
            }
        }
    }

    cubes.len() as i32 * 6 - common_faces * 2 - hidden_faces
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
        assert_eq!(aux_two(Path::new("input/test.txt")), 58);
    }
}
