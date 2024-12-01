use crate::{Aoc, Day01, InputRep, Result};

impl Aoc for Day01 {
    type Output = u32;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let (mut left, mut right): (Vec<_>, Vec<_>) =
            input.lines().iter().map(|&line| parse_line(line)).unzip();

        left.sort_unstable();
        right.sort_unstable();

        let res = std::iter::zip(left, right)
            .map(|(x, y)| u32::abs_diff(x, y))
            .sum();

        Ok(res)
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        use std::collections::HashMap;

        let mut left = Vec::with_capacity(input.lines().len());
        let mut hist = HashMap::with_capacity(input.lines().len());

        for line in input.lines() {
            let (l, r) = parse_line(line);
            left.push(l);
            hist.entry(r).and_modify(|cnt| *cnt += r).or_insert(r);
        }

        let res = left.into_iter().filter_map(|k| hist.get(&k)).sum();

        Ok(res)
    }
}

#[inline]
fn parse_line(line: &str) -> (u32, u32) {
    let (s1, s2) = line.split_once("   ").unwrap();
    let n1 = s1.parse::<u32>().unwrap();
    let n2 = s2.parse::<u32>().unwrap();

    (n1, n2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    test_part1!(Day01, EXAMPLE_INPUT, 11);
    test_part2!(Day01, EXAMPLE_INPUT, 31);
}
