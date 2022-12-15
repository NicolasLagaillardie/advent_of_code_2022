use core::panic;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path, row: i128) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut sensors = Vec::new();

    let mut beacons = Vec::new();

    let mut distances = Vec::new();

    let mut min_column = None;
    let mut max_column = None;

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();

        let line = line.split(": closest beacon is at ").collect::<Vec<&str>>();

        // Get coordinates of sensor
        let sensor = line[0].clone();
        let sensor = sensor.split("Sensor at ").collect::<Vec<&str>>()[1];
        let coordinates_sensor = sensor.split(", ").collect::<Vec<&str>>();
        let x_sensor = coordinates_sensor[0].split("=").collect::<Vec<&str>>()[1];
        let x_sensor = x_sensor.parse::<i128>().unwrap();
        let y_sensor = coordinates_sensor[1].split("=").collect::<Vec<&str>>()[1];
        let y_sensor = y_sensor.parse::<i128>().unwrap();

        // Get coordinates of beacon
        let beacon = line[1].clone();
        let coordinates_beacon = beacon.split(", ").collect::<Vec<&str>>();
        let x_beacon = coordinates_beacon[0].split("=").collect::<Vec<&str>>()[1];
        let x_beacon = x_beacon.parse::<i128>().unwrap();
        let y_beacon = coordinates_beacon[1].split("=").collect::<Vec<&str>>()[1];
        let y_beacon = y_beacon.parse::<i128>().unwrap();

        // Add coordinates to sensors and beacons
        sensors.push((x_sensor, y_sensor));
        beacons.push((x_beacon, y_beacon));

        // Include distance between sensor and beacon
        let distance = i128::abs(x_sensor - x_beacon) + i128::abs(y_sensor - y_beacon);
        distances.push(distance);

        // Get lower bound of coverage
        if let Some(mini) = min_column {
            if mini > min(x_sensor - distance, x_beacon) {
                min_column = Some(min(x_sensor - distance, x_beacon));
            }
        } else {
            min_column = Some(min(x_sensor - distance, x_beacon));
        }

        // Get upper bound of coverage
        if let Some(maxi) = max_column {
            if maxi < max(x_sensor + distance, x_beacon) {
                max_column = Some(max(x_sensor + distance, x_beacon));
            }
        } else {
            max_column = Some(max(x_sensor + distance, x_beacon));
        }
    }

    let min_column = min_column.unwrap();
    let max_column = max_column.unwrap();

    let mut closed_positions = Vec::new();

    for index in min_column..=max_column {
        'sensor_for: for (index_sensor, sensor) in sensors.iter().enumerate() {
            if (i128::abs(sensor.0 - index) + i128::abs(sensor.1 - row)) <= distances[index_sensor]
            {
                if !beacons.contains(&(index, row)) {
                    closed_positions.push((index, row));
                }
                break 'sensor_for;
            }
        }
    }

    closed_positions.len() as i128
}

