use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

use std::str::FromStr;

pub struct Day11;

const NEIGHBORS: [(i8, i8); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl DaySolver for Day11 {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day11", "data/day11.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut energy_level_board = s.parse::<EnergyLevelBoard>()?;

        let result: u32 = (1..=100).map(|_| energy_level_board.make_step()).sum();

        Ok(result)
    }
}

#[derive(PartialEq, Debug)]
struct EnergyLevelBoard {
    board:          Vec<Vec<u8>>,
    columns_number: usize,
    rows_number:    usize,
}

impl FromStr for EnergyLevelBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<Vec<u8>> = s
            .lines()
            .map(|line| line.bytes().map(|level| level - b'0').collect())
            .collect();
        let columns_number = board.len();
        let rows_number = board[0].len();

        Ok(Self {
            board,
            columns_number,
            rows_number,
        })
    }
}

impl EnergyLevelBoard {
    fn flash(&mut self, column: usize, row: usize) -> u32 {
        self.board[column][row] = 0;

        let mut result = 1;

        for (neighbor_column, neighbor_row) in NEIGHBORS.iter().map(|(col_offset, row_offset)| {
            (
                (column as i8 + col_offset) as usize,
                (row as i8 + row_offset) as usize,
            )
        }) {
            if let Some(value) = self
                .board
                .get_mut(neighbor_column)
                .and_then(|column_| column_.get_mut(neighbor_row))
            {
                if *value > 0 {
                    *value += 1;

                    if *value > 9 {
                        result += self.flash(neighbor_column, neighbor_row);
                    }
                }
            }
        }

        result
    }

    fn make_step(&mut self) -> u32 {
        let mut flashes = 0;

        for column in 0..self.columns_number {
            for row in 0..self.rows_number {
                self.board[column][row] += 1;
            }
        }

        for column in 0..self.columns_number {
            for row in 0..self.rows_number {
                if self.board[column][row] > 9 {
                    flashes += self.flash(column, row);
                }
            }
        }

        flashes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn energy_level_board_from_str() {
        let s = r#"91
88"#;
        let board = vec![vec![9, 1], vec![8, 8]];
        let columns_number = 2;
        let rows_number = 2;

        let res = EnergyLevelBoard {
            board,
            columns_number,
            rows_number,
        };
        assert_eq!(s.parse::<EnergyLevelBoard>().unwrap(), res)
    }

    #[test]
    fn energy_level_board_make_step() {
        let s = r#"91
88"#;

        let mut res = s.parse::<EnergyLevelBoard>().unwrap();
        assert_eq!(res.make_step(), 3)
    }

    #[test]
    fn energy_level_board_flash() {
        let s = r#"91
98"#;

        let mut res = s.parse::<EnergyLevelBoard>().unwrap();
        assert_eq!(res.flash(0, 0), 3)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day11::solve_default_file().unwrap(), 1656)
    }
}
