use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::cmp::min;

pub struct Day7;

impl DaySolver for Day7 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day7", "data/day7.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let positions: Vec<i32> = s
            .split(',')
            .map(|position| position.parse::<i32>().unwrap())
            .collect();
            
        let maximum_position = positions.iter().max().unwrap();

        let cost = |location: i32| -> i32 {
            positions
                .iter()
                .map(|position| {
                    let res = location - position;
                    res.abs()
                })
                .sum()
        };

        let all_positions = 0..=*maximum_position;

        let result = all_positions.fold(i32::MAX, |result, location| min(result, cost(location)));

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day7::solve_default_file().unwrap(), 37)
    }
}
