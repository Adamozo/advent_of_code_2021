use anyhow::*;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;

pub struct Day4b;

impl DaySolver for Day4b {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day4", "data/day4.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (called_numbers, boards) = s.split_once("\n\n").unwrap();

        let called_numbers: Vec<usize> = called_numbers
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mut boards: Vec<Board> = boards.split("\n\n").map(Board::new).collect();

        for current_number in called_numbers {
            for board in boards.iter_mut() {
                if let Some(output) = board.call_number(current_number) {
                    return Ok(output);
                }
            }
        }

        Err(anyhow!("Can't solve any board"))
    }
}

type Location = (usize, usize);
type Number = usize;

#[derive(Debug, PartialEq, Eq)]
struct Board {
    number_location:     HashMap<Number, Location>,
    count_marked_in_row: Vec<u32>,
    count_marked_in_col: Vec<u32>,
    sum_unmarked:        usize,
    board_len:           u32,
}

impl Board {
    fn new(input: &str) -> Self {
        let mut number_location = HashMap::default();
        let mut sum_unmarked = 0;

        for (row, line) in input.lines().enumerate() {
            for (column, number) in line
                .split(' ')
                .filter(|cell| !cell.is_empty())
                .map(|num| num.parse::<usize>().unwrap())
                .enumerate()
            {
                sum_unmarked += number;
                number_location.insert(number, (row, column));
            }
        }

        Self {
            number_location,
            count_marked_in_col: vec![0; 5],
            count_marked_in_row: vec![0; 5],
            sum_unmarked,
            board_len: 5,
        }
    }

    fn call_number(&mut self, number: usize) -> Option<usize> {
        let (row, column) = &self.number_location[&number];

        self.sum_unmarked -= number;

        self.count_marked_in_row[*row] += 1;
        if self.count_marked_in_row[*row] == self.board_len {
            return Some(self.sum_unmarked * number);
        }

        self.count_marked_in_col[*column] += 1;
        if self.count_marked_in_col[*column] == self.board_len {
            return Some(self.sum_unmarked * number);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day4b::solve_default_file().unwrap(), 4512)
    }
}
