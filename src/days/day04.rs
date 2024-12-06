use std::fs;
use std::io::{self, BufRead};

pub fn run() {
    let file_path = "src/days/inputs/day4.txt";
    let grid = read_input(file_path).expect("Failed to read input");
    let ans1 = part1(&grid);
    let ans2 =part2(&grid);
    print!("Day 4!!\n");
    println!("part1: {}", ans1);
    println!("part2: {}", ans2);
}

fn read_input(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
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
    let mut count = 0;
    for i in 0..rows {
        let cols = grid[i].len();
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(grid, word, i as isize, j as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let count = count_x_mas(&grid);
    count
}

fn count_x_mas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len() as isize;
    let mut count = 0;
    let sequences = vec![
        // diag can be MAS or SAM
        (['M','A','S'], ['M','A','S']),
        (['M','A','S'], ['S','A','M']),
        (['S','A','M'], ['M','A','S']),
        (['S','A','M'], ['S','A','M']),
    ];

    for i in 0..rows {
        let cols = grid[i as usize].len() as isize;
        for j in 0..cols {
            if grid[i as usize][j as usize] != 'A' {
                continue;
            }
            let top_left = (i-1, j-1);
            let bottom_right = (i+1, j+1);
            let top_right = (i-1, j+1);
            let bottom_left = (i+1, j-1);

            if !in_bounds(grid, top_left) || !in_bounds(grid, bottom_right) ||
               !in_bounds(grid, top_right) || !in_bounds(grid, bottom_left) {
                continue;
            }

            let c_tl = grid[top_left.0 as usize][top_left.1 as usize];
            let c_br = grid[bottom_right.0 as usize][bottom_right.1 as usize];
            let c_tr = grid[top_right.0 as usize][top_right.1 as usize];
            let c_bl = grid[bottom_left.0 as usize][bottom_left.1 as usize];

            for (diag1, diag2) in &sequences {
                // diag1 corresponds to top-left -> A -> bottom-right
                // diag2 corresponds to top-right -> A -> bottom-left

                // ignore 'X', must match exact chars M, A, S
                if diag1[0] == c_tl && diag1[1] == 'A' && diag1[2] == c_br &&
                   diag2[0] == c_tr && diag2[1] == 'A' && diag2[2] == c_bl &&
                   c_tl != 'X' && c_br != 'X' && c_tr != 'X' && c_bl != 'X' {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn in_bounds(grid: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    let (x, y) = pos;
    if x < 0 || x >= grid.len() as isize {
        return false;
    }
    if y < 0 || y >= grid[x as usize].len() as isize {
        return false;
    }
    true
}

fn check_word(
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
        if x < 0 || x >= grid.len() as isize {
            return false;
        }
        if y < 0 || y as usize >= grid[x as usize].len() {
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
