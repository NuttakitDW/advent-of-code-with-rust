use std::fs;
use std::io::{self, BufRead};

pub fn run() {
    println!("Day 7 with concatenation!!");
    let file_path = "src/days/inputs/day7.txt";
    let (equations, test_values) = read_input(file_path).expect("Failed to read input");

    let ans1 = part1(&equations, &test_values); // Only + and *
    let ans2 = part2(&equations, &test_values); // +, *, and ||
    println!("part1: {}", ans1);
    println!("part2: {}", ans2);
}

fn read_input(file_path: &str) -> io::Result<(Vec<Vec<i64>>, Vec<i64>)> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut equations = Vec::new();
    let mut test_values = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        // format: "test_value: n1 n2 n3 ..."
        let parts: Vec<&str> = line.split(':').collect();
        let test_value = parts[0].trim().parse::<i64>().unwrap();
        let numbers_str = parts[1].trim().split_whitespace().collect::<Vec<_>>();
        let numbers = numbers_str
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        test_values.push(test_value);
        equations.push(numbers);
    }

    Ok((equations, test_values))
}

fn part1(equations: &Vec<Vec<i64>>, test_values: &Vec<i64>) -> i64 {
    // sum of test_values for equations that can be made true using only + and *
    let mut sum = 0;
    for (i, eq) in equations.iter().enumerate() {
        let tv = test_values[i];
        if can_make_true_basic(eq, tv) {
            sum += tv;
        }
    }
    sum
}

fn part2(equations: &Vec<Vec<i64>>, test_values: &Vec<i64>) -> i64 {
    // sum of test_values for equations that can be made true using +, *, and ||
    let mut sum = 0;
    for (i, eq) in equations.iter().enumerate() {
        let tv = test_values[i];
        if can_make_true_all_ops(eq, tv) {
            sum += tv;
        }
    }
    sum
}

fn can_make_true_basic(nums: &Vec<i64>, target: i64) -> bool {
    // Only + and * operators
    // If nums.len()==1 just check equality
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let n = nums.len();
    let total_combos = 1 << (n-1); // each gap is + or *

    for combo in 0..total_combos {
        let mut val = nums[0];
        for (idx, &num) in nums.iter().enumerate().skip(1) {
            let bit = (combo >> (idx-1)) & 1;
            if bit == 0 {
                // plus
                val += num;
            } else {
                // multiply
                val *= num;
            }
        }
        if val == target {
            return true;
        }
    }

    false
}

fn can_make_true_all_ops(nums: &Vec<i64>, target: i64) -> bool {
    // Now we have +, *, and |
    // We'll use backtracking as before, but now we have 3 choices per gap
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let ops_count = nums.len()-1;
    let mut ops = vec![' '; ops_count];

    fn backtrack(nums: &[i64], target: i64, idx: usize, ops: &mut [char]) -> bool {
        if idx == ops.len() {
            return evaluate(nums, ops) == target;
        }
        for &op in &['+', '*', '|'] {
            ops[idx] = op;
            if backtrack(nums, target, idx+1, ops) {
                return true;
            }
        }
        false
    }

    backtrack(nums, target, 0, &mut ops)
}

fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
    // Evaluate left-to-right with +, *, and '|'
    let mut val = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        let nxt = nums[i+1];
        match op {
            '+' => val = val + nxt,
            '*' => val = val * nxt,
            '|' => val = concat_numbers(val, nxt),
            _ => {}
        }
    }
    val
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    // Combine digits of b onto a
    let mut mag = 1;
    let mut tmp = b;
    while tmp >= 10 {
        tmp /= 10;
        mag *= 10;
    }
    a * (mag*10) + b
}
