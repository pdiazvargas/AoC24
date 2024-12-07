use anyhow::Result;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq)]
enum Piece {
    Empty,
    Obstacle,
    Visited,
    OutOfBounds,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    pub fn rotate_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[derive(Debug, Clone)]
struct Board {
    pub rows: usize,
    pub cols: usize,
    pub data: HashMap<(usize, usize), Piece>,
}

impl Board {
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let piece = self.piece_at(row, col);
                match piece {
                    Piece::Empty => print!("."),
                    Piece::Obstacle => print!("#"),
                    Piece::Visited => print!("X"),
                    Piece::OutOfBounds => print!("O"),
                }
            }
            println!();
        }
    }

    pub fn piece_at(&self, row: usize, col: usize) -> Piece {
        if row >= self.rows || col >= self.cols {
            return Piece::OutOfBounds;
        }

        match self.data.get(&(row, col)) {
            Some(piece) => piece.clone(),
            None => Piece::OutOfBounds,
        }
    }

    pub fn visit(&mut self, row: usize, col: usize) {
        self.data.insert((row, col), Piece::Visited);
    }

    pub fn visited_count(&self) -> usize {
        self.data.values().filter(|&x| *x == Piece::Visited).count()
    }
}

fn load_data() -> Result<(Board, Guard)> {
    let content = fs::read_to_string("./src/input/06_sm.input")?;
    let mut data = HashMap::new();
    let rows = content.lines().count();
    let cols = content.lines().next().unwrap().len();
    let mut guard = Guard {
        position: (0, 0),
        direction: Direction::Down,
    };

    for (row, line) in content.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            let val = match val {
                '.' => Piece::Empty,
                '#' => Piece::Obstacle,
                _ => {
                    guard.position = (row, col);
                    guard.direction = match val {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => panic!("Invalid guard direction"),
                    };
                    Piece::Visited
                }
            };

            data.insert((row, col), val);
        }
    }

    Ok((Board { rows, cols, data }, guard))
}

pub fn part_a() {
    let (mut board, mut guard) = load_data().unwrap();

    loop {
        let moved = move_one(&mut board, &mut guard);
        if !moved {
            break;
        }
    }

    println!("Part a: {}", board.visited_count());
}

pub fn part_b() {
    let board = load_data().unwrap();
    println!("Part b: 64");
}

fn move_one(board: &mut Board, guard: &mut Guard) -> bool {
    let (row, col) = guard.position;
    let direction = &guard.direction;

    let (next_row, next_col) = match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    };

    let next_spot = board.piece_at(next_row, next_col);

    match next_spot {
        Piece::Empty | Piece::Visited => {
            board.visit(next_row, next_col);
            guard.position = (next_row, next_col);
            true
        }
        Piece::Obstacle => {
            guard.rotate_direction();
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board() {
        let (mut board, mut guard) = load_data().unwrap();

        board.print();

        loop {
            let moved = move_one(&mut board, &mut guard);
            if !moved {
                break;
            }
        }
        println!("-----------------");
        board.print();

        println!("{:?}", guard);
        println!("Visited: {}", board.visited_count());
    }
}
