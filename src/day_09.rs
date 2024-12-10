use anyhow::Result;
use std::{char, fs};

#[derive(Debug, Clone, Copy)]
enum DiskBlock {
    File(u32),
    Free,
}

#[derive(Debug, Clone)]
struct DiskMap(Vec<DiskBlock>);

impl DiskMap {
    pub fn new(content: String) -> Self {
        let values = content
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        DiskMap(blocks_from_layout(values))
    }

    pub fn display_layout(&self) -> String {
        self.0
            .iter()
            .map(|block| match block {
                DiskBlock::File(block_id) => char::from_digit(*block_id, 10).unwrap_or('$'),
                DiskBlock::Free => '.',
            })
            .collect()
    }

    pub fn defragment(&mut self) {
        let mut left_index = 0;
        let mut right_index = self.0.len() - 1;
        let mut pad = true;

        while left_index < right_index - 1 && pad {
            if let Some(DiskBlock::Free) = self.0.get(left_index) {
                while let Some(DiskBlock::Free) = self.0.get(right_index) {
                    right_index -= 1;
                }

                self.0.swap(left_index, right_index);

                // println!("{}", self.display_layout());
            }
            left_index += 1;
            pad = true;
        }
    }

    pub fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(index, block)| match block {
                DiskBlock::File(block_id) => index * (*block_id as usize),
                DiskBlock::Free => 0,
            })
            .sum()
    }
}

fn blocks_from_layout(entries: Vec<u32>) -> Vec<DiskBlock> {
    let mut id_number = 0;
    let mut layout = Vec::new();

    for (index, entry) in entries.iter().enumerate() {
        let block = if index % 2 == 0 {
            // file
            id_number += 1;
            DiskBlock::File(id_number - 1)
        } else {
            // free space
            DiskBlock::Free
        };

        for _ in 0..*entry {
            layout.push(block);
        }
    }

    layout
}

fn load_data() -> Result<DiskMap> {
    let content = fs::read_to_string("./src/input/09.input")?;
    Ok(DiskMap::new(content))
}

pub fn part_a() {
    let mut disk = load_data().unwrap();

    disk.defragment();

    println!("Part a: {}", disk.checksum());
}

pub fn part_b() {
    let equations = load_data().unwrap();
    println!("Part b: 43");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_disk_layout_1() {
        let disk = DiskMap::new("12345".into());

        assert_eq!(&disk.display_layout(), "0..111....22222")
    }

    #[test]
    fn test_disk_defragment_sm() {
        let mut disk = DiskMap::new("2333133121414131402".into());

        disk.defragment();

        assert_eq!(disk.checksum(), 1928)
    }

    #[test]
    fn test_disk_layout_2() {
        let disk = DiskMap::new("2333133121414131402".into());

        assert_eq!(
            &disk.display_layout(),
            "00...111...2...333.44.5555.6666.777.888899"
        )
    }
}
