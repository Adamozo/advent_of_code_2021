use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day3;

impl DaySolver for Day3 {
    type Output = u8;

    const INFO: DayInfo = DayInfo::with_day_and_file("day3", "data/day3.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        const COUNT_OF_DIGITS: usize = 5;

        let bit_occurs =
            s.lines()
                .fold(vec![vec![0; 2]; COUNT_OF_DIGITS], |mut bit_occurs, line| {
                    for (index, value) in line.bytes().enumerate() {
                        let value_index = (value - b'0') as usize; // because ascii value is returned
                        bit_occurs[index][value_index] += 1;
                    }

                    bit_occurs
                });

        let gamma_rate = bit_occurs
            .iter()
            .enumerate()
            .fold(0, |res, (index, values)| {
                // values[0] - number of 0 occurs
                // values[1] - number of 1 occurs,
                // the max. value of the 'index' variable -> 4
                if values[1] > values[0] {
                    res | 1 << (4 - index)
                } else {
                    res
                }
            });

        let bit_mask = (1 << COUNT_OF_DIGITS) - 1;
        let epsilon_rate = !gamma_rate & bit_mask;

        Ok(gamma_rate * epsilon_rate)
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
