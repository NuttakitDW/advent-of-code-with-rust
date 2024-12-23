use std::fs;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}

fn parse_input(file_path: &str) -> (Vec<Vec<Tile>>, Vec<char>, Position) {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut grid = Vec::new();
    let mut movements = Vec::new();
    let mut robot_position = (0, 0);

    for (row_idx, line) in input.lines().enumerate() {
        if line.starts_with('#') {
            let mut row = Vec::new();
            for (col_idx, ch) in line.chars().enumerate() {
                match ch {
                    '#' => row.push(Tile::Wall),
                    '.' => row.push(Tile::Empty),
                    'O' => row.push(Tile::Box),
                    '@' => {
                        robot_position = (row_idx, col_idx);
                        row.push(Tile::Robot);
                    }
                    _ => {}
                }
            }
            grid.push(row);
        } else {
            movements.extend(line.chars());
        }
    }

    (grid, movements, robot_position)
}

fn simulate_movements(
    mut grid: Vec<Vec<Tile>>,
    movements: Vec<char>,
    mut robot_position: Position,
) -> Vec<Vec<Tile>> {
    let directions = [
        ('^', (-1, 0)), // Up
        ('v', (1, 0)),  // Down
        ('<', (0, -1)), // Left
        ('>', (0, 1)),  // Right
    ];

    for movement in movements {
        let (dx, dy) = directions
            .iter()
            .find(|&&(dir, _)| dir == movement)
            .map(|&(_, d)| d)
            .unwrap_or((0, 0));

        let new_robot_pos = (
            (robot_position.0 as isize + dx) as usize,
            (robot_position.1 as isize + dy) as usize,
        );

        match grid.get(new_robot_pos.0).and_then(|row| row.get(new_robot_pos.1)) {
            Some(Tile::Empty) => {
                grid[robot_position.0][robot_position.1] = Tile::Empty;
                grid[new_robot_pos.0][new_robot_pos.1] = Tile::Robot;
                robot_position = new_robot_pos;
            }
            Some(Tile::Box) => {
                let new_box_pos = (
                    (new_robot_pos.0 as isize + dx) as usize,
                    (new_robot_pos.1 as isize + dy) as usize,
                );

                if matches!(grid.get(new_box_pos.0).and_then(|row| row.get(new_box_pos.1)), Some(Tile::Empty)) {
                    grid[new_box_pos.0][new_box_pos.1] = Tile::Box;
                    grid[new_robot_pos.0][new_robot_pos.1] = Tile::Robot;
                    grid[robot_position.0][robot_position.1] = Tile::Empty;
                    robot_position = new_robot_pos;
                }
            }
            _ => {}
        }
    }

    grid
}

fn calculate_gps(grid: &[Vec<Tile>]) -> i32 {
    let mut gps_sum = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if let Tile::Box = tile {
                gps_sum += 100 * row_idx as i32 + col_idx as i32;
            }
        }
    }

    gps_sum
}

fn print_grid(grid: &[Vec<Tile>]) {
    for row in grid {
        for tile in row {
            let symbol = match tile {
                Tile::Wall => '#',
                Tile::Empty => '.',
                Tile::Box => 'O',
                Tile::Robot => '@',
            };
            print!("{}", symbol);
        }
        println!();
    }
    println!();
}

pub fn run() {
    let file_path = "src/days/inputs/day15.txt"; // Update with your actual input file path
    let (grid, movements, robot_position) = parse_input(file_path);

    println!("Initial State:");
    print_grid(&grid);

    let final_grid = simulate_movements(grid, movements, robot_position);

    println!("Final State:");
    print_grid(&final_grid);

    let gps_sum = calculate_gps(&final_grid);
    println!("Sum of GPS coordinates: {}", gps_sum);
}
