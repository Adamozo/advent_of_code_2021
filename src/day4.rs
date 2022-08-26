use anyhow::*;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day4;

impl DaySolver for Day4 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day4", "data/day4.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut input = _s.split("\n\n");

        let bingo_numbers: Vec<usize> = input
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let max_value = bingo_numbers.iter().max().unwrap();

        let mut boards: Vec<Board> = input.map(|board| Board::new(board, max_value)).collect();

        for current_number in bingo_numbers {
            for board in &mut boards {
                board.mark_number(&current_number);

                if board.is_solved(&current_number) {
                    return Ok(board.unmarked_sum() * current_number);
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

#[derive(Debug, PartialEq, Eq)]
struct Board {
    fields_states:   Vec<Vec<FieldState>>,
    number_location: Vec<Option<Location>>,
}

impl Board {
    fn new(input: &str, max_value: &usize) -> Self {
        let fields_states = vec![vec![FieldState::Unmarked; 5]; 5];
        let mut number_location: Vec<Option<Location>> = vec![None; max_value + 1];

        for (row, line) in input.lines().enumerate() {
            for (column, number) in line
                .split(' ')
                .filter(|cell| !cell.is_empty())
                .map(|num| num.parse::<usize>().unwrap())
                .enumerate()
            {
                number_location[number] = Some((row, column));
            }
        }

        Self {
            fields_states,
            number_location,
        }
    }

    fn mark_number(&mut self, number: &usize) {
        if let Some((row, column)) = self.number_location[*number] {
            self.fields_states[row][column] = FieldState::Marked;
        }
    }

    fn is_solved(&self, number: &usize) -> bool {
        if let Some((row, column)) = self.number_location[*number] {
            return !self.fields_states[row]
                .iter()
                .any(|field| *field == FieldState::Unmarked)
                || !(0..self.fields_states.len())
                    .any(|row| self.fields_states[row][column] == FieldState::Unmarked);
        }

        false
    }

    fn unmarked_sum(&self) -> usize {
        self.number_location
            .iter()
            .enumerate()
            .filter(|(_number, location)| location.is_some())
            .map(|(number, location)| {
                let (row, column) = location.unwrap();
                (number, row, column)
            })
            .filter(|(_number, row, column)| {
                self.fields_states[*row][*column] == FieldState::Unmarked
            })
            .map(|(number, _, _)| number)
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

    #[test]
    fn day4_board_new() {
        let input = r#"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19"#;

        let number_location = vec![
            Some((0, 4)),
            Some((4, 0)),
            Some((1, 1)),
            Some((3, 2)),
            Some((1, 3)),
            Some((3, 4)),
            Some((3, 0)),
            Some((2, 4)),
            Some((1, 0)),
            Some((2, 1)),
            Some((3, 1)),
            Some((0, 3)),
            Some((4, 1)),
            Some((0, 1)),
            Some((2, 2)),
            Some((4, 3)),
            Some((2, 3)),
            Some((0, 2)),
            Some((3, 3)),
            Some((4, 4)),
            Some((4, 2)),
            Some((2, 0)),
            Some((0, 0)),
            Some((1, 2)),
            Some((1, 4)),
            None,
            None,
        ];
        let fields_states = vec![vec![FieldState::Unmarked; 5]; 5];

        assert_eq!(
            Board::new(input, &26),
            Board {
                fields_states,
                number_location
            }
        )
    }

    #[test]
    fn day4_board_mark_number() {
        let input = r#"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19"#;

        let mut board = Board::new(input, &26);
        board.mark_number(&26);
        board.mark_number(&22);

        assert_eq!(board.fields_states[0][0], FieldState::Marked)
    }

    #[test]
    fn day4_board_is_solved_false() {
        let input = r#"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19"#;

        let board = Board::new(input, &26);

        assert_eq!(board.is_solved(&11), false)
    }

    #[test]
    fn day4_board_is_solved_true() {
        let number_location = vec![
            Some((0, 4)),
            Some((4, 0)),
            Some((1, 1)),
            Some((3, 2)),
            Some((1, 3)),
            Some((3, 4)),
            Some((3, 0)),
            Some((2, 4)),
            Some((1, 0)),
            Some((2, 1)),
            Some((3, 1)),
            Some((0, 3)),
            Some((4, 1)),
            Some((0, 1)),
            Some((2, 2)),
            Some((4, 3)),
            Some((2, 3)),
            Some((0, 2)),
            Some((3, 3)),
            Some((4, 4)),
            Some((4, 2)),
            Some((2, 0)),
            Some((0, 0)),
            Some((1, 2)),
            Some((1, 4)),
            None,
            None,
        ];
        let fields_states = vec![vec![FieldState::Marked; 5]; 5];

        let board = Board {
            number_location,
            fields_states,
        };

        assert_eq!(board.is_solved(&1), true)
    }

    #[test]
    fn day4_board_unmarked_sum() {
        let number_location = vec![
            Some((0, 4)),
            Some((4, 0)),
            Some((1, 1)),
            Some((3, 2)),
            Some((1, 3)),
            Some((3, 4)),
            Some((3, 0)),
            Some((2, 4)),
            Some((1, 0)),
            Some((2, 1)),
            Some((3, 1)),
            Some((0, 3)),
            Some((4, 1)),
            Some((0, 1)),
            Some((2, 2)),
            Some((4, 3)),
            Some((2, 3)),
            Some((0, 2)),
            Some((3, 3)),
            Some((4, 4)),
            Some((4, 2)),
            Some((2, 0)),
            Some((0, 0)),
            Some((1, 2)),
            Some((1, 4)),
            None,
            None,
        ];
        let fields_states = vec![vec![FieldState::Unmarked; 5]; 5];

        let mut board = Board {
            number_location,
            fields_states,
        };

        board.fields_states[0][0] = FieldState::Marked;
        board.fields_states[1][1] = FieldState::Marked;
        board.fields_states[2][2] = FieldState::Marked;
        board.fields_states[3][3] = FieldState::Marked;
        board.fields_states[4][4] = FieldState::Marked;

        assert_eq!(board.unmarked_sum(), 225)
    }
}
