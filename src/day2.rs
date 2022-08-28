use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::str::FromStr;

pub struct Day2;

impl DaySolver for Day2 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day2", "data/day2.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        use MoveDirection::*;

        let (vertical, horizontal) = s
            .lines()
            .filter_map(|line| line.parse::<MoveDirection>().ok())
            .fold(
                (0, 0),
                |(vertical, horizontal), move_direction| match move_direction {
                    Horizontal(value) => (vertical, horizontal + value),
                    Vertical(value) => (vertical + value, horizontal),
                },
            );
        Ok(vertical * horizontal)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveDirection {
    Horizontal(i32),
    Vertical(i32),
}

impl FromStr for MoveDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MoveDirection::*;

        let (command, value) = s.split_once(' ').unwrap();

        let value = value.parse::<i32>()?;

        let move_direction = match command {
            "forward" => Horizontal(value),
            "up" => Vertical(-value),
            "down" => Vertical(value), // it's a submarine that's why down is with +
            _ => unreachable!(),
        };

        Ok(move_direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day2::solve_default_file().unwrap(), 150)
    }

    #[test_case("forward 5" => MoveDirection::Horizontal(5))]
    #[test_case("down 5" => MoveDirection::Vertical(5))]
    #[test_case("up 5" => MoveDirection::Vertical(-5))]
    fn day2_fromstr(line: &str) -> MoveDirection {
        line.parse::<MoveDirection>().unwrap()
    }
}
