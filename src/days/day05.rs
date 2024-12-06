use std::fs;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    print!("Day 5!!\n");
    let file_path = "src/days/inputs/day5.txt";
    let (rules, updates) = read_input(file_path).expect("Failed to read input");
    let ans1 = part1(&rules, &updates);
    let ans2 = part2(&rules, &updates);
    println!("part1: {}", ans1);
    println!("part2: {}", ans2);
}

fn read_input(file_path: &str) -> io::Result<(Vec<(i32,i32)>, Vec<Vec<i32>>)> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut split_idx = 0;
    for (idx, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            split_idx = idx;
            break;
        }
    }

    let rule_lines = &lines[..split_idx];
    let update_lines = &lines[(split_idx+1)..];

    let mut rules = Vec::new();
    for line in rule_lines {
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split('|').collect();
        let x = parts[0].trim().parse::<i32>().unwrap();
        let y = parts[1].trim().parse::<i32>().unwrap();
        rules.push((x,y));
    }

    let mut updates = Vec::new();
    for line in update_lines {
        if line.trim().is_empty() { continue; }
        let nums = line.split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        updates.push(nums);
    }

    Ok((rules, updates))
}

fn part1(rules: &Vec<(i32,i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for update in updates {
        if update_correct(update, rules) {
            let mid = update[update.len()/2];
            sum += mid;
        }
    }
    sum
}

fn part2(rules: &Vec<(i32,i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    // We only sum the middle page numbers of the incorrectly-ordered updates AFTER sorting them into the correct order.
    let mut sum = 0;
    for update in updates {
        if update_correct(update, rules) {
            // correct order -> ignore for this part
        } else {
            // incorrect order -> sort it topologically according to rules
            let sorted = correct_order(update, rules);
            let mid = sorted[sorted.len()/2];
            sum += mid;
        }
    }
    sum
}

fn correct_order(update: &Vec<i32>, rules: &Vec<(i32,i32)>) -> Vec<i32> {
    // Build a directed graph from the rules that apply to this update
    let pages: HashSet<i32> = update.iter().cloned().collect();

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indeg: HashMap<i32, i32> = HashMap::new();

    for &p in &pages {
        indeg.insert(p, 0);
        graph.insert(p, Vec::new());
    }

    for &(x,y) in rules {
        if pages.contains(&x) && pages.contains(&y) {
            // x must come before y
            graph.get_mut(&x).unwrap().push(y);
            *indeg.get_mut(&y).unwrap() += 1;
        }
    }

    // Topological sort (Kahn's algorithm)
    let mut q = VecDeque::new();
    for &p in &pages {
        if *indeg.get(&p).unwrap() == 0 {
            q.push_back(p);
        }
    }

    let mut result = Vec::new();
    while let Some(u) = q.pop_front() {
        result.push(u);
        for &nxt in &graph[&u] {
            let entry = indeg.get_mut(&nxt).unwrap();
            *entry -= 1;
            if *entry == 0 {
                q.push_back(nxt);
            }
        }
    }

    // result should now be a topological ordering of the pages in update
    // According to the puzzle, it should always yield a valid ordering
    result
}

fn update_correct(update: &Vec<i32>, rules: &Vec<(i32,i32)>) -> bool {
    for &(x,y) in rules {
        if let (Some(ix), Some(iy)) = (update.iter().position(|&v| v==x), update.iter().position(|&v| v==y)) {
            if ix > iy {
                return false;
            }
        }
    }
    true
}
