use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("src/days/inputs/day03.txt").expect("Failed to read input");

    // Part 1
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    // Part 2
    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> i64 {
    // Regex to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    // Regex to match valid instructions
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true; // Start with mul instructions enabled
    let mut sum = 0;

    // Process each character or section of input
    for line in input.split_inclusive([';', '\n']) {
        if dont_re.is_match(line) {
            // Disable future mul instructions
            enabled = false;
        } else if do_re.is_match(line) {
            // Enable future mul instructions
            enabled = true;
        }

        // Process mul only if enabled
        if enabled {
            for cap in mul_re.captures_iter(line) {
                let x: i64 = cap[1].parse().unwrap();
                let y: i64 = cap[2].parse().unwrap();
                sum += x * y;
            }
        }
    }

    sum
}
