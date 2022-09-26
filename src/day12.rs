use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;

pub struct Day12;

impl DaySolver for Day12 {
    type Output = u8;

    const INFO: DayInfo = DayInfo::with_day_and_file("day12", "data/day12.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let caves_connections = get_caves_connections(s);

        let mut counter = 0;
        let mut path: Vec<&str> = Vec::with_capacity(caves_connections.len());
        let mut stack: Vec<Vec<&str>> = Vec::with_capacity(caves_connections.len());

        stack.push(caves_connections.get("start").unwrap().clone());

        while !stack.is_empty() {
            if let Some(checked_cave) = stack.last_mut().unwrap().pop() {
                match CaveType::from(checked_cave) {
                    CaveType::End => {
                        counter += 1;
                    },
                    cave_type => {
                        if cave_type == CaveType::Small && !path.contains(&checked_cave)
                            || cave_type == CaveType::Big
                        {
                            stack.push(caves_connections.get(&checked_cave).unwrap().to_owned());
                            path.push(checked_cave);
                        }
                    },
                }
            } else {
                // when all caves connected to previous cave was checked
                let _unused = stack.pop();
                let _unused = path.pop();
            }
        }

        Ok(counter)
    }
}

#[derive(Debug, PartialEq)]
enum CaveType {
    Big,
    Small,
    End,
}

impl From<&str> for CaveType {
    fn from(s: &str) -> Self {
        if s.chars().next().unwrap().is_lowercase() {
            if s == "end" { Self::End } else { Self::Small }
        } else {
            Self::Big
        }
    }
}

fn get_caves_connections(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::default();

    for line in s.lines() {
        let (left, right) = line.split_once('-').unwrap();

        match left {
            "start" => {
                connections.entry("start").or_default().push(right);
            },
            "end" => {
                connections.entry(right).or_default().push("end");
            },
            left => {
                if right != "start" {
                    connections.entry(left).or_default().push(right);
                }

                if right != "end" {
                    connections.entry(right).or_default().push(left);
                }
            },
        }
    }

    connections
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day12::solve_default_file().unwrap(), 226)
    }
}
