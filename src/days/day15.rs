#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from(dir: u8) -> Self {
        match dir {
            b'^' => Point::new(0, -1),
            b'v' => Point::new(0, 1),
            b'<' => Point::new(-1, 0),
            b'>' => Point::new(1, 0),
            _ => Point::new(0, 0),
        }
    }

    fn right() -> Self {
        Point::new(1, 0)
    }

    fn left() -> Self {
        Point::new(-1, 0)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn run() {
    let input = std::fs::read_to_string("src/days/inputs/day15.txt").expect("Failed to read input file");
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (mut grid, moves, mut robot) = parse(input, 1);

    for &m in &moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                robot = next;
            }
            b'O' => {
                let mut boxes = vec![next];
                let mut path = next + direction;

                while grid[path.y as usize][path.x as usize] == b'O' {
                    boxes.push(path);
                    path += direction;
                }

                if grid[path.y as usize][path.x as usize] == b'.' {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = b'O';
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    coordinates(&grid)
}

fn part2(input: &str) -> usize {
    let (mut grid, moves, mut robot) = parse(input, 2);

    for &m in &moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                robot = next;
            }
            side @ b'[' | side @ b']' => {
                let mut boxes = vec![next];

                if side == b'[' {
                    boxes.push(next + Point::right());
                } else {
                    boxes.push(next + Point::left());
                }

                let mut blocked = false;

                match m {
                    b'^' | b'v' => {
                        let mut current = boxes.clone();

                        while current.len() > 1 {
                            let mut next_boxes = Vec::new();

                            for b in current {
                                let path = b + direction;

                                match grid[path.y as usize][path.x as usize] {
                                    b'#' => {
                                        blocked = true;
                                        next_boxes.clear();
                                        break;
                                    }
                                    side @ b'[' | side @ b']' => {
                                        if !next_boxes.contains(&path) {
                                            boxes.push(path);
                                            next_boxes.push(path);

                                            if side == b'[' {
                                                boxes.push(path + Point::right());
                                                next_boxes.push(path + Point::right());
                                            } else {
                                                boxes.push(path + Point::left());
                                                next_boxes.push(path + Point::left());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            current = next_boxes;
                        }
                    }
                    b'<' | b'>' => {
                        let mut path = next + direction + direction;

                        while [b'[', b']'].contains(&grid[path.y as usize][path.x as usize]) {
                            boxes.push(path);
                            path += direction;
                        }

                        if grid[path.y as usize][path.x as usize] != b'.' {
                            blocked = true;
                        }
                    }
                    _ => {}
                }

                if !blocked {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = grid[b.y as usize][b.x as usize];
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    coordinates(&grid)
}

fn parse(input: &str, part: u8) -> (Vec<Vec<u8>>, Vec<u8>, Point) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut robot = None;

    (
        grid.lines()
            .enumerate()
            .map(|(y, line)| {
                if part == 1 {
                    line.bytes()
                        .enumerate()
                        .map(|(x, b)| {
                            if b == b'@' {
                                robot = Some(Point::new(x as i32, y as i32));
                                b'.'
                            } else {
                                b
                            }
                        })
                        .collect()
                } else {
                    line.bytes()
                        .enumerate()
                        .flat_map(|(x, b)| match b {
                            b'#' => [b'#', b'#'],
                            b'O' => [b'[', b']'],
                            b'.' => [b'.', b'.'],
                            b'@' => {
                                robot = Some(Point::new(x as i32 * 2, y as i32));
                                [b'.', b'.']
                            }
                            _ => unreachable!(),
                        })
                        .collect()
                }
            })
            .collect(),
        moves.lines().flat_map(|line| line.as_bytes().iter().copied()).collect(),
        robot.unwrap(),
    )
}

fn coordinates(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'O' || *b == b'[' {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
