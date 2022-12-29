use core::panic;
use std::cmp::max;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

impl Clone for Cost {
    fn clone(&self) -> Self {
        Cost {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl Clone for Blueprint {
    fn clone(&self) -> Self {
        Blueprint {
            ore_robot: self.ore_robot.clone(),
            clay_robot: self.clay_robot.clone(),
            obsidian_robot: self.obsidian_robot.clone(),
            geode_robot: self.geode_robot.clone(),
        }
    }
}

#[derive(Debug)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Clone for Resources {
    fn clone(&self) -> Self {
        Resources {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }
}

#[derive(Debug)]
struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Clone for Robots {
    fn clone(&self) -> Self {
        Robots {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }
}

fn aux_new_thread(
    blueprint: Blueprint,
    mut resources: Resources,
    robots: Robots,
    remaining_time: i32,
) -> i32 {
    if remaining_time <= 0 {
        return resources.geode;
    }

    let mut max_geodes = 0;

    let mut ore_robot_built = false;
    let mut clay_robot_built = false;
    let mut obsidian_robot_built = false;
    let mut geode_robot_built = false;

    for time in (1..=remaining_time).rev() {
        if ore_robot_built && clay_robot_built && obsidian_robot_built && geode_robot_built {
            return max_geodes;
        }

        if resources.ore >= blueprint.ore_robot.ore && !ore_robot_built {
            let mut new_resources = resources.clone();
            new_resources.ore -= blueprint.ore_robot.ore;

            new_resources.ore += robots.ore;
            new_resources.clay += robots.clay;
            new_resources.obsidian += robots.obsidian;
            new_resources.geode += robots.geode;

            let mut new_robots = robots.clone();
            new_robots.ore += 1;

            max_geodes = max(
                max_geodes,
                aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
            );

            ore_robot_built = true;
        }

        if resources.ore >= blueprint.clay_robot.ore && !clay_robot_built {
            let mut new_resources = resources.clone();
            new_resources.ore -= blueprint.clay_robot.ore;

            new_resources.ore += robots.ore;
            new_resources.clay += robots.clay;
            new_resources.obsidian += robots.obsidian;
            new_resources.geode += robots.geode;

            let mut new_robots = robots.clone();
            new_robots.clay += 1;

            max_geodes = max(
                max_geodes,
                aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
            );

            clay_robot_built = true;
        }

        if resources.ore >= blueprint.obsidian_robot.ore
            && resources.clay >= blueprint.obsidian_robot.clay
            && !obsidian_robot_built
        {
            let mut new_resources = resources.clone();
            new_resources.ore -= blueprint.obsidian_robot.ore;
            new_resources.clay -= blueprint.obsidian_robot.clay;

            new_resources.ore += robots.ore;
            new_resources.clay += robots.clay;
            new_resources.obsidian += robots.obsidian;
            new_resources.geode += robots.geode;

            let mut new_robots = robots.clone();
            new_robots.obsidian += 1;

            max_geodes = max(
                max_geodes,
                aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
            );

            obsidian_robot_built = true;
        }

        if resources.ore >= blueprint.geode_robot.ore
            && resources.obsidian >= blueprint.geode_robot.obsidian
            && !geode_robot_built
        {
            let mut new_resources = resources.clone();
            new_resources.ore -= blueprint.geode_robot.ore;
            new_resources.obsidian -= blueprint.geode_robot.obsidian;

            new_resources.ore += robots.ore;
            new_resources.clay += robots.clay;
            new_resources.obsidian += robots.obsidian;
            new_resources.geode += robots.geode;

            let mut new_robots = robots.clone();
            new_robots.geode += 1;

            max_geodes = max(
                max_geodes,
                aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
            );

            geode_robot_built = true;
        }

        resources.ore += robots.ore;
        resources.clay += robots.clay;
        resources.obsidian += robots.obsidian;
        resources.geode += robots.geode;
    }

    max_geodes
}

fn start_new_thread(
    blueprint_index: i32,
    blueprint: Blueprint,
    resources: Resources,
    robots: Robots,
    remaining_time: i32,
) -> JoinHandle<i32> {
    let blueprint_index_arc = Arc::new(Mutex::new(blueprint_index));
    let blueprint_arc = Arc::new(Mutex::new(blueprint));
    let resources_arc = Arc::new(Mutex::new(resources));
    let robots_arc = Arc::new(Mutex::new(robots));
    let remaining_time_arc = Arc::new(Mutex::new(remaining_time));

    thread::spawn(move || {
        let blueprint_index = *blueprint_index_arc.lock().unwrap();
        let blueprint = blueprint_arc.lock().unwrap();
        let mut resources = resources_arc.lock().unwrap();
        let robots = robots_arc.lock().unwrap();
        let remaining_time = *remaining_time_arc.lock().unwrap();

        println!("Started thread: {blueprint_index}");

        let mut max_geodes = 0;

        let mut ore_robot_built = false;
        let mut clay_robot_built = false;
        let mut obsidian_robot_built = false;
        let mut geode_robot_built = false;

        for time in (1..=remaining_time).rev() {
            if ore_robot_built && clay_robot_built && obsidian_robot_built && geode_robot_built {
                return max_geodes * blueprint_index;
            }

            if resources.ore >= blueprint.ore_robot.ore && !ore_robot_built {
                let mut new_resources = resources.clone();
                new_resources.ore -= blueprint.ore_robot.ore;

                new_resources.ore += robots.ore;
                new_resources.clay += robots.clay;
                new_resources.obsidian += robots.obsidian;
                new_resources.geode += robots.geode;

                let mut new_robots = robots.clone();
                new_robots.ore += 1;

                max_geodes = max(
                    max_geodes,
                    aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
                );

                ore_robot_built = true;
            }

            if resources.ore >= blueprint.clay_robot.ore && !clay_robot_built {
                let mut new_resources = resources.clone();
                new_resources.ore -= blueprint.clay_robot.ore;

                new_resources.ore += robots.ore;
                new_resources.clay += robots.clay;
                new_resources.obsidian += robots.obsidian;
                new_resources.geode += robots.geode;

                let mut new_robots = robots.clone();
                new_robots.clay += 1;

                max_geodes = max(
                    max_geodes,
                    aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
                );

                clay_robot_built = true;
            }

            if resources.ore >= blueprint.obsidian_robot.ore
                && resources.clay >= blueprint.obsidian_robot.clay
                && !obsidian_robot_built
            {
                let mut new_resources = resources.clone();
                new_resources.ore -= blueprint.obsidian_robot.ore;
                new_resources.clay -= blueprint.obsidian_robot.clay;

                new_resources.ore += robots.ore;
                new_resources.clay += robots.clay;
                new_resources.obsidian += robots.obsidian;
                new_resources.geode += robots.geode;

                let mut new_robots = robots.clone();
                new_robots.obsidian += 1;

                max_geodes = max(
                    max_geodes,
                    aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
                );

                obsidian_robot_built = true;
            }

            if resources.ore >= blueprint.geode_robot.ore
                && resources.obsidian >= blueprint.geode_robot.obsidian
                && !geode_robot_built
            {
                let mut new_resources = resources.clone();
                new_resources.ore -= blueprint.geode_robot.ore;
                new_resources.obsidian -= blueprint.geode_robot.obsidian;

                new_resources.ore += robots.ore;
                new_resources.clay += robots.clay;
                new_resources.obsidian += robots.obsidian;
                new_resources.geode += robots.geode;

                let mut new_robots = robots.clone();
                new_robots.geode += 1;

                max_geodes = max(
                    max_geodes,
                    aux_new_thread(blueprint.clone(), new_resources, new_robots, time - 1),
                );

                geode_robot_built = true;
            }

            resources.ore += robots.ore;
            resources.clay += robots.clay;
            resources.obsidian += robots.obsidian;
            resources.geode += robots.geode;
        }

        max_geodes * blueprint_index
    })
}

/// Function for part 01
fn aux_one(file: &Path) -> i32 {
    // Open file
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut threads = Vec::new();

    // Read file line by line, for part 01
    // Get composition of each monkey
    for line in reader.lines() {
        // Split line into direction and steps
        let line = line.unwrap();
        let line = line.trim();

        let blueprint_info = line.split("Blueprint ").collect::<Vec<_>>()[1];
        let blueprint_info = blueprint_info
            .split(": Each ore robot costs ")
            .collect::<Vec<_>>();

        let blueprint_index = blueprint_info[0].parse::<i32>().unwrap();

        let blueprint_info = blueprint_info[1]
            .split(" ore. Each clay robot costs ")
            .collect::<Vec<_>>();

        let cost_ore_robot = blueprint_info[0].parse::<i32>().unwrap();

        let blueprint_info = blueprint_info[1]
            .split(" ore. Each obsidian robot costs ")
            .collect::<Vec<_>>();

        let cost_clay_robot = blueprint_info[0].parse::<i32>().unwrap();

        let blueprint_info = blueprint_info[1]
            .split(" clay. Each geode robot costs ")
            .collect::<Vec<_>>();

        let cost_obsidian_robot = blueprint_info[0].split(" ore and ").collect::<Vec<_>>();
        let cost_obsidian_robot_ore = cost_obsidian_robot[0].parse::<i32>().unwrap();
        let cost_obsidian_robot_clay = cost_obsidian_robot[1].parse::<i32>().unwrap();

        let blueprint_info = blueprint_info[1].split(" obsidian.").collect::<Vec<_>>();

        let cost_geode_robot = blueprint_info[0].split(" ore and ").collect::<Vec<_>>();
        let cost_geode_robot_ore = cost_geode_robot[0].parse::<i32>().unwrap();
        let cost_geode_robot_obsidian = cost_geode_robot[1].parse::<i32>().unwrap();

        let blueprint = Blueprint {
            ore_robot: Cost {
                ore: cost_ore_robot,
                clay: 0,
                obsidian: 0,
            },
            clay_robot: Cost {
                ore: cost_clay_robot,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot: Cost {
                ore: cost_obsidian_robot_ore,
                clay: cost_obsidian_robot_clay,
                obsidian: 0,
            },
            geode_robot: Cost {
                ore: cost_geode_robot_ore,
                clay: 0,
                obsidian: cost_geode_robot_obsidian,
            },
        };

        let start_resources = Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };

        let start_robots = Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };

        threads.push(start_new_thread(
            blueprint_index,
            blueprint,
            start_resources,
            start_robots,
            24,
        ));
    }

    let mut result = 0;

    for thread in threads {
        result += thread.join().unwrap();
    }

    result
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
        assert_eq!(aux_one(Path::new("input/test.txt")), 33);
        // assert_eq!(aux_two(Path::new("input/test.txt")), 58);
    }
}
