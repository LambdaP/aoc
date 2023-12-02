use crate::{Aoc, Day02, Display, FileRep, Result};

use std::cmp::{max, Ordering, PartialOrd};

#[derive(Debug, Clone, Copy, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl PartialOrd for RGB {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            return Some(Ordering::Less);
        } else if self.r >= other.r && self.g >= other.g && self.b >= other.b {
            return Some(Ordering::Greater);
        }

        return None
    }
}

fn max_rgb(a: RGB, b: RGB) -> RGB {
    let r = max(a.r, b.r);
    let g = max(a.g, b.g);
    let b = max(a.b, b.b);

    RGB { r, g, b }
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    cubes: Vec<RGB>,
}

impl Aoc for Day02 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let test_bag = RGB { r: 12, g: 13, b: 14 };
        let lines = &input.string_lines;

        let mut res: usize = 0;
        for line in lines {
            let Some(g) = parse_line(line) else {
                continue;
            };
            if g.cubes.iter().all(|&x| x <= test_bag) {
                res += g.id;
            }
        }

        result!(res)
    }
    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.string_lines;

        let mut res: u32 = 0;
        for line in lines {
            let Some(g) = parse_line(line) else {
                continue;
            };

            let rgb = g.cubes.iter().fold(RGB { r: 0, g: 0, b: 0 }, |acc, &x| max_rgb(acc, x));
            let power = (rgb.r as u32) * (rgb.g as u32) * (rgb.b as u32);

            res += power;
        }
        result!(res)
    }
}

fn parse_line(line: &str) -> Option<Game> {
    let (left, right) = line.split_once(":")?;
    let id = left.strip_prefix("Game ")?.parse::<usize>().ok()?;

    let mut cubes: Vec<RGB> = vec![];
    for revealed in right.split(";") {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for c in revealed.split(",") {
            let c = c.trim_start();
            let (count, color) = c.split_once(" ")?;

            let count = count.parse::<u8>().ok()?;

            match color {
                "red" => { r = count; }
                "green" => { g = count; }
                "blue" => { b = count; }
                _ => { return None }
            }
        }

        cubes.push(RGB{r, g, b}); 

    }

    Some(Game { id, cubes })
}
