use tinyjson::format;

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

    fn add(&mut self, left: u64, right: u64) {
        self.left.push(left);
        self.right.push(right);
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }

    fn total_distance(&self) -> Option<u64> {
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
        if self.left.len() == 0 {
            return None;
        }
        Some(
            self.left
                .iter()
                .zip(self.right.iter())
                .map(|(a, b)| b.abs_diff(*a))
                .sum(),
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut two_columns_vec = input
        .lines()
        .map(|line| {
            let entries: Vec<&str> = line.split_whitespace().collect();
            if entries.len() != 2 {
                panic!("{}", format!("Invalid input: {}", line));
            }
            // parse the first entry as a u64
            // if it fails, return an error message
            //
            // unwrap the result to get the u64 value
            //
            //

            [
                entries[0]
                    .parse::<u64>()
                    .map_err(|_| format!("{} is not a positive integer", &entries[0]))
                    .unwrap(),
                entries[1]
                    .parse::<u64>()
                    .map_err(|_| format!("{} is not a positive integer", &entries[1]))
                    .unwrap(),
            ]
        })
        .fold(TwoCloumns::new(), |mut acc, curr| {
            acc.add(curr[0], curr[1]);
            acc
        });

    two_columns_vec.sort();
    two_columns_vec.total_distance()
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
