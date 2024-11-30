use crate::{Aoc, Day01, InputRep, Result};

impl Aoc for Day01 {
    type Output = u32;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let res = input
            .lines()
            .iter()
            .map(|line| {
                let line = line.as_bytes();
                let first = line[line.iter().position(|&b| b.is_ascii_digit()).unwrap()] - b'0';
                let last = line[line.iter().rposition(|&b| b.is_ascii_digit()).unwrap()] - b'0';

                u32::from(10 * first + last)
            })
            .sum();

        Ok(res)
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        const DIGITS: [(&str, u32); 18] = [
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let res = input
            .lines()
            .iter()
            .map(|line| {
                let first = (0..line.len())
                    .find_map(|i| {
                        DIGITS
                            .iter()
                            .find(|(s, _)| line[i..].starts_with(*s))
                            .map(|(_, v)| *v)
                    })
                    .unwrap();

                let last = (0..line.len())
                    .rev()
                    .find_map(|i| {
                        DIGITS
                            .iter()
                            .find(|(s, _)| line[..=i].ends_with(*s))
                            .map(|(_, v)| *v)
                    })
                    .unwrap();

                10 * first + last
            })
            .sum();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_INPUT_PART_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    test_part1!(Day01, EXAMPLE_INPUT_PART_1, 142);
    test_part2!(Day01, EXAMPLE_INPUT_PART_2, 281);
}
