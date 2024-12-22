use std::collections::VecDeque;
use std::fs;
use std::collections::HashMap;


pub fn run() {
    println!("Day 11 - Plutonian Pebbles!!");
    let file_path = "src/days/inputs/day11.txt";
    let initial_stones = read_input(file_path);
    let blinks = 25;

    let total_stones = simulate_stones(&initial_stones, blinks);
    println!("Total stones after {} blinks: {}", blinks, total_stones);

    println!("Day 11 - Plutonian Pebbles!! Part 2");
    let blinks = 75;

    let total_stones = simulate_stones_optimized(&initial_stones, blinks);
    println!("Total stones after {} blinks: {}", blinks, total_stones);
}

// Read input file and parse numbers as u64
fn read_input(file_path: &str) -> Vec<u64> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<u64>().expect("Invalid number in input file"))
        .collect()
}

fn simulate_stones(initial: &[u64], blinks: usize) -> usize {
    let mut queue: VecDeque<u64> = VecDeque::from(initial.to_vec());

    for _ in 0..blinks {
        let mut next_queue = VecDeque::new();
        
        while let Some(stone) = queue.pop_front() {
            if stone == 0 {
                next_queue.push_back(1);
            } else if has_even_digits(stone) {
                let (left, right) = split_number(stone);
                next_queue.push_back(left);
                next_queue.push_back(right);
            } else {
                // Safely multiply the number by 2024
                next_queue.push_back(stone * 2024);
            }
        }

        queue = next_queue;
    }

    queue.len()
}

fn has_even_digits(num: u64) -> bool {
    let digits = num.to_string().len();
    digits % 2 == 0
}

fn split_number(num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let mid = num_str.len() / 2;
    let left = num_str[..mid].parse::<u64>().unwrap();
    let right = num_str[mid..].parse::<u64>().unwrap();
    (left, right)
}


fn simulate_stones_optimized(initial: &[u64], blinks: usize) -> u64 {
    let mut stone_count: HashMap<u64, u64> = HashMap::new();

    // Initialize stone count: group stones by value
    for &stone in initial {
        *stone_count.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut next_stone_count: HashMap<u64, u64> = HashMap::new();

        for (&stone, &count) in stone_count.iter() {
            if stone == 0 {
                // Rule 1: 0 → 1
                *next_stone_count.entry(1).or_insert(0) += count;
            } else if has_even_digits(stone) {
                // Rule 2: Split into two stones
                let (left, right) = split_number(stone);
                *next_stone_count.entry(left).or_insert(0) += count;
                *next_stone_count.entry(right).or_insert(0) += count;
            } else {
                // Rule 3: Multiply by 2024
                let new_stone = stone * 2024;
                *next_stone_count.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_count = next_stone_count;
    }

    // Sum up total stones
    stone_count.values().sum()
}
