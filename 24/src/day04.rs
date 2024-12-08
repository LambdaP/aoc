use eyre::WrapErr;
use std::array;

use crate::{Aoc, Day04, InputRep, Result};

impl Aoc for Day04 {
    type Output = u32;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        fn is_xmas_word(s: &[u8]) -> bool {
            s == b"XMAS" || s == b"SAMX"
        }

        let byte_lines = input.byte_lines();

        let res = x_windows::<4>(byte_lines)
            .flat_map(<[_; 2]>::from)
            .chain(horizontal_windows::<4>(byte_lines))
            .chain(vertical_windows::<4>(byte_lines))
            .filter(|s| is_xmas_word(s))
            .count();

        <u32>::try_from(res).wrap_err("")
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        fn is_mas_word(s: &[u8]) -> bool {
            s == b"MAS" || s == b"SAM"
        }

        let byte_lines = input.byte_lines();

        let res = x_windows::<3>(byte_lines)
            .filter(|(l, r)| is_mas_word(l) && is_mas_word(r))
            .count();

        <u32>::try_from(res).wrap_err("")
    }
}

fn x_windows<'a, const N: usize>(
    data: &'a [&'a [u8]],
) -> impl Iterator<Item = ([u8; N], [u8; N])> + 'a {
    (0..=data.len() - N).flat_map(move |row| {
        (0..=data[row].len() - N).map(move |col| {
            (
                array::from_fn(|i| data[row + i][col + i]),
                array::from_fn(|i| data[row + i][col + N - i - 1]),
            )
        })
    })
}

fn horizontal_windows<'a, const N: usize>(
    data: &'a [&'a [u8]],
) -> impl Iterator<Item = [u8; N]> + 'a {
    data.iter()
        .flat_map(|row| (0..=row.len() - N).map(move |col| row[col..col + N].try_into().unwrap()))
}

fn vertical_windows<'a, const N: usize>(
    data: &'a [&'a [u8]],
) -> impl Iterator<Item = [u8; N]> + 'a {
    (0..=data.len() - N).flat_map(move |row| {
        (0..data[row].len()).map(move |col| array::from_fn(|i| data[row + i][col]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    test_part1!(Day04, EXAMPLE_INPUT, 18);
    test_part2!(Day04, EXAMPLE_INPUT, 9);
}
