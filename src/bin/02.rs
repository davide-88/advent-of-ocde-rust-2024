advent_of_code::solution!(2);

enum ReportSafety {
    Safe,
    Unsafe,
}

impl ReportSafety {
    fn get_value(&self) -> u32 {
        match self {
            ReportSafety::Safe => 1,
            ReportSafety::Unsafe => 0,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| {
                    level
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("{}", format!("{level} is not an integer")))
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    reports
}

fn is_safe(report: &Vec<u32>) -> bool {
    let diffs: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(level, next_level)| *level as i32 - *next_level as i32)
        .collect();
    diffs.iter().all(|diff| &1 <= diff && diff <= &3)
        || diffs.iter().all(|diff| &-3 <= diff && diff <= &-1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);

    Some(reports.iter().filter(|report| is_safe(report)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);

    Some(
        reports
            .iter()
            .filter(|report| {
                let mut slice_index = 0;
                loop {
                    if slice_index == report.len() {
                        return false;
                    }
                    let mut sub_array = Vec::with_capacity(report.len() - 1);
                    sub_array.extend_from_slice(&report[..slice_index]);
                    sub_array.extend_from_slice(&report[slice_index + 1..]);
                    if is_safe(&sub_array) {
                        return true;
                    }
                    slice_index += 1;
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
