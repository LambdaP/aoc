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

        Ok(res as u16)
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

        Ok(res as u16)
    }
}

// If the list is *safe*, returns None
// If the list is safe up to some level i, return Some(i)
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

    #[test]
    fn foo() {
        assert!(is_almost_safe(&[10, 1, 2, 3]));
        assert!(is_almost_safe(&[1, 4, 10, 5]));
        assert!(is_almost_safe(&[1, 10, 2, 3]));
        assert!(is_almost_safe(&[1, 10, 11, 12]));
        assert!(is_almost_safe(&[9, 1, 10, 11, 12]));
        assert!(is_almost_safe(&[0, 1, 10, 2, 3]));

        assert!(is_almost_safe(&[1, 5, 4, 3]));
        assert!(is_almost_safe(&[1, 4, 3, 2]));
        assert!(is_almost_safe(&[3, 10, 2, 1]));
        assert!(is_almost_safe(&[1, 12, 11, 10]));
        assert!(is_almost_safe(&[8, 5, 10, 4]));
    }

    #[test]
    fn bar() {
        use rand::prelude::*;

        for _ in 0..1000 {
            let asc: bool = random();

            let mut vec: Vec<u8> = vec![128];

            let len = 4 + random::<usize>() % 4;

            for i in 1..len {
                let inc = 1 + random::<u8>() % 3;
                if asc {
                    vec.push(vec[i-1] + inc);
                } else {
                    vec.push(vec[i-1] - inc);
                }
            }

            let insert_ix = random::<usize>() % len;

            let mut random_value = random::<u8>() % 64;

            if random::<bool>() {
                random_value = u8::MAX - random_value;
            }

            vec.insert(insert_ix, random_value);

            assert!(is_almost_safe(&vec), "failed on input: {vec:?}");
        }
    }
}
