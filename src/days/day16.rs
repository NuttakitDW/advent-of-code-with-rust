use std::fs;
use std::time::Instant;

const INF: usize = usize::MAX;

fn combine_routes(routes: &mut Vec<Vec<char>>, draw: &Vec<Vec<char>>) {
    for i in 0..draw.len() {
        for j in 0..draw[0].len() {
            if draw[i][j] == '+' && routes[i][j] != '+' {
                routes[i][j] = '+';
            }
        }
    }
}

fn is_safe(grid: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, y: usize, x: usize) -> bool {
    x < grid[0].len() && y < grid.len() && grid[y][x] != '#' && !visited[y][x]
}

fn find_shortest_path(
    grid: &Vec<Vec<char>>,
    dir: [bool; 4],
    visited: &mut Vec<Vec<bool>>,
    pos_y: usize,
    pos_x: usize,
    goal_y: usize,
    goal_x: usize,
    curr_dist: usize,
    weights: &mut Vec<Vec<usize>>,
    part: usize,
    draw: &mut Vec<Vec<char>>,
    routes: &mut Vec<Vec<char>>,
    min_dist: &mut usize,
) {
    if curr_dist > weights[pos_y][pos_x] + 1000 {
        return;
    }
    if *min_dist <= curr_dist && part == 1 {
        return;
    }
    if pos_y == goal_y && pos_x == goal_x {
        if curr_dist < *min_dist {
            *routes = grid.clone();
            *min_dist = curr_dist;
            combine_routes(routes, draw);
        }
        if curr_dist == *min_dist {
            combine_routes(routes, draw);
        }
        return;
    }

    visited[pos_y][pos_x] = true;
    weights[pos_y][pos_x] = curr_dist;
    draw[pos_y][pos_x] = '+';

    let directions = [
        (0, 1, [false, true, false, false]),  // right
        (1, 0, [true, false, false, false]), // down
        (!0, 0, [false, false, true, false]), // up (use wrapping_sub)
        (0, !0, [false, false, false, true]), // left (use wrapping_sub)
    ];

    for &(dy, dx, new_dir) in &directions {
        let next_y = pos_y.wrapping_add(dy); // Use wrapping_add for safe arithmetic
        let next_x = pos_x.wrapping_add(dx);

        if is_safe(grid, visited, next_y, next_x) {
            let dist_inc = if !dir[1] { 1001 } else { 1 };
            find_shortest_path(
                grid,
                new_dir,
                visited,
                next_y,
                next_x,
                goal_y,
                goal_x,
                curr_dist + dist_inc,
                weights,
                part,
                draw,
                routes,
                min_dist,
            );
        }
    }

    visited[pos_y][pos_x] = false;
    draw[pos_y][pos_x] = '.';
}

fn solve(part: usize) {
    let input = fs::read_to_string("src/days/inputs/day16.txt").expect("Failed to read input file");
    let mut grid = Vec::new();
    let mut visited = Vec::new();
    let mut weights = Vec::new();
    let mut routes = Vec::new();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut goal_x = 0;
    let mut goal_y = 0;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        let mut visit_row = Vec::new();
        let mut weight_row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            visit_row.push(false);
            weight_row.push(INF);
            if c == 'S' {
                start_x = x;
                start_y = y;
            } else if c == 'E' {
                goal_x = x;
                goal_y = y;
            }
        }
        grid.push(row);
        visited.push(visit_row);
        weights.push(weight_row);
        routes.push(vec!['.'; line.len()]);
    }

    let mut min_dist = INF;
    let mut draw = grid.clone();
    let start_time = Instant::now();

    find_shortest_path(
        &grid,
        [false, true, false, false],
        &mut visited,
        start_y,
        start_x,
        goal_y,
        goal_x,
        0,
        &mut weights,
        part,
        &mut draw,
        &mut routes,
        &mut min_dist,
    );

    if part == 1 {
        println!("Final: {}", min_dist);
        println!("--- {} seconds ---", start_time.elapsed().as_secs_f64());
    } else {
        let part_two_count = routes.iter().flatten().filter(|&&c| c == '+').count() + 1; // Include the goal cell
        println!("Final Part Two: {}", part_two_count);
        println!("--- {} seconds ---", start_time.elapsed().as_secs_f64());
    }
}

pub fn run() {
    println!("Part 1:");
    solve(1);
    println!("Part 2:");
    solve(2);
}
