use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn run() {
    println!("Day 10!!");
    let file_path = "src/days/inputs/day10.txt";
    let map = read_input(file_path);

    let total_score = calculate_total_trailhead_score(&map);
    println!("Total trailhead score: {}", total_score);

    println!("Day 10 - Part 2!!");

    let total_rating = calculate_total_trailhead_rating(&map);
    println!("Total trailhead rating: {}", total_rating);
}

fn read_input(file_path: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn calculate_total_trailhead_score(map: &[Vec<u32>]) -> usize {
    let mut total_score = 0;

    let rows = map.len();
    let cols = map[0].len();

    // Iterate over all positions to find trailheads (height 0)
    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == 0 {
                let score = find_reachable_nines(map, i, j);
                total_score += score;
            }
        }
    }

    total_score
}

fn find_reachable_nines(map: &[Vec<u32>], start_x: usize, start_y: usize) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up
    let mut reachable_nines = HashSet::new();

    queue.push_back((start_x, start_y, 0)); // (x, y, current height)
    visited.insert((start_x, start_y));

    while let Some((x, y, current_height)) = queue.pop_front() {
        // If we find a 9, add it to the reachable set
        if map[x][y] == 9 {
            reachable_nines.insert((x, y));
            continue; // Stop exploring beyond height 9
        }

        // Explore neighboring positions
        for &(dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0
                && new_x < rows as isize
                && new_y >= 0
                && new_y < cols as isize
            {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Ensure we only move to positions with height = current_height + 1
                if map[new_x][new_y] == current_height + 1 && !visited.contains(&(new_x, new_y)) {
                    visited.insert((new_x, new_y));
                    queue.push_back((new_x, new_y, current_height + 1));
                }
            }
        }
    }

    reachable_nines.len()
}

fn calculate_total_trailhead_rating(map: &[Vec<u32>]) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut total_rating = 0;

    // Iterate over all positions to find trailheads (height 0)
    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == 0 {
                let mut visited = vec![vec![false; cols]; rows];
                let rating = count_paths_to_nines(map, i, j, 0, &mut visited);
                total_rating += rating;
            }
        }
    }

    total_rating
}

fn count_paths_to_nines(
    map: &[Vec<u32>],
    x: usize,
    y: usize,
    current_height: u32,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    // Base case: If we are at height 9, we found a valid path
    if map[x][y] == 9 {
        return 1;
    }

    // Mark this position as visited
    visited[x][y] = true;

    let mut path_count = 0;

    // Define movement directions: up, down, left, right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0
            && new_x < rows as isize
            && new_y >= 0
            && new_y < cols as isize
        {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Move only to positions with height = current height + 1
            if map[new_x][new_y] == current_height + 1 && !visited[new_x][new_y] {
                path_count += count_paths_to_nines(map, new_x, new_y, current_height + 1, visited);
            }
        }
    }

    // Backtrack: Unmark this position for other paths
    visited[x][y] = false;

    path_count
}