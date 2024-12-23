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


fn count_sides(
    region_cells: &HashSet<(usize, usize)>,
    _grid: &[Vec<char>],
    plant_type: char,
) -> u32 {

    let mut sides = 0;

    // Iterate through each cell in the region
    for &(r, c) in region_cells {
        // Define neighbors
        let north = (r as isize - 1, c as isize);
        let south = (r as isize + 1, c as isize);
        let west = (r as isize, c as isize - 1);
        let east = (r as isize, c as isize + 1);
        let nw = (r as isize - 1, c as isize - 1);
        let sw = (r as isize + 1, c as isize - 1);
        let ne = (r as isize - 1, c as isize + 1);

        // Check each edge
        if !region_cells.contains(&(north.0 as usize, north.1 as usize)) {
            // North edge
            let same_edge = region_cells.contains(&(west.0 as usize, west.1 as usize))
                && !region_cells.contains(&(nw.0 as usize, nw.1 as usize));
            if !same_edge {
                sides += 1;
                println!(
                    "North edge counted for cell ({}, {}) | Plant type '{}'",
                    r, c, plant_type
                );
            }
        }

        if !region_cells.contains(&(south.0 as usize, south.1 as usize)) {
            // South edge
            let same_edge = region_cells.contains(&(west.0 as usize, west.1 as usize))
                && !region_cells.contains(&(sw.0 as usize, sw.1 as usize));
            if !same_edge {
                sides += 1;
                println!(
                    "South edge counted for cell ({}, {}) | Plant type '{}'",
                    r, c, plant_type
                );
            }
        }

        if !region_cells.contains(&(west.0 as usize, west.1 as usize)) {
            // West edge
            let same_edge = region_cells.contains(&(north.0 as usize, north.1 as usize))
                && !region_cells.contains(&(nw.0 as usize, nw.1 as usize));
            if !same_edge {
                sides += 1;
                println!(
                    "West edge counted for cell ({}, {}) | Plant type '{}'",
                    r, c, plant_type
                );
            }
        }

        if !region_cells.contains(&(east.0 as usize, east.1 as usize)) {
            // East edge
            let same_edge = region_cells.contains(&(north.0 as usize, north.1 as usize))
                && !region_cells.contains(&(ne.0 as usize, ne.1 as usize));
            if !same_edge {
                sides += 1;
                println!(
                    "East edge counted for cell ({}, {}) | Plant type '{}'",
                    r, c, plant_type
                );
            }
        }
    }

    println!("Total sides for plant type '{}': {}", plant_type, sides);
    sides
}