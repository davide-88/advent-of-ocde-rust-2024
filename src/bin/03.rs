advent_of_code::solution!(3);

use regex::Regex;

fn parse_operand(matched: regex::Match<'_>) -> u32 {
    let slice = matched.as_str();
    String::from(slice)
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("{}", format!("Could not parse {slice}")))
}

fn compute(captures: regex::Captures<'_>) -> u32 {
    let left = captures.name("left").map(parse_operand);
    let right = captures.name("right").map(parse_operand);
    match (left, right) {
        (Some(left), Some(right)) => left * right,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)")
            .expect("Invalid regex")
            .captures_iter(input)
            .map(|captures| compute(captures))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut skip = false;
    Some(
        Regex::new(
            r"(?<consider>do\(\))|(?<skip>don't\(\))|mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)",
        )
        .expect("Invalid regex")
        .captures_iter(input)
        .map(|matched| {
            skip = matched
                .name("consider")
                .map(|_| false)
                .or(matched.name("skip").map(|_| true))
                .unwrap_or(skip);
            if !skip {
                compute(matched)
            } else {
                0
            }
        })
        .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd
            .join("data")
            .join("examples")
            .join(format!("03_pt2.txt"));
        let f = fs::read_to_string(filepath);
        let result = part_two(&f.expect("could not open input file"));
        assert_eq!(result, Some(48));
    }
}
