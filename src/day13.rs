use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashSet as HashSet;
use std::str::FromStr;

pub struct Day13;

type Point = (u8, u8);

impl DaySolver for Day13 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day13", "data/day13.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (coordinates, folds) = s.split_once("\n\n").unwrap();

        let folds = parse_folds(folds);
        let dots: HashSet<Point> = parse_input_coordinates(coordinates)
            .into_iter()
            .map(|point| convert_point(&folds[0], point))
            .collect();

        Ok(dots.len())
    }
}

type Line = u8;

#[derive(Debug)]
enum Fold {
    Horizontally(Line),
    Vertically(Line),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('=').unwrap();

        let value = right.parse::<Line>()?;

        let result = match left.chars().last().unwrap() {
            'x' => Fold::Vertically(value),
            'y' => Fold::Horizontally(value),
            _ => unreachable!(),
        };

        Ok(result)
    }
}

fn parse_folds(s: &str) -> Vec<Fold> {
    s.lines()
        .map(|line| line.parse::<Fold>().unwrap())
        .collect()
}

fn parse_input_coordinates(s: &str) -> HashSet<Point> {
    let result: HashSet<Point> = s
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect();

    result
}

fn convert_point(fold: &Fold, point: Point) -> Point {
    let (x, y) = point;
    match fold {
        Fold::Horizontally(line) if *line < y => (x, *line * 2 - y),
        Fold::Vertically(line) if *line < x => (*line * 2 - x, y),
        _ => (x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day13::solve_default_file().unwrap(), 17)
    }
}
