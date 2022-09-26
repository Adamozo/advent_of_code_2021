use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use aoc_utils::Grid;
use itertools::Itertools;

pub struct Day13b;

type Point = (i8, i8);

impl DaySolver for Day13b {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day13", "data/day13.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (coordinates, folds) = s.split_once("\n\n").unwrap();

        let mut dots: Grid<Point> = parse_input_coordinates(coordinates);

        for fold_line in folds
            .lines()
            .map(|line| {
                let (left, right) = line.split_once('=').unwrap();
                (right, left.chars().last().unwrap())
            })
            .filter(|(_line, axis)| *axis == 'y')
            .map(|(line, _)| line.parse::<u8>().unwrap())
        {
            for (_position, point) in dots.enumerate_mut() {
                *point = convert_point_horizontal(fold_line as i8, *point);
            }
        }

        Ok(dots.values().unique().count())
    }
}

fn parse_input_coordinates(s: &str) -> Grid<Point> {
    let mut result: Grid<Point> = Grid::new(1, s.lines().count());

    for (idx, line) in s.lines().enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        *result.get_mut_with_idx(idx).unwrap() =
            (x.parse::<i8>().unwrap(), y.parse::<i8>().unwrap());
    }

    result
}

fn convert_point_horizontal(symmetry_line: i8, point: Point) -> Point {
    let (x, y) = point;

    let distance = (symmetry_line - y).abs();

    (x, symmetry_line - distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day13b::solve_default_file().unwrap(), 17)
    }
}
