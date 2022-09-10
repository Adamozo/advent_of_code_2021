use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day7;

impl DaySolver for Day7 {
    type Output = f32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day7", "data/day7.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut positions: Vec<u32> = s
            .split(',')
            .map(|position| position.parse::<u32>().unwrap())
            .collect();

        let median = count_median(&mut positions);

        let result = positions
            .iter()
            .map(|position| {
                let res = *position as f32 - median;
                res.abs()
            })
            .sum();

        Ok(result)
    }
}

fn count_median(positions: &mut Vec<u32>) -> f32 {
    positions.sort_unstable();

    if positions.len() % 2 == 0 {
        let mid_right = positions.len() / 2;
        let mid_left = mid_right - 1;

        (positions[mid_left] + positions[mid_right]) as f32 / 2.0
    } else {
        positions[positions.len() / 2] as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day7::solve_default_file().unwrap(), 37.0)
    }

    #[test]
    fn day7_count_median() {
        assert_eq!(count_median(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 2.0)
    }
}
