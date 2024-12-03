use color_eyre::eyre::{Ok, Result};
use std::fmt::Display;

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

pub struct Day01;
pub struct Day02;
pub struct Day03;

pub struct InputRep<'a> {
    raw: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> InputRep<'a> {
    #[must_use]
    pub fn new(raw: &'a str) -> Self {
        let lines = raw.lines().collect();
        Self { raw, lines }
    }

    #[must_use]
    pub fn lines(&self) -> &[&str] {
        &self.lines
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.raw
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_bytes()
    }
}

pub trait Aoc {
    type Output: Display;

    fn part1(&self, _input: &InputRep) -> Result<Self::Output> {
        todo!()
    }

    fn part2(&self, _input: &InputRep) -> Result<Self::Output> {
        todo!()
    }

    fn run(&self, input: &InputRep) -> Result<()> {
        let t0 = std::time::SystemTime::now();
        let res1 = self.part1(input);
        let t1 = t0.elapsed();
        let t0 = std::time::SystemTime::now();
        let res2 = self.part2(input);
        let t2 = t0.elapsed();

        println!("part 1: {} ({:?})", res1?, t1?);
        println!("part 2: {} ({:?})", res2?, t2?);

        Ok(())
    }

    fn run_file<P>(&self, fpath: P) -> Result<()>
    where
        P: AsRef<std::path::Path> + Copy,
    {
        let string = std::fs::read_to_string(fpath)?;
        let input = InputRep::new(&string);
        self.run(&input)
    }
}
