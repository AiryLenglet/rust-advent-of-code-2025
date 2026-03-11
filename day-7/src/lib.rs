use std::result;
use crate::Cell::{Beam, Manifold};

struct TachyonManifolds {
    cells: Vec<Vec<Cell>>,
}

impl TachyonManifolds {
    fn parse(input: &str) -> Result<TachyonManifolds> {
        let cells = input
            .lines()
            .map(|line| line.chars().map(|c| Cell::from_char(&c)).collect::<Result<Vec<Cell>>>())
            .collect::<Result<Vec<Vec<Cell>>>>()?;
        Ok(TachyonManifolds { cells })
    }

    fn open(&mut self) -> u64 {
        let row_count = self.cells.len();
        let col_count = self.cells[0].len();
        let mut split_count: u64 = 0;
        for row_index in 1..row_count {
            for col_index in 0..col_count {
                let above_cell = self.cells[row_index-1][col_index].clone();
                let cell = &self.cells[row_index][col_index];
                match cell {
                    Cell::Empty if above_cell == Manifold || above_cell == Beam => {
                        self.try_beam(row_index, col_index);
                    }
                    Cell::Splitter if above_cell == Manifold || above_cell == Beam => {
                        split_count += 1;
                        if col_index > 0 {
                            self.try_beam(row_index, col_index-1);
                        }
                        self.try_beam(row_index, col_index+1);
                    }
                    _ => {}
                }
            }
        }
        split_count
    }

    fn try_beam(&mut self, row_index: usize, col_index: usize) {
        if col_index > 0 && let Cell::Empty = &self.cells[row_index][col_index] {
            self.cells[row_index][col_index] = Beam;
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Manifold,
    Empty,
    Beam,
    Splitter,
}

impl Cell {
    fn from_char(c: &char) -> Result<Cell> {
        match c {
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Manifold),
            '|' => Ok(Cell::Beam),
            '^' => Ok(Cell::Splitter),
            other => Err(ManifoldError {
                cause: format!("Unable to parse cell {}", other),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ManifoldError {
    cause: String,
}

type Result<T> = result::Result<T, ManifoldError>;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_sample() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut manifolds = TachyonManifolds::parse(&input).unwrap();
        assert_eq!(manifolds.open(), 21);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let mut manifolds = TachyonManifolds::parse(&input).unwrap();
        assert_eq!(manifolds.open(), 1537);
    }
}
