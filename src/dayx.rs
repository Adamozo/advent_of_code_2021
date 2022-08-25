use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct DayX;

impl DaySolver for DayX {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("dayx", "data/dayx.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(DayX::solve_default_file().unwrap(), 0)
    }
}
