use std::collections::HashMap;
use std::fs;

pub fn run() {
    let (left_list, right_list) = read_numbers("src/days/inputs/day1.txt");
    part1(&left_list, &right_list);
    part2(&left_list, &right_list);

}

fn part1(left_list: &[i32], right_list: &[i32]) {
    let mut left_sorted = left_list.to_vec(); // TODO: what is to_vec
    let mut right_sorted = right_list.to_vec();

    left_sorted.sort();
    right_sorted.sort();

    let total_distance: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter()) // TODO: what is zip
        .map(|(l,r)|(l-r).abs())
        .sum();  
    println!("Part1: The total distance between the lists is: {}", total_distance);
}

fn part2(left_list: &[i32], right_list: &[i32]) {
    let mut right_counts = HashMap::new();
    for &num in right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum();
    println!("Part2: The simirality score is: {}", similarity_score)
}


fn read_numbers(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_list.push(parts[0].parse::<i32>().expect("Failed to parse number"));
            right_list.push(parts[1].parse::<i32>().expect("Failed to parse number"));
        }
    }

    (left_list, right_list)
}