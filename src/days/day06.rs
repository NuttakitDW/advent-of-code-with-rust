use std::fs;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};

pub fn run() {
    println!("Day 6!!");
    let file_path = "src/days/inputs/day6.txt";
    let mut grid = read_input(file_path).expect("Failed to read input");
    // let ans1 = part1(&mut grid);
    let ans2: usize = part2(&mut grid);
    // println!("part1: {}", ans1);
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

fn part1(grid: &mut [Vec<char>]) -> usize {
    // Count distinct visited positions from initial scenario (no added obstruction)
    let (sx, sy, dir) = find_guard(grid);
    let visited_positions = simulate(grid, sx, sy, dir, None);
    visited_positions.len()
}

fn part2(grid: &mut [Vec<char>]) -> usize {
    // We want to find how many positions we can place a new '#' to cause a loop.
    // The new obstruction:
    // 1) Can't be at guard's starting position
    // 2) Must change a '.' cell to '#'
    // If placing this obstruction causes the guard to get stuck in a loop, count it.

    let (sx, sy, sdir) = find_guard(grid);

    let rows = grid.len();
    let mut count = 0;

    for i in 0..rows {
        let cols = grid[i].len();
        for j in 0..cols {
            if i == sx && j == sy {
                continue; // can't place obstruction at starting position
            }
            if grid[i][j] == '.' {
                // Temporarily place obstruction
                grid[i][j] = '#';

                if causes_loop(grid, sx, sy, sdir) {
                    count += 1;
                }

                // revert
                grid[i][j] = '.';
            }
        }
    }

    count
}

fn causes_loop(grid: &[Vec<char>], sx: usize, sy: usize, sdir: usize) -> bool {
    // If guard falls into a loop after adding the obstruction at a certain cell,
    // it means we revisit a (x,y,dir) state.
    // We'll simulate similarly, but detect loops using a state set.

    let mut visited_states = HashSet::new();
    let mut x = sx;
    let mut y = sy;
    let mut dir = sdir;

    // Replace guard symbol with '.'
    // just for simulation (the guard symbol should only appear once)
    // we do this by ignoring guard symbol as direction source only once at start

    let directions = [(-1,0),(0,1),(1,0),(0,-1)];

    visited_states.insert((x,y,dir));

    loop {
        let (dx,dy) = directions[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // If out of bounds forward => guard leaves area, no loop
        if out_of_bounds(grid,nx,ny) {
            return false;
        }

        // If obstacle forward => turn right
        if grid[nx as usize][ny as usize] == '#' {
            dir = (dir+1)%4;
            // Check if this new state also repeated?
            // Actually we only add state when move forward
            // no forward step means no new position
        } else {
            // Move forward
            x = nx as usize;
            y = ny as usize;
            // If out of bounds after move => no loop
            if out_of_bounds(grid,x as isize,y as isize) {
                return false;
            }

            let state = (x,y,dir);
            if visited_states.contains(&state) {
                // Found loop!
                return true;
            }
            visited_states.insert(state);
        }

        // Safety break if something goes too large
        if visited_states.len()>1_000_000 { 
            // just a safety cutoff
            return false;
        }
    }
}

fn simulate(grid: &[Vec<char>], sx: usize, sy: usize, sdir: usize, obstruction: Option<(usize,usize)>) -> HashSet<(usize,usize)> {
    // Basic simulation to get visited path from starting conditions
    let mut visited = HashSet::new();
    let mut x = sx;
    let mut y = sy;
    let mut dir = sdir;

    visited.insert((x,y));

    let directions = [(-1,0),(0,1),(1,0),(0,-1)];

    loop {
        let (dx, dy) = directions[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if out_of_bounds(grid, nx, ny) {
            break; // guard leaves area
        }

        let cell = grid[nx as usize][ny as usize];

        // Check forward
        if cell == '#' {
            // turn right
            dir = (dir+1)%4;
        } else {
            // move forward
            x = nx as usize;
            y = ny as usize;
            visited.insert((x,y));

            if out_of_bounds(grid,x as isize,y as isize) {
                break; // out of map
            }
        }

        // Safety break
        if visited.len()>1_000_000 {
            break;
        }
    }

    visited
}

fn find_guard(grid: &[Vec<char>]) -> (usize, usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            match c {
                '^'=>return (i,j,0),
                '>'=>return (i,j,1),
                'v'=>return (i,j,2),
                '<'=>return (i,j,3),
                _=>{}
            }
        }
    }
    panic!("No guard found");
}

fn out_of_bounds(grid:&[Vec<char>], x:isize,y:isize)->bool {
    if x<0||x>=grid.len() as isize {return true}
    if y<0||y>=grid[x as usize].len() as isize {return true}
    false
}
