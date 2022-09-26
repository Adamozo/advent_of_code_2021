use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use aoc_utils::{Grid, PositionInGrid};
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

struct OrigamiSetup {
    grid:          Grid<char>,
    folding_specs: Vec<FoldSpec>,
}

impl OrigamiSetup {
    fn fold_once_and_count(&self) -> usize {
        let folding_spec = &self.folding_specs[0];

        let max_row = self.grid.max_row();
        let max_column = self.grid.max_column();

        self.grid
            .enumerate()
            .filter(|(pos, _)| match folding_spec.direction {
                FoldDirection::Horizontally => pos.row < folding_spec.value,
                FoldDirection::Vertically => pos.column < folding_spec.value,
            })
            .filter(|(pos, &value)| {
                value == '#'
                    || match folding_spec.direction {
                        FoldDirection::Horizontally => {
                            let row_mirror = max_row - pos.row;
                            *self.grid.get_with_tuple(&(row_mirror, pos.column)).unwrap() == '#'
                        },
                        FoldDirection::Vertically => {
                            let column_mirror = max_column - pos.column;
                            *self.grid.get_with_tuple(&(pos.row, column_mirror)).unwrap() == '#'
                        },
                    }
            })
            .count()
    }
}

impl FromStr for OrigamiSetup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_coordinates(s: &str) -> Grid<char> {
            let mut grid = Grid::new_with_init_value(15, 11, '.');

            for line in s.lines() {
                let (y, x) = line
                    .split_once(',')
                    .map(|(x_str, y_str)| {
                        (
                            y_str.parse::<usize>().ok().unwrap(),
                            x_str.parse::<usize>().ok().unwrap(),
                        )
                    })
                    .unwrap();

                let pos = PositionInGrid::new(y, x);

                if let Some(value) = grid.get_mut_with_pos(&pos) {
                    *value = '#';
                }
            }

            // println!("{}", grid);

            grid
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

        let grid = parse_coordinates(coordinates);
        let folding_specs = parse_folds(folds);

        // println!("{:#?}", folding_specs);

        Ok(Self {
            grid,
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
