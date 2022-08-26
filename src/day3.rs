use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day3;

impl DaySolver for Day3 {
    type Output = u8;

    const INFO: DayInfo = DayInfo::with_day_and_file("day3", "data/day3.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let bit_occurs = _s
            .lines()
            .fold(vec![vec![0; 2]; 5], |mut bit_occurs, line| {
                for (index, value) in line.chars().enumerate() {
                    bit_occurs[index][value as usize - '0' as usize] += 1; // because ascii value is returned
                }

                bit_occurs
            });

        let most_common = bit_occurs
            .iter()
            .enumerate()
            .fold(0, |res, (index, values)| {
                if values[1] > values[0] { // values[1] number of 1 occurs, values[0] number of 0 occurs
                    return res | 1 << (4 - index);
                }

                res
            });

        Ok(most_common * ((!most_common) - 224)) // 224 <=> 11100000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day3::solve_default_file().unwrap(), 198)
    }
}
