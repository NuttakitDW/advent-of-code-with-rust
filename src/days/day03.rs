use std::fs;
use std::io::{self, BufRead};

pub fn count_xmas_occurrences(grid: Vec<Vec<char>>) -> usize {
    let word = "XMAS";
    let directions = vec![
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(&grid, word, i as isize, j as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn check_word(
    grid: &Vec<Vec<char>>,
    word: &str,
    start_x: isize,
    start_y: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let mut x = start_x;
    let mut y = start_y;

    for &ch in &chars {
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return false;
        }
        if grid[x as usize][y as usize] != ch {
            return false;
        }
        x += dx;
        y += dy;
    }

    true
}

pub fn run() {
    let file_path = "src/days/inputs/day3.txt";
    let grid = read_input(file_path);

    match grid {
        Ok(grid) => {
            let result = count_xmas_occurrences(grid);
            println!("Total occurrences of XMAS: {}", result);
        }
        Err(e) => {
            println!("Failed to read input: {}", e);
        }
    }
}

pub fn read_input(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}
