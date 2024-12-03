use crate::{Aoc, Day03, InputRep, Result};

impl Aoc for Day03 {
    type Output = u64;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        Ok(sum_all_mul(input.as_str()))
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let mut on = true;
        let mut res = 0;

        for s in input.as_str().split("do") {
            if s.starts_with("n't()") {
                on = false;
                continue;
            }

            if let Some(s) = s.strip_prefix("()") {
                on = true;
                res += sum_all_mul(s);
            } else if on {
                res += sum_all_mul(s);
            }
        }

        Ok(res)
    }
}

fn sum_all_mul(s: &str) -> u64 {
    s.split("mul")
        .filter_map(|s| parse_u16_pair(s).map(|(x, y)| u64::from(x) * u64::from(y)))
        .sum()
}

fn parse_u16_pair(s: &str) -> Option<(u16, u16)> {
    if !s.starts_with('(') {
        return None;
    }

    let mut i = 1;

    let (len_left, left) = parse_u16_delimiter::<3>(&s[usize::from(i)..], ',')?;

    i += len_left + 1;

    let (_, right) = parse_u16_delimiter::<3>(&s[usize::from(i)..], ')')?;

    Some((left, right))
}

fn parse_u16_delimiter<const N: u8>(s: &str, delimiter: char) -> Option<(u8, u16)> {
    let mut chars = (0..=N).zip(s.chars());

    let mut val = chars.next()?.1.to_digit(10)?;

    for (i, c) in chars {
        if c == delimiter {
            return Some((i, val as u16));
        }
        val = 10 * val + c.to_digit(10)?;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT_PART_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT_PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    test_part1!(Day03, EXAMPLE_INPUT_PART_1, 161);
    test_part2!(Day03, EXAMPLE_INPUT_PART_2, 48);
}
