use core::panic;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

/// Function for part 01
fn aux_one(file: &Path, row: i32) -> i32 {
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
        let x_sensor = x_sensor.parse::<i32>().unwrap();
        let y_sensor = coordinates_sensor[1].split("=").collect::<Vec<&str>>()[1];
        let y_sensor = y_sensor.parse::<i32>().unwrap();

        // Get coordinates of beacon
        let beacon = line[1].clone();
        let coordinates_beacon = beacon.split(", ").collect::<Vec<&str>>();
        let x_beacon = coordinates_beacon[0].split("=").collect::<Vec<&str>>()[1];
        let x_beacon = x_beacon.parse::<i32>().unwrap();
        let y_beacon = coordinates_beacon[1].split("=").collect::<Vec<&str>>()[1];
        let y_beacon = y_beacon.parse::<i32>().unwrap();

        // Add coordinates to sensors and beacons
        sensors.push((x_sensor, y_sensor));
        beacons.push((x_beacon, y_beacon));

        // Include distance between sensor and beacon
        let distance = i32::abs(x_sensor - x_beacon) + i32::abs(y_sensor - y_beacon);
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

    println!("sensors: {:?}", sensors);
    println!("beacons: {:?}", beacons);

    let min_column = min_column.unwrap();
    let max_column = max_column.unwrap();

    println!("min_column: {:?}", min_column);
    println!("max_column: {:?}", max_column);

    let mut closed_positions = Vec::new();

    for index in min_column..=max_column {
        if !beacons.contains(&(index, row)) {
            'sensor_for: for (index_sensor, sensor) in sensors.iter().enumerate() {
                if (i32::abs(sensor.0 - index) + i32::abs(sensor.1 - row))
                    <= distances[index_sensor]
                {
                    closed_positions.push((index, row));
                    break 'sensor_for;
                }
            }
        }
    }

    println!("closed_positions: {:?}", closed_positions);

    // Include both external columns and zero,
    // Exclude available positions
    // And beacons already present
    closed_positions.len() as i32
}

/// Function for part 02
fn aux_two(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let _reader = BufReader::new(file);

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
    println!("Which row is tested");

    let mut row = String::new();

    stdin().read_line(&mut row).expect("Failed to read input");

    let result = match choice.trim() {
        "1" => aux_one(file, row.trim().parse::<i32>().unwrap()),
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
        assert_eq!(aux_one(Path::new("input/test.txt"), 10), 26);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 93);
    }
}
