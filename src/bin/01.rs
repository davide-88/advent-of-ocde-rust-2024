use std::collections::HashMap;

advent_of_code::solution!(1);

struct TwoCloumns {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl TwoCloumns {
    fn new() -> Self {
        TwoCloumns {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn parse(input: &str) -> Self {
        input
            .lines()
            .map(|line| {
                let entries: Vec<&str> = line.split_whitespace().collect();
                if entries.len() != 2 {
                    panic!("{}", format!("Invalid input: {}", line));
                }
                (
                    entries[0]
                        .parse::<u64>()
                        .map_err(|_| format!("{} is not a positive integer", &entries[0]))
                        .unwrap(),
                    entries[1]
                        .parse::<u64>()
                        .map_err(|_| format!("{} is not a positive integer", &entries[1]))
                        .unwrap(),
                )
            })
            .fold(TwoCloumns::new(), |mut acc, curr| {
                acc.add(curr.0, curr.1);
                acc
            })
    }

    fn add(&mut self, left: u64, right: u64) {
        self.left.push(left);
        self.right.push(right);
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }

    fn check_vec_lenth(&self) {
        if self.left.len() != self.right.len() {
            panic!(
                "{}",
                format!(
                    "Columns have different lengths ({}!={})",
                    self.left.len(),
                    self.right.len()
                )
            );
        }
    }

    fn total_distance(&self) -> Option<u64> {
        self.check_vec_lenth();
        if self.left.len() == 0 {
            return None;
        }
        Some(
            self.left
                .iter()
                .zip(self.right.iter())
                .map(|(left, right)| left.abs_diff(*right))
                .sum(),
        )
    }

    fn similaryty_score(&self) -> Option<u64> {
        self.check_vec_lenth();
        if self.left.len() == 0 {
            return None;
        }
        let counters_by_id =
            self.right
                .iter()
                .fold(HashMap::with_capacity(self.right.len()), |mut acc, curr| {
                    acc.insert(curr, acc.get(curr).unwrap_or(&0) + 1);
                    acc
                });

        Some(
            self.left
                .iter()
                .map(|left| *left * *counters_by_id.get(left).unwrap_or(&0))
                .sum(),
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut two_columns_vec = TwoCloumns::parse(input);

    two_columns_vec.sort();
    two_columns_vec.total_distance()
}

pub fn part_two(input: &str) -> Option<u64> {
    TwoCloumns::parse(input).similaryty_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
