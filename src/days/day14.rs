use std::fs;

pub fn run() {
    let file_path = "src/days/inputs/day14.txt";
    let robots = read_input(file_path);

    let width = 101;
    let height = 103;
    let time = 100;

    let positions = simulate_positions(&robots, width, height, time);
    let safety_factor = calculate_safety_factor(&positions, width, height);

    println!("Safety Factor after {} seconds: {}", time, safety_factor);
}

fn read_input(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos = parts[0]
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let vel = parts[1]
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            ((pos[0], pos[1]), (vel[0], vel[1]))
        })
        .collect()
}

fn simulate_positions(
    robots: &Vec<((i32, i32), (i32, i32))>,
    width: i32,
    height: i32,
    time: i32,
) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|&((x, y), (vx, vy))| {
            (
                ((x + vx * time) % width + width) % width, // Handle negative mod
                ((y + vy * time) % height + height) % height,
            )
        })
        .collect()
}

fn calculate_safety_factor(positions: &Vec<(i32, i32)>, width: i32, height: i32) -> i32 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0, 0, 0, 0]; // Q1, Q2, Q3, Q4

    for &(x, y) in positions {
        if x == mid_x || y == mid_y {
            continue; // Exclude robots in the middle rows or columns
        }

        if x < mid_x && y < mid_y {
            quadrants[0] += 1; // Q1
        } else if x > mid_x && y < mid_y {
            quadrants[1] += 1; // Q2
        } else if x < mid_x && y > mid_y {
            quadrants[2] += 1; // Q3
        } else if x > mid_x && y > mid_y {
            quadrants[3] += 1; // Q4
        }
    }

    quadrants.iter().product()
}
