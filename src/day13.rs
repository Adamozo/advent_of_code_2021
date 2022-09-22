use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashSet as HashSet;

pub struct Day13;

type Point = (i8, i8);

impl DaySolver for Day13 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day13", "data/day13.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (coordinates, folds) = s.split_once("\n\n").unwrap();

        let mut dots: HashSet<Point> = coordinates
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<i8>().unwrap(), y.parse::<i8>().unwrap())
            })
            .collect();

        for fold_line in folds
            .lines()
            .map(|line| {
                let (left, right) = line.split_once('=').unwrap();
                (right, left.chars().last().unwrap())
            })
            .filter(|(_line, axis)| *axis == 'y')
            .map(|(line, _)| line.parse::<u8>().unwrap())
        {
            dots = dots
                .iter()
                .fold(HashSet::default(), |mut dots: HashSet<Point>, point| {
                    let new_point = convert_point_horizontal(fold_line as i8, *point);
                    let _unused = dots.insert(new_point);
                    dots
                });
        }

        Ok(dots.len())
    }
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
        assert_eq!(Day13::solve_default_file().unwrap(), 17)
    }
}
