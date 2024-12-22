use std::fs;

#[derive(Clone, Debug)]
enum Sector {
    File(u32),     // Represents a file with a specific ID
    FreeSpace,     // Represents free space
}

struct Filesystem {
    sectors: Vec<Sector>,
}

impl Filesystem {
    fn new(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read input file");
        let mut sectors = Vec::new();
        let mut is_file = true;
        let mut file_id = 0;

        for c in input.trim().chars() {
            let length = c.to_digit(10).expect("Invalid input");
            if is_file {
                sectors.extend(vec![Sector::File(file_id); length as usize]);
                file_id += 1;
            } else {
                sectors.extend(vec![Sector::FreeSpace; length as usize]);
            }
            is_file = !is_file;
        }

        Filesystem { sectors }
    }

    fn get_checksum(&self) -> u64 {
        self.sectors
            .iter()
            .enumerate()
            .filter_map(|(pos, sector)| {
                if let Sector::File(id) = sector {
                    Some((pos as u64) * (*id as u64))
                } else {
                    None
                }
            })
            .sum()
    }

    fn compact_block_by_block(&mut self) {
        for current_index in (0..self.sectors.len()).rev() {
            if let Sector::File(file_id) = self.sectors[current_index] {
                // Find the leftmost free space
                if let Some(target_index) = self.sectors.iter().position(|s| matches!(s, Sector::FreeSpace)) {
                    // Move block to free space
                    self.sectors[target_index] = Sector::File(file_id);
                    self.sectors[current_index] = Sector::FreeSpace;
                }
            }
        }
    }

    fn find_free_spans(&self) -> Vec<(usize, usize)> {
        let mut spans = Vec::new();
        let mut start = None;

        for (i, sector) in self.sectors.iter().enumerate() {
            match (start, sector) {
                (None, Sector::FreeSpace) => start = Some(i),
                (Some(s), Sector::File(_)) => {
                    spans.push((s, i - 1));
                    start = None;
                }
                _ => {}
            }
        }

        if let Some(s) = start {
            spans.push((s, self.sectors.len() - 1));
        }

        spans
    }

    fn compact_file_by_file(&mut self) {
        let mut files = Vec::new();
        let mut file_id = None;

        // Collect all file IDs and their positions
        for (i, sector) in self.sectors.iter().enumerate() {
            match (file_id, sector) {
                (None, Sector::File(id)) => {
                    file_id = Some(*id);
                    files.push((*id, vec![i]));
                }
                (Some(id), Sector::File(cur_id)) if id == *cur_id => {
                    files.last_mut().unwrap().1.push(i);
                }
                (_, _) => file_id = None,
            }
        }

        // Sort files by decreasing file ID
        files.sort_by_key(|(id, _)| std::cmp::Reverse(*id));

        // Move files to leftmost free space if possible
        for (id, positions) in files {
            let file_length = positions.len();
            let free_spans = self.find_free_spans();

            for (start, end) in free_spans {
                if (end - start + 1) >= file_length {
                    let target_positions: Vec<usize> = (start..(start + file_length)).collect();

                    for (&old, &new) in positions.iter().zip(target_positions.iter()) {
                        self.sectors[new] = Sector::File(id);
                        self.sectors[old] = Sector::FreeSpace;
                    }
                    break;
                }
            }
        }
    }
}

pub fn run() {
    println!("Day 9!!");
    let file_path = "src/days/inputs/day9.txt";

    // Part 1: Block-by-block compaction
    let mut filesystem_part1 = Filesystem::new(file_path);
    filesystem_part1.compact_block_by_block();
    let checksum_part1 = filesystem_part1.get_checksum();
    println!("Part 1 Checksum: {}", checksum_part1);

    // Part 2: File-by-file compaction
    let mut filesystem_part2 = Filesystem::new(file_path);
    filesystem_part2.compact_file_by_file();
    let checksum_part2 = filesystem_part2.get_checksum();
    println!("Part 2 Checksum: {}", checksum_part2);
}
