use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day8;

impl DaySolver for Day8 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day8", "data/day8.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let looked_segments_len = [2, 4, 3, 7]; // segments number of numbers: [1, 4, 7, 8]

        let res = s
            .lines()
            .flat_map(|line| {
                let (_, digits) = line.split_once(" | ").unwrap();
                digits.split(' ')
            })
            .map(|digit_segments| digit_segments.len())
            .filter(|number_of_segments| looked_segments_len.contains(number_of_segments))
            .count();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day8::solve_default_file().unwrap(), 26)
    }
}
