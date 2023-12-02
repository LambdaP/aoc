use crate::{Result, Day17, FileRep};
use std::fmt::Display;

const SHAPE_MINUS: [[u8;4];1] = [*b"####"];
const SHAPE_PLUS: [[u8;3];3] = [*b".#.", *b"###", *b".#."];
const SHAPE_CORN: [[u8;3];3] = [*b"..#", *b"..#", *b"###"];
const SHAPE_PIPE: [[u8;1];4] = [*b"#",*b"#",*b"#",*b"#"];
const SHAPE_SQUARE: [[u8;2];2] = [*b"##", *b"##"];

impl crate::Aoc for Day17 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        result!("todo")
    }
    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        result!("todo")
    }
}
