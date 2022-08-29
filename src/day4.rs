use anyhow::*;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

pub struct Day4;

impl DaySolver for Day4 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day4", "data/day4.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (called_numbers, boards) = s.split_once("\n\n").unwrap();

        let mut boards: Vec<Board> = boards
            .split("\n\n")
            .filter_map(|board| board.parse::<Board>().ok())
            .collect();

        for current_number in called_numbers
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
        {
            for board in boards.iter_mut() {
                if let Some(output) = board.call_number(&current_number) {
                    return Ok(output);
                }
            }
        }

        Err(anyhow!("Can't solve any board"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldState {
    Marked,
    Unmarked,
}

type Location = (usize, usize);
type Number = usize;

#[derive(Debug, PartialEq, Eq)]
struct Board {
    fields_states:   Vec<Vec<FieldState>>,
    number_location: HashMap<Number, Location>,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields_states = vec![vec![FieldState::Unmarked; 5]; 5];
        let mut number_location = HashMap::default();

        for (row, line) in input.lines().enumerate() {
            for (column, number) in line
                .split(' ')
                .filter(|cell| !cell.is_empty())
                .map(|num| num.parse::<usize>().unwrap())
                .enumerate()
            {
                number_location.insert(number, (row, column));
            }
        }

        Ok(Self {
            fields_states,
            number_location,
        })
    }
}

impl Board {
    fn call_number(&mut self, number: &usize) -> Option<usize> {
        if let Some((row, column)) = self.number_location.get(number) {
            self.fields_states[*row][*column] = FieldState::Marked;

            let is_solved = |row: &usize, column: &usize| {
                !self.fields_states[*row]
                    .iter()
                    .any(|field| *field == FieldState::Unmarked)
                    || !(0..self.fields_states.len())
                        .any(|row| self.fields_states[row][*column] == FieldState::Unmarked)
            };

            if is_solved(row, column) {
                let res = self.unmarked_sum();
                return Some(res * number);
            }
        }

        None
    }

    fn unmarked_sum(&self) -> usize {
        self.number_location
            .iter()
            .filter(|(_number, (row, column))| {
                self.fields_states[*row][*column] == FieldState::Unmarked
            })
            .map(|(number, _)| number)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day4::solve_default_file().unwrap(), 4512)
    }
}
