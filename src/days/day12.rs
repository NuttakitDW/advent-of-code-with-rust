use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn run() {
    println!("Day 12 - Garden Groups!!");
    let file_path = "src/days/inputs/day12.txt";
    let grid = read_input(file_path);

    let total_price = calculate_total_price(&grid);
    println!("Total price of fencing: {}", total_price);

    println!("Day 12 - Garden Groups!! (Part 2)");

    let total_price = calculate_total_price2(&grid);
    println!("New total price of fencing: {}", total_price);
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_total_price(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut total_price = 0;

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let plant_type = grid[i][j];
                let (area, perimeter) = flood_fill_and_calculate(&grid, &mut visited, i, j, plant_type, &directions);
                let price = area * perimeter;
                total_price += price;
            }
        }
    }

    total_price
}

fn flood_fill_and_calculate(
    grid: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
    plant_type: char,
    directions: &[(isize, isize)],
) -> (u32, u32) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeque::new();
    let mut region_cells = HashSet::new();
    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        region_cells.insert((x, y));

        for &(dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == plant_type {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    let area = region_cells.len() as u32;
    let perimeter = calculate_perimeter(&region_cells, grid, plant_type, directions);

    (area, perimeter)
}

fn calculate_perimeter(
    region_cells: &HashSet<(usize, usize)>,
    grid: &[Vec<char>],
    plant_type: char,
    directions: &[(isize, isize)],
) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut perimeter = 0;

    for &(x, y) in region_cells {
        for &(dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                // Out of bounds - add to perimeter
                perimeter += 1;
            } else {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] != plant_type {
                    // Adjacent cell is not part of the region
                    perimeter += 1;
                }
            }
        }
    }

    perimeter
}

fn calculate_total_price2(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up

    let mut total_price = 0;

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let plant_type = grid[i][j];
                let (area, sides) = flood_fill_and_count_sides(&grid, &mut visited, i, j, plant_type, &directions);
                let price = area * sides;
                total_price += price;
            }
        }
    }

    total_price
}

fn flood_fill_and_count_sides(
    grid: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
    plant_type: char,
    directions: &[(isize, isize)],
) -> (u32, u32) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeque::new();
    let mut region_cells = HashSet::new();
    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        region_cells.insert((x, y));

        for &(dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == plant_type {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    let area = region_cells.len() as u32;
    let sides = count_sides(&region_cells, grid, plant_type);

    (area, sides)
}


pub fn count_sides(
    region_cells: &HashSet<(usize, usize)>, 
    grid: &[Vec<char>], 
    plant_type: char
) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions for north, south, east, west
    let directions = [
        (0, -1),  // North
        (0, 1),   // South
        (-1, 0),  // West
        (1, 0),   // East
    ];

    let mut sides = 0;
    let mut visited_edges = HashSet::new(); // Track counted edges

    for &(x, y) in region_cells {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            let current_cell = (x, y);

            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                // Out of bounds - external edge
                let normalized_edge = normalize_edge(current_cell, (nx, ny));

                if !visited_edges.contains(&normalized_edge) {
                    visited_edges.insert(normalized_edge);
                    sides += 1;

                    println!(
                        "Edge counted: {:?} (out of bounds) | Plant type at {:?}: '{}'",
                        normalized_edge, (x, y), plant_type
                    );
                }
            } else {
                let neighbor_cell = (nx as usize, ny as usize);
                let neighbor_type = grid[nx as usize][ny as usize];

                if neighbor_type != plant_type {
                    // Neighbor is not the same plant type - external edge
                    let normalized_edge = normalize_edge(current_cell, (nx, ny));

                    if !visited_edges.contains(&normalized_edge) {
                        visited_edges.insert(normalized_edge);
                        sides += 1;

                        println!(
                            "Edge counted: {:?} (neighbor: {:?} is '{}') | Plant type at {:?}: '{}'",
                            normalized_edge, neighbor_cell, neighbor_type, (x, y), plant_type
                        );
                    }
                }
            }
        }
    }

    println!("Total sides for plant type '{}': {}", plant_type, sides);
    sides
}

// Normalize an edge so that (a, b) == (b, a)
fn normalize_edge(a: (usize, usize), b: (isize, isize)) -> ((usize, usize), (usize, usize)) {
    let b = (b.0.max(0) as usize, b.1.max(0) as usize);
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}
