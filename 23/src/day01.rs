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
        const DIGITS: [&[u8]; 9] = [
            b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
        ];

        let res = input
            .lines()
            .iter()
            .map(|line| {
                let line = line.as_bytes();

                let first = (0..line.len())
                    .map(|i| &line[i..])
                    .find_map(|s| {
                        s.first()
                            .and_then(|b| b.is_ascii_digit().then_some(b - b'0'))
                            .or_else(|| {
                                (1u8..=9u8).find(|&d| s.starts_with(DIGITS[usize::from(d - 1)]))
                            })
                    })
                    .unwrap();

                let last = (0..line.len())
                    .rev()
                    .map(|i| &line[..=i])
                    .find_map(|s| {
                        s.last()
                            .and_then(|b| b.is_ascii_digit().then_some(b - b'0'))
                            .or_else(|| {
                                (1u8..=9u8).find(|&d| s.ends_with(DIGITS[usize::from(d - 1)]))
                            })
                    })
                    .unwrap();

                u32::from(10 * first + last)
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
