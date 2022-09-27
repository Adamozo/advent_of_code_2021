use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashSet as HashSet;
use std::str::FromStr;

pub struct Day13;

type Point = (i8, i8);

impl DaySolver for Day13 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day13", "data/day13.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (coordinates, folds) = s.split_once("\n\n").unwrap();

        let mut dots: HashSet<Point> = parse_input_coordinates(coordinates);
        let folds = parse_folds(folds);

        dots = dots.iter().fold(HashSet::default(), |mut new_dots, point| {
            let _unused = new_dots.insert(convert_point(&folds[0], *point));
            new_dots
        });

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
            (x.parse::<i8>().unwrap(), y.parse::<i8>().unwrap())
        })
        .collect();

    result
}

fn convert_point(fold: &Fold, point: Point) -> Point {
    match fold {
        Fold::Horizontally(line) => convert_point_horizontal(*line as i8, point),
        Fold::Vertically(line) => convert_point_vertical(*line as i8, point),
    }
}

fn convert_point_horizontal(symmetry_line: i8, point: Point) -> Point {
    let (x, y) = point;

    let distance = (symmetry_line - y).abs();

    (x, symmetry_line - distance)
}

fn convert_point_vertical(symmetry_line: i8, point: Point) -> Point {
    let (x, y) = point;

    let distance = (symmetry_line - x).abs();

    (symmetry_line - distance, y)
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
