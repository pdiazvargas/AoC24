use anyhow::Result;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Board {
    rows: usize,
    cols: usize,
    data: HashMap<(usize, usize), char>,
}

impl Board {
    pub fn new(rows: usize, cols: usize, data: HashMap<(usize, usize), char>) -> Self {
        Self { rows, cols, data }
    }

    pub fn print(&self) {
        for (row, col) in self.data.keys() {
            println!("{} {} {}", row, col, self.data[&(*row, *col)]);
        }
    }

    pub fn counts_a(&self) -> usize {
        let mut count = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let values = self.square_values_a(row, col);

                count += values
                    .iter()
                    .filter(|v| *v == "XMAS" || *v == "SAMX")
                    .count();
            }
        }

        count
    }

    pub fn counts_b(&self) -> usize {
        let mut count = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let values = self.diag_values_b(row, col);
                match (values[0].as_str(), values[1].as_str()) {
                    ("MAS", "MAS") | ("SAM", "SAM") | ("MAS", "SAM") | ("SAM", "MAS") => {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }

        count
    }

    pub fn diag_values_b(&self, row: usize, col: usize) -> [String; 2] {
        let first = [(row, col), (row + 1, col + 1), (row + 2, col + 2)];
        let second = [(row + 2, col), (row + 1, col + 1), (row, col + 2)];

        [self.value(first), self.value(second)]
    }

    fn value(&self, coords: [(usize, usize); 3]) -> String {
        String::from(
            coords
                .iter()
                .map(|(r, c)| self.data.get(&(*r, *c)).unwrap_or(&'*'))
                .collect::<String>(),
        )
    }

    pub fn square_values_a(&self, row: usize, col: usize) -> Vec<String> {
        let mut coords = vec![];

        coords.push([(row, col), (row, col + 1), (row, col + 2), (row, col + 3)]);
        coords.push([(row, col), (row + 1, col), (row + 2, col), (row + 3, col)]);
        coords.push([
            (row, col),
            (row + 1, col + 1),
            (row + 2, col + 2),
            (row + 3, col + 3),
        ]);
        coords.push([
            (row + 3, col),
            (row + 2, col + 1),
            (row + 1, col + 2),
            (row, col + 3),
        ]);

        let values = coords
            .iter()
            .map(|coord| {
                String::from(
                    coord
                        .iter()
                        .map(|(r, c)| self.data.get(&(*r, *c)).unwrap_or(&'*'))
                        .collect::<String>(),
                )
            })
            .collect();

        values
    }
}

pub fn load_data() -> Result<Board> {
    let content = fs::read_to_string("./src/input/04.input")?;
    let mut data = HashMap::new();
    let rows = content.lines().count();
    let cols = content.lines().next().unwrap().len();

    for (row, line) in content.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            data.insert((row, col), val);
        }
    }

    Ok(Board::new(rows, cols, data))
}

pub fn part_a() {
    let board = load_data().unwrap();
    println!("Part a: {}", board.counts_a());
}

pub fn part_b() {
    let board = load_data().unwrap();
    println!("Part b: {}", board.counts_b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board() {
        let board = load_data().unwrap();
        println!("{:?}", board.counts_b());
    }
}
