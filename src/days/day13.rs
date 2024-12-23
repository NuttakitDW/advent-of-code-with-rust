use std::collections::HashSet;
use std::fs;

pub fn run() {
    println!("Day 13 - Claw Contraption (Optimized Part 2)!");
    let file_path = "src/days/inputs/day13.txt";

    // Read and parse the input
    let machines = read_input(file_path);

    // Apply correction for Part 2
    let corrected_machines = apply_prize_correction(&machines, 10_000_000_000_000);

    // Solve Part 2
    let (prizes_won_part2, total_cost_part2) = calculate_tokens_optimized(&corrected_machines);
    println!("Part 2 (Optimized): Prizes won: {}", prizes_won_part2);
    println!("Part 2 (Optimized): Total cost: {}", total_cost_part2);
}

fn read_input(file_path: &str) -> HashSet<((i64, i64), (i64, i64), (i64, i64))> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut machines = HashSet::new();

    for chunk in input.split("\n\n") {
        let lines: Vec<&str> = chunk.lines().collect();

        let button_a = parse_coordinates(lines[0], "Button A: ");
        let button_b = parse_coordinates(lines[1], "Button B: ");
        let prize = parse_prize(lines[2]);

        machines.insert((button_a, button_b, prize));
    }

    machines
}

fn parse_coordinates(line: &str, prefix: &str) -> (i64, i64) {
    let line = line.strip_prefix(prefix).expect("Invalid prefix in line");
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].replace("X+", "").parse::<i64>().expect("Invalid X value");
    let y = parts[1].replace("Y+", "").parse::<i64>().expect("Invalid Y value");
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let line = line.strip_prefix("Prize: ").expect("Invalid prefix in line");
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].replace("X=", "").parse::<i64>().expect("Invalid X value");
    let y = parts[1].replace("Y=", "").parse::<i64>().expect("Invalid Y value");
    (x, y)
}

fn apply_prize_correction(
    machines: &HashSet<((i64, i64), (i64, i64), (i64, i64))>,
    correction: i64,
) -> HashSet<((i64, i64), (i64, i64), (i64, i64))> {
    machines
        .iter()
        .map(|&(button_a, button_b, (px, py))| (button_a, button_b, (px + correction, py + correction)))
        .collect()
}

fn calculate_tokens_optimized(
    machines: &HashSet<((i64, i64), (i64, i64), (i64, i64))>,
) -> (usize, i64) {
    let mut prizes_won = 0;
    let mut total_cost = 0;

    for &((ax, ay), (bx, by), (px, py)) in machines {
        if let Some((x, y, cost)) = solve_algebraically(ax, ay, bx, by, px, py) {
            prizes_won += 1;
            total_cost += cost;
        }
    }

    (prizes_won, total_cost)
}

fn solve_algebraically(
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
) -> Option<(i64, i64, i64)> {
    // Use modular arithmetic to eliminate x or y
    let determinant = ax * by - ay * bx;

    if determinant == 0 {
        return None; // No unique solution
    }

    // Find a particular solution using the determinant
    let x_part = (px * by - py * bx) / determinant;
    let y_part = (py * ax - px * ay) / determinant;

    // Ensure the solution is integer and satisfies both equations
    if (px - ax * x_part - bx * y_part) == 0 && (py - ay * x_part - by * y_part) == 0 {
        let cost = 3 * x_part + y_part;
        if x_part >= 0 && y_part >= 0 {
            return Some((x_part, y_part, cost));
        }
    }

    None
}
