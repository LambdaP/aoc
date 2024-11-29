use crate::{Aoc, Day03, Display, InputRep, Result};

impl Aoc for Day03 {
    fn part1(&self, input: &InputRep) -> Result<Box<dyn Display>> {
        let lines = input.lines();
        for line in lines {
            for x in line.match_indices(|c| c != '.') {
                println!("{x:?}");
            }
        }
        println!("todo!");
        result!(0)
    }

    fn part2(&self, _input: &InputRep) -> Result<Box<dyn Display>> {
        println!("todo!");
        result!(0)
    }
}
