advent_of_code::solution!(3);

use regex::Regex;

fn parse(matched: regex::Match<'_>) -> u32 {
    let slice = matched.as_str();
    String::from(slice)
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("{}", format!("Could not parse {slice}")))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)")
            .expect("Invalid regex")
            .captures_iter(input)
            .map(|captures| {
                let left = captures.name("left").map(parse);
                let right = captures.name("right").map(parse);
                match (left, right) {
                    (Some(left), Some(right)) => left * right,
                    _ => 0,
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
