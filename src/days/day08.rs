use std::collections::HashSet;
use std::fs;

pub fn run() {
    println!("Day 8!!");
    let file_path = "src/days/inputs/day8.txt";
    let grid = read_input(file_path);

    let ans1 = part1(&grid);
    println!("part1: {}", ans1);

    let ans2 = part2(&grid);
    println!("part2: {}", ans2);
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antinodes = HashSet::new();

    for i1 in 0..rows {
        for j1 in 0..cols {
            if grid[i1][j1] == '.' {
                continue;
            }

            for i2 in 0..rows {
                for j2 in 0..cols {
                    if (i1, j1) == (i2, j2) || grid[i1][j1] != grid[i2][j2] {
                        continue;
                    }

                    let di = i2 as isize - i1 as isize;
                    let dj = j2 as isize - j1 as isize;

                    let antinode1 = (i1 as isize - di, j1 as isize - dj);
                    let antinode2 = (i2 as isize + di, j2 as isize + dj);

                    if in_bounds(antinode1, rows, cols) {
                        antinodes.insert(antinode1);
                    }
                    if in_bounds(antinode2, rows, cols) {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part2(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antinodes = HashSet::new();

    for i1 in 0..rows {
        for j1 in 0..cols {
            if grid[i1][j1] == '.' {
                continue;
            }

            for i2 in 0..rows {
                for j2 in 0..cols {
                    if (i1, j1) == (i2, j2) || grid[i1][j1] != grid[i2][j2] {
                        continue;
                    }

                    let di = i2 as isize - i1 as isize;
                    let dj = j2 as isize - j1 as isize;

                    let mut positions = vec![(i1 as isize, j1 as isize), (i2 as isize, j2 as isize)];

                    for k in 1.. {
                        let next_pos = (i2 as isize + k * di, j2 as isize + k * dj);
                        if !in_bounds(next_pos, rows, cols) {
                            break;
                        }
                        positions.push(next_pos);
                    }

                    for k in 1.. {
                        let prev_pos = (i1 as isize - k * di, j1 as isize - k * dj);
                        if !in_bounds(prev_pos, rows, cols) {
                            break;
                        }
                        positions.push(prev_pos);
                    }

                    for pos in positions {
                        antinodes.insert(pos);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn in_bounds(pos: (isize, isize), rows: usize, cols: usize) -> bool {
    let (i, j) = pos;
    i >= 0 && i < rows as isize && j >= 0 && j < cols as isize
}
