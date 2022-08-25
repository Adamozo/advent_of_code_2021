use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::iter::Sum;
use std::str::FromStr;

pub struct Day2;

impl DaySolver for Day2 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day2", "data/day2.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let location: Location = _s
            .lines()
            .map(|line| line.parse::<Location>().unwrap())
            .sum();
        Ok(location.depth * location.horizontal)
    }
}

#[derive(PartialEq, Debug)]
struct Location {
    horizontal: i32,
    depth:      i32,
}

impl FromStr for Location {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value) = s.split_once(' ').unwrap();

        let movement = value.parse::<i32>()?;

        match command {
            "forward" => Ok(Location {
                horizontal: movement,
                depth:      0,
            }),
            "down" => Ok(Location {
                horizontal: 0,
                depth:      movement,
            }),
            "up" => Ok(Location {
                horizontal: 0,
                depth:      -movement,
            }),
            _ => unreachable!(),
        }
    }
}

impl Sum for Location {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Location>,
    {
        let mut res = Location {
            horizontal: 0,
            depth:      0,
        };

        for location in iter {
            res.depth += location.depth;
            res.horizontal += location.horizontal;
        }

        res
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

    #[test_case("forward 5" => Location {horizontal: 5, depth: 0})]
    #[test_case("down 5" => Location {horizontal: 0, depth: 5})]
    #[test_case("up 5" => Location {horizontal: 0, depth: -5})]
    fn day2_fromstr(line: &str) -> Location {
        line.parse::<Location>().unwrap()
    }
}
