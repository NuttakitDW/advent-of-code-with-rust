use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("src/days/inputs/day3.txt").expect("Failed to read input");

    // Part 1
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    // Part 2
    part2();
}

fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

use std::fs;

/// Finds all positions of a word in the content
fn get_positions_of_word(content: &str, word: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut previous_position = 0;

    while let Some(position) = content[previous_position..].find(word) {
        positions.push(previous_position + position);
        previous_position += position + 1;
    }

    positions
}

/// Finds the next position greater than `start` in a list of positions
fn get_next_position(start: usize, positions: &[usize]) -> Option<usize> {
    positions.iter().copied().find(|&position| position > start)
}

/// Filters instructions based on `do()` and `don't()` rules
fn get_enabled_instructions(content: &str) -> String {
    let chars: Vec<char> = content.chars().collect();
    let mut result = Vec::new();
    let mut write = true;

    let dont_positions = get_positions_of_word(content, "don't()");
    let do_positions = get_positions_of_word(content, "do()");

    let mut next_dont = get_next_position(0, &dont_positions);
    let mut next_do = None;

    for (i, &ch) in chars.iter().enumerate() {
        if Some(i) == next_dont {
            write = false;
            next_do = get_next_position(i, &do_positions);
        } else if Some(i) == next_do {
            write = true;
            next_dont = get_next_position(i, &dont_positions);
        }

        if write {
            result.push(ch);
        }
    }

    result.iter().collect()
}

/// Processes the content and computes the total result
fn get_total(content: &str) -> i64 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(content)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

pub fn part2() {
    let file_path = "src/days/inputs/day3.txt";
    let content = fs::read_to_string(file_path).expect("Failed to read input file");

    // Apply `do()` and `don't()` filtering logic
    let enabled_instructions = get_enabled_instructions(&content);

    // Calculate the total from valid `mul` instructions
    let total = get_total(&enabled_instructions);

    println!("Part 2 Total: {}", total);
}
