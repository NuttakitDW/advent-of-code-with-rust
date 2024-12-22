use std::collections::HashMap;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self {
            position: (x, y),
            velocity: (vx, vy),
        }
    }

    fn update_position(&mut self) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(WIDTH);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(HEIGHT);
    }
}

fn calculate_safety_factor(robots: &mut Vec<Robot>, seconds: i32) -> i32 {
    let mut grid = HashMap::new();

    // Update positions after the specified time
    for robot in robots.iter_mut() {
        for _ in 0..seconds {
            robot.update_position();
        }
        *grid.entry(robot.position).or_insert(0) += 1;
    }

    // Count robots in each quadrant
    let mut quadrants = [0; 4];
    for (&(x, y), &count) in &grid {
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue; // Skip robots on the middle lines
        }
        if x < WIDTH / 2 && y < HEIGHT / 2 {
            quadrants[0] += count; // Top-left
        } else if x >= WIDTH / 2 && y < HEIGHT / 2 {
            quadrants[1] += count; // Top-right
        } else if x < WIDTH / 2 && y >= HEIGHT / 2 {
            quadrants[2] += count; // Bottom-left
        } else {
            quadrants[3] += count; // Bottom-right
        }
    }

    // Calculate the safety factor
    quadrants.iter().product()
}

pub fn run() {
    let input = vec![
        ((0, 4), (3, -3)),
        ((6, 3), (-1, -3)),
        ((10, 3), (-1, 2)),
        ((2, 0), (2, -1)),
        ((0, 0), (1, 3)),
        ((3, 0), (-2, -2)),
        ((7, 6), (-1, -3)),
        ((3, 0), (-1, -2)),
        ((9, 3), (2, 3)),
        ((7, 3), (-1, 2)),
        ((2, 4), (2, -3)),
        ((9, 5), (-3, -3)),
    ];

    // Initialize robots
    let mut robots: Vec<Robot> = input
        .into_iter()
        .map(|((x, y), (vx, vy))| Robot::new(x, y, vx, vy))
        .collect();

    // Calculate the safety factor after 100 seconds
    let safety_factor = calculate_safety_factor(&mut robots, 100);
    println!("Safety Factor: {}", safety_factor);
}
