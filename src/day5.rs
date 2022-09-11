use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::cmp;
use text_io::try_scan;

pub struct Day5;

type VentsLocation = HashMap<Point, u32>; // key = cartesian location, value = number of location occurrence in lines
type Point = (u32, u32);

impl DaySolver for Day5 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day5", "data/day5.txt");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let located_vents = s
            .lines()
            .flat_map(|line| {
                points_from_line(line).unwrap_or_else(|_| {
                    panic!(
                        "Unable to convert line: {}. Required format is x1,y1 -> x2,y2",
                        line
                    )
                })
            })
            .flatten()
            .fold(
                HashMap::default(),
                |mut marked_vents: VentsLocation, vent_location| {
                    *marked_vents.entry(vent_location).or_insert(0) += 1;

                    marked_vents
                },
            );

        let number_of_points_with_may_lines = located_vents
            .values()
            .into_iter()
            .filter(|&num| *num >= 2)
            .count();

        Ok(number_of_points_with_may_lines)
    }
}

// return vector of points that occurs in given line
fn points_from_line(line: &str) -> anyhow::Result<Option<Vec<Point>>> {
    let x1: u32;
    let y1: u32;
    let x2: u32;
    let y2: u32;

    try_scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);

    if x1 == x2 {
        // points that occurs vertically
        let start = cmp::min(y1, y2);
        let end = cmp::max(y1, y2);

        Ok(Some((start..=end).map(|y_value| (x1, y_value)).collect()))
    } else if y1 == y2 {
        // points that occurs horizontally
        let start = cmp::min(x1, x2);
        let end = cmp::max(x1, x2);

        Ok(Some((start..=end).map(|x_value| (x_value, y1)).collect()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day5::solve_default_file().unwrap(), 5)
    }

    #[test]
    fn day5_points_from_line_ok() {
        assert_eq!(
            points_from_line("2,2 -> 2,1").unwrap(),
            Some(vec![(2, 1), (2, 2)])
        )
    }

    #[test]
    fn day5_points_from_line_err() {
        assert!(points_from_line("2,  -> 2,1").is_err())
    }
}
