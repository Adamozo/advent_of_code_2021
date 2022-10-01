use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

pub struct Day14;

impl DaySolver for Day14 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day14", "data/day14.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut runner = s.parse::<Runner>()?;
        runner.make_steps(10);
        Ok(runner.get_result())
    }
}

#[derive(Debug)]
struct Runner {
    letter_occurrence: HashMap<char, usize>,
    pair_occurrence:   HashMap<[char; 2], usize>,
    rules:             HashMap<[char; 2], char>,
}

impl Runner {
    fn make_steps(&mut self, number_of_steps: usize) {
        for _ in 1..=number_of_steps {
            let mut new_pair_occurrence: HashMap<[char; 2], usize> = HashMap::default();

            for (key, value) in self.pair_occurrence.iter() {
                let medium_letter = self.rules.get(key).unwrap();

                *self.letter_occurrence.entry(*medium_letter).or_default() += value;

                // left from checked pair, received letter
                *new_pair_occurrence
                    .entry([key[0], *medium_letter])
                    .or_default() += value;
                // received letter, right from checked pair
                *new_pair_occurrence
                    .entry([*medium_letter, key[1]])
                    .or_default() += value;
            }

            self.pair_occurrence = new_pair_occurrence;
        }
    }

    fn get_result(self) -> usize {
        let mut v: Vec<(char, usize)> = self.letter_occurrence.into_iter().collect();
        v.sort_by(|x, y| x.1.cmp(&y.1));

        let (_, max) = v.last().unwrap();
        let (_, min) = v.first().unwrap();

        max - min
    }
}

impl FromStr for Runner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, rules) = s.split_once("\n\n").unwrap();

        let rules = {
            let mut res: HashMap<[char; 2], char> = HashMap::default();

            for (left, right) in rules.lines().map(|line| line.split_once(" -> ").unwrap()) {
                let mut left = left.chars();
                let key: [char; 2] = [left.next().unwrap(), left.next().unwrap()];

                let _unused = res.insert(key, right.chars().next().unwrap());
            }
            res
        };

        let letter_occurrence = {
            let mut result: HashMap<char, usize> = HashMap::default();

            for letter in start.chars() {
                *result.entry(letter).or_default() += 1;
            }

            result
        };

        let pair_occurrence = {
            let mut chars = start.chars();
            let first_char = chars.next().unwrap();

            let (_, res) = chars.fold(
                (first_char, HashMap::default()),
                |(previous_char, mut pair_occurrence), current_char| {
                    *pair_occurrence
                        .entry([previous_char, current_char])
                        .or_default() += 1;
                    (current_char, pair_occurrence)
                },
            );

            res
        };

        Ok(Self {
            letter_occurrence,
            pair_occurrence,
            rules,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day14::solve_default_file().unwrap(), 1588)
    }
}
