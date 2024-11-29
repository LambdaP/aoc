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
        const DIGITS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut res: u32 = 0;
        for line in input.lines() {
            let d1 = {
                let mut val = 0;
                let mut first_ix = usize::MAX;
                if let Some(ix) = line.find(|c: char| c.is_ascii_digit()) {
                    val = line.as_bytes()[ix] - b'0';
                    first_ix = ix;
                }

                let ixs: Vec<_> = DIGITS
                    .iter()
                    .map(|pat| line.find(pat).unwrap_or(usize::MAX))
                    .collect();

                for i in 0..9 {
                    if ixs[i as usize] < first_ix {
                        first_ix = ixs[i as usize];
                        val = i + 1;
                    }
                }
                val
            };

            let d2 = {
                let mut last_ix = usize::MIN;
                let mut val = 0;
                if let Some(ix) = line.rfind(|c: char| c.is_ascii_digit()) {
                    val = line.as_bytes()[ix] - b'0';
                    last_ix = ix;
                }

                let ixs: Vec<_> = DIGITS
                    .iter()
                    .map(|pat| line.rfind(pat).unwrap_or(usize::MIN))
                    .collect();

                for i in 0..9 {
                    if ixs[i as usize] > last_ix {
                        last_ix = ixs[i as usize];
                        val = i + 1;
                    }
                }

                val
            };

            res += 10 * (d1 as u32) + (d2 as u32);
        }

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
