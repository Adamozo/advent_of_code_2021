use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

pub struct Day14;

impl DaySolver for Day14 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day14", "data/day14.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let runner = s.parse::<Runner>()?;

        Ok(runner.make_steps(10))
    }
}

type Pair = [char; 2];

#[derive(Debug)]
struct Runner {
    polymer_template: Vec<char>,
    rules:            HashMap<Pair, char>,
}

impl Runner {
    fn make_steps(&self, number_of_steps: usize) -> usize {
        let mut letter_occurrence =
            self.polymer_template
                .iter()
                .fold(HashMap::default(), |mut result, letter| {
                    *result.entry(letter).or_default() += 1;
                    result
                });

        let mut pair_occurrence = self.polymer_template.windows(2).map(|w| [w[0], w[1]]).fold(
            HashMap::default(),
            |mut counters, pair| {
                *counters.entry(pair).or_insert(0) += 1;
                counters
            },
        );

        for _ in 1..=number_of_steps {
            let mut new_pair_occurrence: HashMap<Pair, usize> = HashMap::default();

            for (pair @ [left, right], count) in pair_occurrence {
                let middle = self.rules.get(&pair).unwrap();

                *letter_occurrence.entry(middle).or_default() += count;

                *new_pair_occurrence.entry([left, *middle]).or_default() += count;
                *new_pair_occurrence.entry([*middle, right]).or_default() += count;
            }

            pair_occurrence = new_pair_occurrence;
        }

        // ---------------------------------------------------------------------

        let mut v: Vec<(&char, usize)> = letter_occurrence.into_iter().collect();
        v.sort_by(|x, y| x.1.cmp(&y.1));

        let (_, max) = v.last().unwrap();
        let (_, min) = v.first().unwrap();

        max - min
    }
}

impl FromStr for Runner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_line, rules) = s.split_once("\n\n").unwrap();
        let polymer_template = first_line.chars().collect();

        let rules = rules.lines().fold(HashMap::default(), |mut rules, line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut left = left.chars();
            let key: Pair = [left.next().unwrap(), left.next().unwrap()];
            let value = right.chars().next().unwrap();

            rules.insert(key, value);
            rules
        });

        Ok(Self {
            polymer_template,
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