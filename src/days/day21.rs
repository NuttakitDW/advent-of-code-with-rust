use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Keys {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyLeft,
    Empty,
}

use Keys::*;

fn get_input(file_path: &str) -> Result<Vec<(Vec<Keys>, usize)>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;
    let result = input
        .trim()
        .lines()
        .map(|line| {
            let keys = line
                .chars()
                .map(|c| match c {
                    '0' => Key0,
                    '1' => Key1,
                    '2' => Key2,
                    '3' => Key3,
                    '4' => Key4,
                    '5' => Key5,
                    '6' => Key6,
                    '7' => Key7,
                    '8' => Key8,
                    '9' => Key9,
                    'A' => KeyA,
                    _ => unreachable!(),
                })
                .collect();

            let value = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();

            Ok((keys, value))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(result)
}

fn find_shortests(
    start: (usize, usize),
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, Vec<Vec<Keys>>> {
    let mut paths: HashMap<Keys, Vec<Vec<Keys>>> = HashMap::new();
    let mut to_do: VecDeque<((usize, usize), Vec<Keys>)> = vec![(start, vec![])].into();

    let start_key = keypad.get(&start).unwrap();
    paths.insert(*start_key, vec![]);

    while let Some((coordinate, path)) = to_do.pop_front() {
        let current_key = keypad.get(&coordinate).unwrap();
        let shortest = paths.entry(*current_key).or_default();

        if shortest.is_empty() || shortest.last().unwrap().len() > path.len() {
            *shortest = vec![path.clone()];
        } else if shortest.last().unwrap().len() == path.len() {
            (*shortest).push(path.clone());
        } else {
            continue;
        }

        let neighbors = [
            ((coordinate.0.wrapping_sub(1), coordinate.1), KeyUp),
            ((coordinate.0, coordinate.1 + 1), KeyRight),
            ((coordinate.0 + 1, coordinate.1), KeyDown),
            ((coordinate.0, coordinate.1.wrapping_sub(1)), KeyLeft),
        ];

        for (neighbor, direction) in neighbors {
            if let Some(key) = keypad.get(&neighbor) {
                if *key != Empty && key != start_key {
                    let mut next_path = path.clone();
                    next_path.push(direction);
                    to_do.push_back((neighbor, next_path));
                }
            }
        }
    }

    for sub_path in paths.values_mut() {
        for path in sub_path.iter_mut() {
            path.push(KeyA);
        }
    }
    paths
}

fn find_all_shortests(
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>> {
    keypad
        .iter()
        .map(|(coord, key)| (*key, find_shortests(*coord, keypad)))
        .collect()
}

fn calculate_complexities(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let numpad: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Key7),
        ((0, 1), Key8),
        ((0, 2), Key9),
        ((1, 0), Key4),
        ((1, 1), Key5),
        ((1, 2), Key6),
        ((2, 0), Key1),
        ((2, 1), Key2),
        ((2, 2), Key3),
        ((3, 0), Empty),
        ((3, 1), Key0),
        ((3, 2), KeyA),
    ]
    .into_iter()
    .collect();

    let control: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Empty),
        ((0, 1), KeyUp),
        ((0, 2), KeyA),
        ((1, 0), KeyLeft),
        ((1, 1), KeyDown),
        ((1, 2), KeyRight),
    ]
    .into_iter()
    .collect();

    let codes = get_input(file_path)?;
    let mut complexities = 0;
    let shortests_paths_numpad = find_all_shortests(&numpad);
    let shortests_paths_control = find_all_shortests(&control);

    for (code, value) in codes {
        let mut previous_key = KeyA;
        let mut len = 0;
        for key in code {
            len += shortests_paths_numpad
                .get(&previous_key)
                .and_then(|paths| paths.get(&key))
                .map(|v| v[0].len())
                .unwrap_or(0);
            previous_key = key;
        }
        complexities += value * len;
    }

    Ok(complexities)
}

pub fn run() {
    let file_path = "src/days/inputs/day21.txt";
    match calculate_complexities(file_path) {
        Ok(complexities) => println!("Total complexities: {}", complexities),
        Err(e) => eprintln!("Error calculating complexities: {}", e),
    }
}
