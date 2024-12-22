use std::fs;
use regex::Regex;

pub fn run() {
    println!("Day 13 - Claw Contraption!!");

    let file_path = "src/days/inputs/day13.txt";
    let machines = read_input(file_path);

    let result = solve_machines(&machines);
    println!("Fewest tokens to win all possible prizes: {}", result);
}

struct Machine {
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_target: i64,
    y_target: i64,
}

fn read_input(file_path: &str) -> Vec<Machine> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut machines = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        let x_a = parse_value(&line, "X+");
        let y_a = parse_value(&line, "Y+");

        let line_b = lines.next().unwrap();
        let x_b = parse_value(&line_b, "X+");
        let y_b = parse_value(&line_b, "Y+");

        let line_prize = lines.next().unwrap();
        let x_target = parse_value(&line_prize, "X=");
        let y_target = parse_value(&line_prize, "Y=");

        machines.push(Machine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_target,
            y_target,
        });
    }

    machines
}

fn parse_value(line: &str, key: &str) -> i64 {
    let re = Regex::new(&format!(r"{}(\d+)", key)).unwrap(); // Matches key followed by digits
    re.captures(line)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i64>().ok())
        .expect(&format!("Invalid input format: {}", line))
}


fn solve_machines(machines: &[Machine]) -> i64 {
    let mut total_cost = 0;

    for machine in machines {
        if let Some(cost) = find_min_cost(machine) {
            total_cost += cost;
        }
    }

    total_cost
}

fn find_min_cost(machine: &Machine) -> Option<i64> {
    let mut min_cost = None;

    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * machine.x_a + b * machine.x_b;
            let y = a * machine.y_a + b * machine.y_b;

            if x == machine.x_target && y == machine.y_target {
                let cost = a * 3 + b;
                if min_cost.is_none() || cost < min_cost.unwrap() {
                    min_cost = Some(cost);
                }
            }
        }
    }

    min_cost
}
