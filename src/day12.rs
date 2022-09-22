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
                if checked_cave == "end" {
                    counter += 1;
                } else if is_small_cave(checked_cave) {
                    // small cave can occur only once in path
                    if !path.contains(&checked_cave) {
                        stack.push(caves_connections.get(&checked_cave).unwrap().clone());
                        path.push(checked_cave);
                    }
                } else {
                    // case when big cave is checked
                    stack.push(caves_connections.get(&checked_cave).unwrap().clone());
                    path.push(checked_cave);
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

fn is_small_cave(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn get_caves_connections(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::default();

    for line in s.lines() {
        let (left, right) = line.split_once('-').unwrap();

        match left {
            "start" => {
                connections.entry("start").or_insert(Vec::new()).push(right);
            },
            "end" => {
                connections.entry(right).or_insert(Vec::new()).push("end");
            },
            left => {
                if right != "start" {
                    connections.entry(left).or_insert(Vec::new()).push(right);
                }

                if right != "end" {
                    connections.entry(right).or_insert(Vec::new()).push(left);
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
