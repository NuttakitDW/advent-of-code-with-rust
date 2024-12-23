use std::fs;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let grid_input = parts[0];
    let directions: Vec<char> = parts[1].chars().collect();

    let mut grid: Vec<Vec<char>> = grid_input.lines().map(|line| line.chars().collect()).collect();
    let mut robot = (0, 0);

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == '@' {
                robot = (i, j);
                *cell = '.'; // Replace robot's initial position with empty
            }
        }
    }

    (grid, directions, robot)
}

fn simulate(grid: &mut Vec<Vec<char>>, directions: &[char], mut robot: (usize, usize)) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());

    for &d in directions {
        let (i, j) = robot;
        match d {
            '<' => {
                let mut k = j as isize - 1;
                while k >= 0 && grid[i][k as usize] == 'O' {
                    k -= 1;
                }
                if k >= 0 && grid[i][k as usize] == '.' {
                    // Update positions
                    grid[i][k as usize] = 'O';
                    grid[i][j - 1] = '.';
                    robot = (i, j - 1);
                }
            }
            '>' => {
                let mut k = j + 1;
                while k < n && grid[i][k] == 'O' {
                    k += 1;
                }
                if k < n && grid[i][k] == '.' {
                    grid[i][k] = 'O';
                    grid[i][j + 1] = '.';
                    robot = (i, j + 1);
                }
            }
            '^' => {
                let mut k = i as isize - 1;
                while k >= 0 && grid[k as usize][j] == 'O' {
                    k -= 1;
                }
                if k >= 0 && grid[k as usize][j] == '.' {
                    grid[k as usize][j] = 'O';
                    grid[i - 1][j] = '.';
                    robot = (i - 1, j);
                }
            }
            'v' => {
                let mut k = i + 1;
                while k < m && grid[k][j] == 'O' {
                    k += 1;
                }
                if k < m && grid[k][j] == '.' {
                    grid[k][j] = 'O';
                    grid[i + 1][j] = '.';
                    robot = (i + 1, j);
                }
            }
            _ => {}
        }
    }

    // Calculate GPS sum
    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                total += 100 * i as i32 + j as i32;
            }
        }
    }

    total
}

pub fn run() {
    let input = fs::read_to_string("src/days/inputs/day15.txt").expect("Failed to read input file");
    let (mut grid, directions, robot) = parse_input(&input);

    let result = simulate(&mut grid, &directions, robot);
    println!("Sum of GPS coordinates: {}", result);
}
