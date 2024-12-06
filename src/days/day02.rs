use std::fs;

pub fn run() {
    // Read the input file
    let reports = read_reports("src/days/inputs/day2.txt");

    print!("Day 2!!\n");

    // Part 1: Count safe reports
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    println!("Part 1: The number of safe reports is: {}", safe_count);

    // Part 2: Count safe reports with the Problem Dampener
    let safe_with_dampener_count = reports.iter().filter(|report| is_safe_with_dampener(report)).count();
    println!(
        "Part 2: The number of safe reports with the Problem Dampener is: {}",
        safe_with_dampener_count
    );
}

// Read the input file and parse it into a list of reports
fn read_reports(filename: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(filename).expect("Failed to read file");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect()
        })
        .collect()
}

// Check if a report is safe
fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false; // A single level or empty report cannot be safe
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];

        // Check the difference condition
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Update the trends
        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
    }

    // The report is safe if it's either fully increasing or fully decreasing
    is_increasing || is_decreasing
}

// Check if a report is safe with the Problem Dampener
fn is_safe_with_dampener(report: &[i32]) -> bool {
    // If the report is already safe, return true
    if is_safe(report) {
        return true;
    }

    // Try removing each level and check if the modified report is safe
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i); // Remove the level at index `i`
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}
