use crate::{Aoc, Day02, InputRep, Result};

impl Aoc for Day02 {
    type Output = u16;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let res = input
            .lines()
            .iter()
            .map(|&line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|levels| unsafe_index(levels).is_none())
            .count();

        u16::try_from(res).map_err(|e| e.into())
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let res = input
            .lines()
            .iter()
            .map(|&line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|levels| is_almost_safe(levels))
            .count();

        u16::try_from(res).map_err(|e| e.into())
    }
}

// If the list is *safe*, returns None
// If the list is safe up to some level i, return Some(i+1)
fn unsafe_index(levels: &[u8]) -> Option<usize> {
    let asc = {
        let [a, b, ..] = levels[..] else {
            return None;
        };
        if a == b {
            return Some(1);
        }
        a < b
    };

    if asc {
        levels
            .iter()
            .zip(&levels[1..])
            .position(|(a, b)| b <= a || b - a > 3)
    } else {
        levels
            .iter()
            .zip(&levels[1..])
            .position(|(a, b)| a <= b || a - b > 3)
    }
    .map(|x| x + 1)
}

fn is_almost_safe(levels: &[u8]) -> bool {
    let Some(ui) = unsafe_index(levels) else {
        return true;
    };

    if unsafe_index(&levels[1..]).is_none() {
        return true;
    }

    let mut l = Vec::from(levels);

    l.remove(ui);

    if unsafe_index(&l).is_none() {
        return true;
    }

    let mut l = Vec::from(levels);

    l.remove(ui - 1);

    unsafe_index(&l).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    test_part1!(Day02, EXAMPLE_INPUT, 2);
    test_part2!(Day02, EXAMPLE_INPUT, 4);
}
