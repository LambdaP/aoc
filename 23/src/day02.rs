use crate::{Aoc, Day02, Display, InputRep, Result};

use std::{
    cmp::{max, Ordering, PartialOrd},
    convert::Infallible,
    str::FromStr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl PartialOrd for Rgb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            Some(Ordering::Less)
        } else if self.r >= other.r && self.g >= other.g && self.b >= other.b {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl FromStr for Rgb {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rgb: Self = Default::default();

        for c in s.split(',') {
            let c = c.trim_start();
            let (count, color) = c.split_once(' ').unwrap();
            let count = count.parse::<u8>().ok().unwrap();

            match color {
                "red" => {
                    rgb.r = count;
                }
                "green" => {
                    rgb.g = count;
                }
                "blue" => {
                    rgb.b = count;
                }
                _ => panic!(),
            }
        }

        Ok(rgb)
    }
}

fn max_rgb(a: Rgb, b: Rgb) -> Rgb {
    let r = max(a.r, b.r);
    let g = max(a.g, b.g);
    let b = max(a.b, b.b);

    Rgb { r, g, b }
}

fn power(Rgb { r, g, b }: Rgb) -> u32 {
    (r as u32) * (g as u32) * (b as u32)
}

#[derive(Debug, Default, Clone)]
struct Game {
    id: usize,
    cubes: Vec<Rgb>,
}

impl Aoc for Day02 {
    fn part1(&self, input: &InputRep) -> Result<Box<dyn Display>> {
        let test_bag = Rgb {
            r: 12,
            g: 13,
            b: 14,
        };
        let lines = &input.lines();

        let res: usize = lines
            .iter()
            .filter_map(|line| parse_line(line))
            .map(|game| (game.id, game.cubes.into_iter().reduce(max_rgb).unwrap()))
            .filter_map(|(id, rgb)| (rgb <= test_bag).then_some(id))
            .sum();

        result!(res)
    }
    fn part2(&self, input: &InputRep) -> Result<Box<dyn Display>> {
        let lines = &input.lines();

        let res: u32 = lines
            .iter()
            .filter_map(|line| parse_line(line))
            .map(|game| game.cubes.into_iter().reduce(max_rgb).unwrap())
            .map(power)
            .sum();

        result!(res)
    }
}

fn parse_line(line: &str) -> Option<Game> {
    let (left, right) = line.split_once(':')?;
    let id = left.strip_prefix("Game ")?.parse::<usize>().ok()?;

    let mut cubes: Vec<Rgb> = vec![];
    for revealed in right.split(';') {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for c in revealed.split(',') {
            let c = c.trim_start();
            let (count, color) = c.split_once(' ')?;

            let count = count.parse::<u8>().ok()?;

            match color {
                "red" => {
                    r = count;
                }
                "green" => {
                    g = count;
                }
                "blue" => {
                    b = count;
                }
                _ => return None,
            }
        }

        cubes.push(Rgb { r, g, b });
    }

    Some(Game { id, cubes })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    test_part1!(Day02, EXAMPLE_INPUT, 8_u32);
    test_part2!(Day02, EXAMPLE_INPUT, 2286_u32);
}
