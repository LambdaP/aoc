use eyre::OptionExt;

use crate::{Aoc, Day07, InputRep, Result};

impl Aoc for Day07 {
    type Output = u64;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(13);
        let mut res = 0;

        for line in input.lines() {
            let (target, nums) = parse_line(&mut buf, line)?;
            if check(target, nums, false) {
                res += target;
            }
        }

        Ok(res)
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(13);
        let mut res = 0;

        for line in input.lines() {
            let (target, nums) = parse_line(&mut buf, line)?;
            if check(target, nums, true) {
                res += target;
            }
        }

        Ok(res)
    }
}

fn check(target: u64, nums: &[u16], concat: bool) -> bool {
    if nums.is_empty() {
        return false;
    } else if nums.len() == 1 {
        return u64::from(nums[0]) == target;
    }

    let mut stack = Vec::with_capacity(nums.len());
    stack.push((target, nums.len() as u8 - 1));

    while let Some((t, ix)) = stack.pop() {
        let x = u64::from(nums[usize::from(ix)]);

        if ix == 0 {
            if t == x {
                return true;
            }

            continue;
        }

        if t < x {
            continue;
        }

        stack.push((t - x, ix - 1));

        if t % x == 0 {
            stack.push((t / x, ix - 1));
        }

        if !concat {
            continue;
        }

        let mut h = 10;

        while h <= x {
            h *= 10;
        }

        if t % h == x {
            stack.push((t / h, ix - 1));
        }
    }

    false
}

fn parse_line<'a>(buf: &'a mut Vec<u16>, line: &str) -> Result<(u64, &'a [u16])> {
    buf.clear();
    let (l, r) = line.split_once(':').ok_or_eyre("parse error: {line:?}")?;
    let target = l.parse::<u64>()?;
    buf.extend(
        r.split_ascii_whitespace()
            .map(|s| s.parse::<u16>().unwrap()),
    );

    Ok((target, &buf[..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    test_part1!(Day07, EXAMPLE_INPUT, 3749);
    test_part2!(Day07, EXAMPLE_INPUT, 11387);
}
