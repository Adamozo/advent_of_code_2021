use std::collections::VecDeque;

use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day10;

const BRACKETS_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

impl DaySolver for Day10 {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day10", "data/day10.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let res = s.lines().map(line_syntax_error_score).sum();
        Ok(res)
    }
}

fn line_syntax_error_score(line: &str) -> u32 {
    let mut stack: VecDeque<char> = VecDeque::new();

    'next_bracket: for bracket in line.chars() {
        let is_valid = false;
        for (left_bracket, right_bracket) in BRACKETS_PAIRS {
            if bracket == left_bracket {
                stack.push_front(bracket);
                continue 'next_bracket;
            } else if bracket == right_bracket && stack.front() == Some(&left_bracket) {
                let _ = stack.pop_front();
                continue 'next_bracket;
            }
        }
        if !is_valid {
            return get_char_score(bracket);
        }
    }
    0
}

fn get_char_score(bracket: char) -> u32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(')' => 3)]
    #[test_case(']' => 57)]
    #[test_case('}' => 1197)]
    #[test_case('>' => 25137)]
    fn day10_get_char_score(bracket: char) -> u32 {
        get_char_score(bracket)
    }

    #[test_case("{([(<{}[<>[]}>{[]{[(<()>" => 1197)]
    #[test_case("[[<[([]))<([[{}[[()]]]" => 3)]
    #[test_case("[{[{({}]{}}([{[{{{}}([]" => 57)]
    fn day10_line_syntax_error_score(line: &str) -> u32 {
        line_syntax_error_score(line)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day10::solve_default_file().unwrap(), 26397)
    }
}
