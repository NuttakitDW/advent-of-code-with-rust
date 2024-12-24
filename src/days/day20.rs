use std::collections::HashSet;
use std::fs;

pub fn run() {
    let file_path = "src/days/inputs/day20.txt";
    let grid = fs::read_to_string(file_path).expect("Failed to read file");
    let track = parse_track(&grid);

    let result_cheats_2 = count_saved_steps(&track, 2, 100);
    let result_cheats_20 = count_saved_steps(&track, 20, 100);

    println!("Cheats saving at least 100 steps (max distance 2): {}", result_cheats_2);
    println!("Cheats saving at least 100 steps (max distance 20): {}", result_cheats_20);
}

fn parse_track(grid: &str) -> Vec<(usize, usize)> {
    let grid: Vec<&str> = grid.lines().collect();
    let (mut x, mut y) = grid.iter()
        .enumerate()
        .find_map(|(y, row)| row.chars().enumerate().find(|(_, c)| *c == 'S').map(|(x, _)| (x, y)))
        .expect("Start position 'S' not found in the grid");
    
    let mut track = vec![(x, y)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((x, y));

    while grid[y].chars().nth(x) != Some('E') {
        if let Some((nx, ny)) = neighbors(x, y)
            .into_iter()
            .filter(|&(nx, ny)| {
                grid.get(ny).and_then(|row| row.chars().nth(nx)) != Some('#')
                    && !visited.contains(&(nx, ny))
            })
            .next()
        {
            track.push((nx, ny));
            visited.insert((nx, ny));
            x = nx;
            y = ny;
        } else {
            panic!("No valid path to 'E' found.");
        }
    }

    track
}

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut nbs = vec![];
    if x > 0 { nbs.push((x - 1, y)); }
    nbs.push((x + 1, y));
    if y > 0 { nbs.push((x, y - 1)); }
    nbs.push((x, y + 1));
    nbs
}

fn count_saved_steps(track: &[(usize, usize)], max_dist: usize, threshold: usize) -> usize {
    let mut count = 0;

    for (t1, &(x1, y1)) in track.iter().enumerate() {
        for t2 in (t1 + 3)..track.len() {
            let (x2, y2) = track[t2];
            let dist = (x2 as isize - x1 as isize).abs() + (y2 as isize - y1 as isize).abs();
            let path_len = t2 - t1;

            if dist as usize <= max_dist && path_len > dist as usize {
                let saved = path_len - dist as usize;
                if saved >= threshold {
                    count += 1;
                }
            }
        }
    }

    count
}
