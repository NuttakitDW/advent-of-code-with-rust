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

    fn get_sector(&self, index: usize) -> &Sector {
        &self.sectors[index]
    }

    fn set_sector(&mut self, index: usize, sector: Sector) {
        self.sectors[index] = sector;
    }

    fn get_partition(&self) -> &Vec<Sector> {
        &self.sectors
    }

    fn get_free_space(&self) -> Vec<usize> {
        self.sectors
            .iter()
            .enumerate()
            .filter(|(_, sector)| matches!(sector, Sector::FreeSpace))
            .map(|(index, _)| index)
            .collect()
    }

    fn get_files(&self) -> Vec<(usize, Sector)> {
        self.sectors
            .iter()
            .enumerate()
            .filter(|(_, sector)| matches!(sector, Sector::File(_)))
            .map(|(index, sector)| (index, sector.clone()))
            .collect()
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
        let partition = filesystem.get_partition();
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
            let free_space = filesystem.get_free_space();
            let files = filesystem.get_files();

            if let Some(&free_idx) = free_space.first() {
                if let Some(&(file_idx, ref file_data)) = files.last() {
                    filesystem.set_sector(free_idx, file_data.clone());
                    filesystem.set_sector(file_idx, Sector::FreeSpace);
                }
            }
        }
    }
}

pub fn run() {
    println!("Day 9!!");
    let file_path = "src/days/inputs/day9.txt";
    let mut filesystem = Filesystem::new(file_path);

    PerBlockDefragmenter::defragment(&mut filesystem);
    let checksum = filesystem.get_checksum();
    println!("Checksum: {}", checksum);
}
