use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashSet as HashSet;
use std::str::FromStr;

pub struct Day13b;

impl DaySolver for Day13b {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day13", "data/day13.txt");

    fn solution(s: &str) -> anyhow::Result<Self::Output> {
        let origami_setup: OrigamiSetup = s.parse()?;

        Ok(origami_setup.fold_once_and_count())
    }
}

#[derive(Debug)]
enum FoldDirection {
    Horizontally,
    Vertically,
}

#[derive(Debug)]
struct FoldSpec {
    direction: FoldDirection,
    value:     usize,
}

type Point = (usize, usize);

struct OrigamiSetup {
    points:        HashSet<Point>,
    folding_specs: Vec<FoldSpec>,
}

impl OrigamiSetup {
    fn fold_once_and_count(self) -> usize {
        let OrigamiSetup {
            points,
            folding_specs,
        } = self;
        let folding_spec = &folding_specs[0];

        points
            .into_iter()
            .map(|(mut x, mut y)| {
                let updated = match folding_spec.direction {
                    FoldDirection::Horizontally => &mut y,
                    FoldDirection::Vertically => &mut x,
                };

                if *updated > folding_spec.value {
                    *updated = 2 * folding_spec.value - *updated;
                }

                (x, y)
            })
            .collect::<HashSet<Point>>()
            .len()
    }
}

impl FromStr for OrigamiSetup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_coordinates(s: &str) -> HashSet<Point> {
            let mut points: HashSet<Point> = HashSet::default();

            for line in s.lines() {
                let (x, y) = line
                    .split_once(',')
                    .map(|(x_str, y_str)| {
                        (
                            x_str.parse::<usize>().ok().unwrap(),
                            y_str.parse::<usize>().ok().unwrap(),
                        )
                    })
                    .unwrap();

                points.insert((x, y));
            }

            points
        }

        fn parse_folds(s: &str) -> Vec<FoldSpec> {
            s.lines()
                .map(|line| {
                    let (left, right) = line.split_once('=').unwrap();
                    let direction = match left.chars().last().unwrap() {
                        'x' => FoldDirection::Vertically,
                        'y' => FoldDirection::Horizontally,
                        _ => unreachable!(),
                    };
                    FoldSpec {
                        direction,
                        value: right.parse().unwrap(),
                    }
                })
                .collect()
        }

        let (coordinates, folds) = s.split_once("\n\n").unwrap();

        let points = parse_coordinates(coordinates);
        let folding_specs = parse_folds(folds);

        Ok(Self {
            points,
            folding_specs,
        })
    }
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
