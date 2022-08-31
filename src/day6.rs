use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day6;

impl DaySolver for Day6 {
    type Output = u16;

    const INFO: DayInfo = DayInfo::with_day_and_file("day6", "data/day6.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        const NUMBER_OF_DAYS: usize = 80;
        const NUMBER_OF_STAGES: usize = 9;
        const DAYS_TO_DUPLICATE: usize = 7;

        let fishes_days_to_duplicate = s.split(',').map(|day| day.parse::<usize>().unwrap()).fold(
            vec![0; 9],
            |mut fishes_days_to_duplicate, days_to_duplicate| {
                fishes_days_to_duplicate[days_to_duplicate] += 1;
                fishes_days_to_duplicate
            },
        );

        let res: u16 = (1..NUMBER_OF_DAYS)
            .fold(
                fishes_days_to_duplicate,
                |mut fishes_days_to_duplicate, day| {
                    fishes_days_to_duplicate[(day + DAYS_TO_DUPLICATE) % NUMBER_OF_STAGES] +=
                        fishes_days_to_duplicate[day % NUMBER_OF_STAGES];
                    fishes_days_to_duplicate
                },
            )
            .iter()
            .sum();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day6::solve_default_file().unwrap(), 5934)
    }
}
