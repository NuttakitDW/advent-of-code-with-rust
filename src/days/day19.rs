use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("Day 19 - Linen Layout!");
    let file_path = "src/days/inputs/day19.txt";
    let (towel_patterns, designs) = read_input(file_path);

    let possible_count = count_possible_designs(&towel_patterns, &designs);
    println!("Number of possible designs: {}", possible_count);

    let total_possible_ways = count_all_possible_ways(&towel_patterns, &designs);
    println!("Total possible ways to create all designs: {}", total_possible_ways);
}

fn read_input(file_path: &str) -> (HashSet<String>, Vec<String>) {
    let input = std::fs::read_to_string(file_path).expect("Failed to read input file");
    let mut lines = input.lines();
    let patterns = lines
        .next()
        .expect("Missing towel patterns")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs = lines
        .skip(1) // Skip the blank line
        .map(|s| s.trim().to_string())
        .collect();
    (patterns, designs)
}

// Part 1: Count the number of designs that can be created
fn count_possible_designs(towel_patterns: &HashSet<String>, designs: &[String]) -> usize {
    designs.iter().filter(|design| design_possible(towel_patterns, design, 0, &mut HashMap::new())).count()
}

fn design_possible(
    towel_patterns: &HashSet<String>,
    design: &str,
    index: usize,
    cache: &mut HashMap<usize, bool>,
) -> bool {
    if index == design.len() {
        return true;
    }

    if let Some(&result) = cache.get(&index) {
        return result;
    }

    for pattern in towel_patterns {
        if design[index..].starts_with(pattern) {
            if design_possible(towel_patterns, design, index + pattern.len(), cache) {
                cache.insert(index, true);
                return true;
            }
        }
    }

    cache.insert(index, false);
    false
}

// Part 2: Count the total number of ways to create all designs
fn count_all_possible_ways(towel_patterns: &HashSet<String>, designs: &[String]) -> usize {
    designs.iter().map(|design| design_possible_count(towel_patterns, design, &mut HashMap::new())).sum()
}

fn design_possible_count(
    towel_patterns: &HashSet<String>,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut result = 0;
    for pattern in towel_patterns {
        if design.starts_with(pattern) {
            result += design_possible_count(towel_patterns, &design[pattern.len()..], cache);
        }
    }

    cache.insert(design.to_string(), result);
    result
}
