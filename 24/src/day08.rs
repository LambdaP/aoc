use crate::{Aoc, Day08, InputRep, Result};

impl Aoc for Day08 {
    type Output = u16;

    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let mut antinodes = vec![0u64; input.byte_lines().len()];

        let h = input.byte_lines().len() as i8;
        let w = input.byte_lines()[0].len() as i8;

        for a in parse_antennas(input.byte_lines()) {
            for (i, (x1, y1)) in a.iter().enumerate() {
                for (x2, y2) in &a[i + 1..] {
                    let p @ (x, y) = (2 * x1 - x2, 2 * y1 - y2);
                    if point_is_in_grid((h, w), p) {
                        antinodes[x as usize] |= 1 << y;
                    }
                    let p @ (x,y) = (2 * x2 - x1, 2 * y2 - y1);
                    if point_is_in_grid((h, w), p) {
                        antinodes[x as usize] |= 1 << y;
                    }
                }
            }
        }

        Ok(antinodes.into_iter().map(|x| x.count_ones()).sum::<u32>() as u16)
    }

    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let mut antinodes = vec![0u64; input.byte_lines().len()];

        let h = input.byte_lines().len() as i8;
        let w = input.byte_lines()[0].len() as i8;

        for a in parse_antennas(input.byte_lines()) {
            for (i, (x1, y1)) in a.iter().enumerate() {
                for (x2, y2) in &a[i + 1..] {
                    for (x,y) in semi_line((h,w), (*x2, *y2), (x2-x1, y2-y1)) {
                        antinodes[x as usize] |= 1 << y;
                    }
                    for (x,y) in semi_line((h,w), (*x1, *y1), (x1-x2, y1-y2)) {
                        antinodes[x as usize] |= 1 << y;
                    }
                }
            }
        }

        Ok(antinodes.into_iter().map(|x| x.count_ones()).sum::<u32>() as u16)
    }
}

#[inline]
fn point_is_in_grid((h, w): (i8, i8), (x, y): (i8, i8)) -> bool {
    0 <= x && x < h && 0 <= y && y < w
}

#[inline]
fn semi_line(grid_dims: (i8, i8), p0: (i8, i8), (delta_x, delta_y): (i8, i8)) -> impl Iterator<Item = (i8, i8)> {
    std::iter::successors(Some(p0), move |(x, y)| {
        let x = x + delta_x;
        let y = y + delta_y;

        if point_is_in_grid(grid_dims, (x, y)) {
            Some((x, y))
        } else {
            None
        }
    })
}

const RANGE: usize = 1 + (b'z' - b'0') as usize;

fn parse_antennas(byte_lines: &[&[u8]]) -> [Vec<(i8, i8)>; RANGE] {
    let mut antennas: [Vec<(i8, i8)>; RANGE] = vec![Vec::new(); RANGE].try_into().unwrap();

    (0i8..)
        .zip(byte_lines)
        .flat_map(|(i, l)| {
            (0i8..)
                .zip(*l)
                .filter(|(_, b)| **b != b'.')
                .map(move |(j, b)| (b, (i, j)))
        })
        .for_each(|(b, coords)| antennas[usize::from(*b - b'0')].push(coords));

    antennas
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    test_part1!(Day08, EXAMPLE_INPUT, 14);
    test_part2!(Day08, EXAMPLE_INPUT, 34);
}
