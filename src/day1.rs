use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day1;

impl DaySolver for Day1 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day1", "data/day1.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (_, counter) = _s.lines().map(|line| line.parse::<u16>().unwrap()).fold(
            (u16::MAX, 0), // to don't care about first input value 
            |(previous_measurement, counter), measurement| {
                if measurement > previous_measurement {
                    return (measurement, counter + 1);
                }

                (measurement, counter)
            },
        );

        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day1::solve_default_file().unwrap(), 7)
    }
}