/// Function for part 02
fn aux_two(file: &Path, is_test: bool) -> i128 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut sensors = Vec::new();

    let mut distances = Vec::new();

    let mut min_row = None;
    let mut max_row = None;

    let mut cells_to_test = Vec::new();

    let max_bound = if is_test { 20 } else { 4000000 };

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();

        let line = line.split(": closest beacon is at ").collect::<Vec<&str>>();

        // Get coordinates of sensor
        let sensor = line[0].clone();
        let sensor = sensor.split("Sensor at ").collect::<Vec<&str>>()[1];
        let coordinates_sensor = sensor.split(", ").collect::<Vec<&str>>();
        let x_sensor = coordinates_sensor[0].split("=").collect::<Vec<&str>>()[1];
        let x_sensor = x_sensor.parse::<i128>().unwrap();
        let y_sensor = coordinates_sensor[1].split("=").collect::<Vec<&str>>()[1];
        let y_sensor = y_sensor.parse::<i128>().unwrap();

        // Get coordinates of beacon
        let beacon = line[1].clone();
        let coordinates_beacon = beacon.split(", ").collect::<Vec<&str>>();
        let x_beacon = coordinates_beacon[0].split("=").collect::<Vec<&str>>()[1];
        let x_beacon = x_beacon.parse::<i128>().unwrap();
        let y_beacon = coordinates_beacon[1].split("=").collect::<Vec<&str>>()[1];
        let y_beacon = y_beacon.parse::<i128>().unwrap();

        // Add coordinates to sensors
        sensors.push((x_sensor, y_sensor));

        // Include distance between sensor and beacon
        let distance = i128::abs(x_sensor - x_beacon) + i128::abs(y_sensor - y_beacon);
        distances.push(distance);

        // Get lower bound of Y coverage
        if let Some(mini) = min_row {
            if mini > min(y_sensor - distance, y_beacon) {
                min_row = Some(min(y_sensor - distance, y_beacon));
            }
        } else {
            min_row = Some(min(y_sensor - distance, y_beacon));
        }

        // Get upper bound of Y coverage
        if let Some(maxi) = max_row {
            if maxi < max(y_sensor + distance, y_beacon) {
                max_row = Some(max(y_sensor + distance, y_beacon));
            }
        } else {
            max_row = Some(max(y_sensor + distance, y_beacon));
        }

        // Get cells right outside each sensor coverage area
        for x in 0..((distance + 1) * 4) {
            if x < (distance + 1) {
                let cell_x = x_sensor + (x % (distance + 1));
                if cell_x >= 0 && cell_x <= max_bound {
                    let cell_y = y_sensor + (x % (distance + 1)) - (distance + 1);
                    if cell_y >= 0 && cell_y <= max_bound {
                        cells_to_test.push((cell_x, cell_y));
                    }
                }
            } else if x < 2 * (distance + 1) {
                let cell_x = x_sensor + (distance + 1) - (x % (distance + 1));
                if cell_x >= 0 && cell_x <= max_bound {
                    let cell_y = y_sensor + (x % (distance + 1));
                    if cell_y >= 0 && cell_y <= max_bound {
                        cells_to_test.push((cell_x, cell_y));
                    }
                }
            } else if x < 3 * (distance + 1) {
                let cell_x = x_sensor - (x % (distance + 1));
                if cell_x >= 0 && cell_x <= max_bound {
                    let cell_y = y_sensor + (distance + 1) - (x % (distance + 1));
                    if cell_y >= 0 && cell_y <= max_bound {
                        cells_to_test.push((cell_x, cell_y));
                    }
                }
            } else if x < 4 * (distance + 1) {
                let cell_x = x_sensor - (distance + 1) + (x % (distance + 1));
                if cell_x >= 0 && cell_x <= max_bound {
                    let cell_y = y_sensor - (x % (distance + 1));
                    if cell_y >= 0 && cell_y <= max_bound {
                        cells_to_test.push((cell_x, cell_y));
                    }
                }
            }
        }
    }

    // Test each cell outside of each sensor coverage area
    for cell in cells_to_test.iter() {
        let mut is_available = true;

        'sensor_for: for (index_sensor, sensor) in sensors.iter().enumerate() {
            if (i128::abs(sensor.0 - cell.0) + i128::abs(sensor.1 - cell.1))
                <= distances[index_sensor]
            {
                is_available = false;
                break 'sensor_for;
            }
        }

        if is_available {
            return cell.0 * 4000000 + cell.1;
        }
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

    // Ask row
    let mut row = String::new();

    if choice.trim() == "1" {
        println!("Which row is tested");

        stdin().read_line(&mut row).expect("Failed to read input");
    }

    let result = match choice.trim() {
        "1" => aux_one(file, row.trim().parse::<i128>().unwrap()),
        "2" => aux_two(file, false),
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
        assert_eq!(aux_one(Path::new("input/test.txt"), 10), 26);
        assert_eq!(aux_two(Path::new("input/test.txt"), true), 56000011);
    }
}
