use color_eyre::eyre::*;
use std::fmt::Display;
// use std::time::Duration;

macro_rules! result {
    ($res: expr) => {
        Ok(Box::new($res))
    };
}

#[cfg(test)]
macro_rules! test_part1 {
    ($day:expr, $input:expr, $expected:expr) => {
        #[test]
        fn test_part1() {
            let input = InputRep::new($input);
            let result = $day.part1(&input).unwrap();
            assert_eq!(result.to_string(), $expected.to_string());
        }
    };
}

#[cfg(test)]
macro_rules! test_part2 {
    ($day:expr, $input:expr, $expected:expr) => {
        #[test]
        fn test_part2() {
            let input = InputRep::new($input);
            let result = $day.part2(&input).unwrap();
            assert_eq!(result.to_string(), $expected.to_string());
        }
    };
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day10;
mod day11;

pub struct Day01;
pub struct Day02;
pub struct Day03;
pub struct Day04;
pub struct Day05;
pub struct Day06;
pub struct Day10;
pub struct Day11;

pub struct InputRep<'a> {
    raw: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> InputRep<'a> {
    pub fn new(raw: &'a str) -> Self {
        let lines = raw.lines().collect();
        Self { raw, lines }
    }

    pub fn lines(&self) -> &[&str] {
        &self.lines
    }

    pub fn as_str(&self) -> &str {
        self.raw
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_bytes()
    }
}

pub trait Aoc {
    fn part1(&self, _input: &InputRep) -> Result<Box<dyn Display>> {
        result!("todo!")
    }

    fn part2(&self, _input: &InputRep) -> Result<Box<dyn Display>> {
        result!("todo!")
    }

    fn run<P: AsRef<std::path::Path> + Copy>(&self, fpath: P) -> Result<()>
    {
        let string = std::fs::read_to_string(fpath)?;
        let input = InputRep::new(&string);

        let t0 = std::time::SystemTime::now();
        let res1 = self.part1(&input);
        let t1 = t0.elapsed();
        let t0 = std::time::SystemTime::now();
        let res2 = self.part2(&input);
        let t2 = t0.elapsed();

        println!("part 1: {} ({:?})", res1?, t1?);
        println!("part 2: {} ({:?})", res2?, t2?);

        Ok(())
    }
}
