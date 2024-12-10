use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
struct Board {
    pub rows: usize,
    pub cols: usize,
    pub data: HashMap<(i32, i32), i32>,
}

impl Board {
    fn value_at(&self, row: i32, col: i32) -> Option<&i32> {
        self.data.get(&(row, col))
    }

    fn trace(&self, path: [(i32, i32); 10]) {
        path.iter()
            .map(|(row, col)| self.value_at(*row, *col).unwrap())
            .for_each(|val| print!("{}", val));
    }
}

fn load_data() -> Result<Board> {
    let content = fs::read_to_string("./src/input/10.input")?;
    let mut data = HashMap::new();
    let rows = content.lines().count();
    let cols = content.lines().next().unwrap().len();

    for (row, line) in content.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            let new_val = match val {
                '.' => 100,
                _ => val.to_digit(10).unwrap_or(101),
            };

            data.insert((row as i32, col as i32), new_val as i32);
        }
    }

    Ok(Board { rows, cols, data })
}

fn find_paths(board: &Board, row: i32, col: i32, acc: String, paths: &mut Vec<String>) {
    let possibilities = [
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];

    let current = board.value_at(row, col).unwrap();
    if *current == 9 {
        paths.push(acc);
        return;
    }

    for (next_row, next_col) in possibilities {
        if let Some(new_val) = board.value_at(next_row, next_col) {
            if new_val - current == 1 {
                let mut new_path = acc.clone();
                new_path.push_str(&format!("({}, {})", next_row, next_col));
                find_paths(board, next_row, next_col, new_path, paths);
            }
        }
    }
}

fn trailheads(board: &Board) -> Vec<(i32, i32)> {
    board
        .data
        .iter()
        .filter(|(_k, v)| **v == 0)
        .map(|(k, _)| *k)
        .collect()
}

pub fn part_a() {
    let mut disk = load_data().unwrap();
    println!("Part a: {}", 23);
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
        let board = load_data().unwrap();

        let trailheads = trailheads(&board);
        let mut score = 0;
        // println!("Trailheads {:?}", trailheads);

        for (row, col) in trailheads {
            let mut paths = Vec::new();
            find_paths(&board, row, col, String::default(), &mut paths);
            score += paths.len();
            println!("Paths ({row}, {col}) {:?}", paths.len());
        }

        println!("Score: {}", score);
    }

    #[test]
    fn test_disk_layout_2() {
        let board = load_data().unwrap();

        let trailheads = trailheads(&board);
        // println!("Trailheads {:?}", trailheads);

        let mut paths = Vec::new();
        let (row, col) = (0, 0);
        find_paths(&board, row, col, String::default(), &mut paths);

        println!("Paths ({row}, {col}) {:?}", paths.len());
    }
}
