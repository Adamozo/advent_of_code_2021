use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::str::FromStr;

pub struct Day9;

impl DaySolver for Day9 {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day9", "data/day9.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let heightmap = s.parse::<Heightmap>().unwrap();
        let res = heightmap.count_risk();

        Ok(res)
    }
}

const NEIGHBORS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct Heightmap {
    data: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}

impl FromStr for Heightmap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.bytes().map(|num| num as u32 - 48).collect())
            .collect();

        let rows = data.len();
        let columns = data[0].len();

        Ok(Self {
            data,
            rows,
            columns,
        })
    }
}

impl Heightmap {
    fn has_lower_neighbors(&self, current_row: usize, current_column: usize) -> bool {
        let current_value = self.data[current_row][current_column];
        NEIGHBORS
            .iter()
            .map(|(row, column)| (*row + current_row as i8, *column + current_column as i8))
            .filter(|(row, column)| {
                (0..self.rows as i8).contains(row) && (0..self.columns as i8).contains(column)
            })
            .any(|(row, column)| self.data[row as usize][column as usize] < current_value)
    }

    fn count_risk(&self) -> u32 {
        let mut risk = 0;

        for row in 0..self.rows {
            for column in (0..self.columns).filter(|&column| !self.has_lower_neighbors(row, column))
            {
                risk += self.data[row][column] + 1;
            }
        }

        risk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day9::solve_default_file().unwrap(), 15)
    }

    #[test]
    fn day9_heightmap_count_risk() {
        let data = vec![vec![0, 1], vec![1, 2]];
        let rows = 2;
        let columns = 2;

        let heightmap = Heightmap {
            data,
            rows,
            columns,
        };
        assert_eq!(heightmap.count_risk(), 1)
    }

    #[test]
    fn day9_has_lower_neighbors() {
        let data = vec![vec![0, 1], vec![1, 2]];
        let rows = 2;
        let columns = 2;

        let heightmap = Heightmap {
            data,
            rows,
            columns,
        };

        assert_eq!(heightmap.has_lower_neighbors(0, 0), false);
        assert_eq!(heightmap.has_lower_neighbors(1, 1), true);
    }
}
