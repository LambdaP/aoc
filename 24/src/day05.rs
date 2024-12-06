use crate::{Aoc, Day05, InputRep, Result};

impl Aoc for Day05 {
    type Output = u32;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let PuzzleData { precedes, updates } = input.lines().try_into()?;

        let res = updates
            .into_iter()
            .filter(|u| check_update(&precedes, u))
            .map(|u| u32::from(u[u.len() / 2]))
            .sum();

        Ok(res)
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let PuzzleData { precedes, updates } = input.lines().try_into()?;

        let res = updates
            .into_iter()
            .filter(|u| !check_update(&precedes, u))
            .map::<u32, _>(|u| {
                let mask = u.iter().fold(0, |acc, x| acc | (1 << x));

                u.iter()
                    .copied()
                    .find(|&x| {
                        (precedes[usize::from(x)] & mask).count_ones() as usize == u.len() / 2
                    })
                    .unwrap()
                    .into()
            })
            .sum();

        Ok(res)
    }
}

#[inline]
fn check_update(precedes: &[u128; 100], update: &[u8]) -> bool {
    update.iter().enumerate().all(|(i, &x)| {
        update[i + 1..]
            .iter()
            .all(|y| precedes[usize::from(x)] & (1 << y) == 0)
    })
}

#[derive(Debug, Eq, PartialEq)]
struct PuzzleData {
    precedes: [u128; 100],
    updates: Vec<Vec<u8>>,
}

impl TryFrom<&[&str]> for PuzzleData {
    type Error = std::num::ParseIntError;

    // Calling this is potentially unsafe on non-ASCII input.
    fn try_from(lines: &[&str]) -> Result<Self, Self::Error> {
        let mut lines = lines.iter();

        let mut precedes = [0u128; 100];

        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let l: u8 = line[..2].parse()?;
            let r: usize = line[3..].parse()?;

            precedes[r] |= 1 << l;
        }

        let updates = lines
            .map(|line| {
                line.as_bytes()
                    .chunks(3)
                    // SAFETY: Input is ASCII, so any 2-byte slice is valid UTF-8
                    .map(|c| unsafe { std::str::from_utf8_unchecked(&c[..2]) }.parse::<u8>())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { precedes, updates })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    test_part1!(Day05, EXAMPLE_INPUT, 143);
    test_part2!(Day05, EXAMPLE_INPUT, 123);
}
