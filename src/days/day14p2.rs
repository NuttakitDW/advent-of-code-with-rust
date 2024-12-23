use std::collections::{HashMap, HashSet};
use std::fs;
use regex::Regex;
use image::{RgbImage, Rgb};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn move_n(&self, n: i32) -> Robot {
        Robot {
            x: (self.x + self.vx * n).rem_euclid(WIDTH),
            y: (self.y + self.vy * n).rem_euclid(HEIGHT),
            vx: self.vx,
            vy: self.vy,
        }
    }
}

fn read_input(file_path: &str) -> Vec<Robot> {
    let robo_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let input = fs::read_to_string(file_path).expect("Failed to read input file");

    input
        .lines()
        .filter_map(|line| {
            robo_re.captures(line).map(|cap| Robot {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                vx: cap[3].parse().unwrap(),
                vy: cap[4].parse().unwrap(),
            })
        })
        .collect()
}

fn find_min_unique_positions(robots: &[Robot]) -> i32 {
    for t in 0.. {
        let mut positions = HashSet::new();
        let mut duplicate_found = false;

        for robot in robots {
            let moved = robot.move_n(t);
            if !positions.insert((moved.x, moved.y)) {
                duplicate_found = true;
                break;
            }
        }

        if !duplicate_found {
            return t; // All positions are unique at this time
        }
    }
    unreachable!()
}

fn save_image(robots: &[Robot], second: i32, file_name: &str) {
    let mut img = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    let white = Rgb([255, 255, 255]);
    let black = Rgb([0, 0, 0]);

    // Fill the image with black pixels
    for pixel in img.pixels_mut() {
        *pixel = black;
    }

    // Set robot positions to white
    for robot in robots {
        let moved = robot.move_n(second);
        img.put_pixel(moved.x as u32, moved.y as u32, white);
    }

    // Save the image
    img.save(file_name).expect("Failed to save image");
}

pub fn run() {
    println!("Day 14 - Restroom Redoubt!");

    let file_path = "src/days/inputs/day14.txt";
    let output_image = "robots_image.png";

    let robots = read_input(file_path);
    let time = find_min_unique_positions(&robots);

    println!("Fewest seconds to unique positions: {}", time);
    save_image(&robots, time, output_image);
    println!("Saved the robot arrangement at time {} to '{}'", time, output_image);
}
