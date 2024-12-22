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
}

struct PerBlockDefragmenter;

impl PerBlockDefragmenter {
    fn gaps_exist(filesystem: &Filesystem) -> bool {
        let partition = &filesystem.sectors;
        let mut gaps = 0;

        for i in 0..(partition.len() - 1) {
            if matches!(partition[i], Sector::File(_)) && matches!(partition[i + 1], Sector::FreeSpace) {
                gaps += 1;
                if gaps > 1 {
                    return true;
                }
            }
        }
        false
    }

    fn defragment(filesystem: &mut Filesystem) {
        while Self::gaps_exist(filesystem) {
            let free_space = filesystem
                .sectors
                .iter()
                .enumerate()
                .filter(|(_, sector)| matches!(sector, Sector::FreeSpace))
                .map(|(index, _)| index)
                .collect::<Vec<_>>();

            let files = filesystem
                .sectors
                .iter()
                .enumerate()
                .filter(|(_, sector)| matches!(sector, Sector::File(_)))
                .map(|(index, sector)| (index, sector.clone()))
                .collect::<Vec<_>>();

            if let Some(&free_idx) = free_space.first() {
                if let Some(&(file_idx, ref file_data)) = files.last() {
                    filesystem.sectors[free_idx] = file_data.clone();
                    filesystem.sectors[file_idx] = Sector::FreeSpace;
                }
            }
        }
    }
}

struct PerFileDefragmenter;

impl PerFileDefragmenter {
    fn build_file_map(filesystem: &Filesystem) -> Vec<(u32, Vec<usize>, usize)> {
        let mut file_map: std::collections::BTreeMap<u32, (Vec<usize>, usize)> = std::collections::BTreeMap::new();
    
        for (index, sector) in filesystem.sectors.iter().enumerate() {
            if let Sector::File(file_id) = sector {
                let entry = file_map.entry(*file_id).or_insert((Vec::new(), 0));
                entry.0.push(index); // Add index
                entry.1 += 1;        // Increment size
            }
        }
    
        // Convert map to a vector and unpack the inner tuple
        let mut file_map_vec: Vec<_> = file_map
            .into_iter()
            .map(|(file_id, (indexes, size))| (file_id, indexes, size)) // Unpack the tuple
            .collect();
    
        // Sort by descending file ID
        file_map_vec.sort_by_key(|(file_id, _, _)| std::cmp::Reverse(*file_id));
        file_map_vec
    }
    

    fn is_free_space(filesystem: &Filesystem, start: usize, end: usize) -> bool {
        for i in start..end {
            if let Sector::File(_) = filesystem.sectors[i] {
                return false;
            }
        }
        true
    }

    fn defragment(filesystem: &mut Filesystem) {
        let file_map = Self::build_file_map(filesystem);

        for (file_id, indexes, size) in file_map {
            let required_space = size;
            let first_file_index = *indexes.iter().min().unwrap();

            for x in 0..first_file_index {
                if x + required_space <= filesystem.sectors.len()
                    && Self::is_free_space(filesystem, x, x + required_space)
                {
                    // Move the file to this free space
                    for (old_pos, new_pos) in indexes.iter().zip(x..(x + required_space)) {
                        filesystem.sectors[new_pos] = filesystem.sectors[*old_pos].clone();
                        filesystem.sectors[*old_pos] = Sector::FreeSpace;
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

    // Part 1: Block-by-block defragmentation
    let mut filesystem_part1 = Filesystem::new(file_path);
    PerBlockDefragmenter::defragment(&mut filesystem_part1);
    let checksum_part1 = filesystem_part1.get_checksum();
    println!("Part 1 Checksum: {}", checksum_part1);

    // Part 2: File-by-file defragmentation
    let mut filesystem_part2 = Filesystem::new(file_path);
    PerFileDefragmenter::defragment(&mut filesystem_part2);
    let checksum_part2 = filesystem_part2.get_checksum();
    println!("Part 2 Checksum: {}", checksum_part2);
}
