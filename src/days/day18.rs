use std::collections::{HashSet, VecDeque};

pub fn run() {
    println!("Day 18 - RAM Run!!");
    let file_path = "src/days/inputs/day18.txt";
    let input = read_input(file_path);
    const GRID_SIZE: usize = 71;

    // Simulate memory corruption
    let mut grid = vec![vec![true; GRID_SIZE]; GRID_SIZE]; // `true` = safe, `false` = corrupted
    for &(x, y) in input.iter().take(1024) {
        grid[y][x] = false;
    }

    // Find shortest path
    match bfs_shortest_path(&grid, (0, 0), (70, 70)) {
        Some(steps) => println!("Minimum steps to reach the exit: {}", steps),
        None => println!("No path to the exit."),
    }
}

fn read_input(file_path: &str) -> Vec<(usize, usize)> {
    let input = std::fs::read_to_string(file_path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

fn bfs_shortest_path(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Down, Right, Up, Left

    queue.push_back((start, 0)); // (position, steps)
    visited.insert(start);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps); // Reached the target
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < grid.len() as isize && ny < grid.len() as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[ny][nx] && visited.insert((nx, ny)) {
                    queue.push_back(((nx, ny), steps + 1));
                }
            }
        }
    }

    None // No path found
}
