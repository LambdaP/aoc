use crate::{Aoc, Day01, Display, FileRep, Result};

impl Aoc for Day01 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        // let lines = &input.byte_lines;
        // let mut res: u32 = 0;
        // for line in lines {
        //     let mut digits = line.into_iter().filter(|c| c.is_ascii_digit()).peekable();
        //     let d1 = **digits.peek().unwrap() - b'0';
        //     let d2 = *digits.last().unwrap() - b'0';
        //     res += 10 * (d1 as u32) + (d2 as u32);
        // }
        // result!(res)
        result!(0)
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.string_lines;
        let mut res: u32 = 0;
        for line in lines {
            let d1 = {
                let mut val = 0;
                let mut first_ix = usize::MAX;
                if let Some(ix) = line.find(|c: char| c.is_ascii_digit()) {
                    val = line.as_bytes()[ix] - b'0';
                    first_ix = ix;
                }

                let ixs: Vec<_> = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .map(|pat| line.find(pat).unwrap_or(usize::MAX))
                .collect();

                for i in 0..9 {
                    if ixs[i as usize] < first_ix {
                        first_ix = ixs[i as usize];
                        val = i + 1;
                    }
                }
                val
            };

            let d2 = {
                let mut last_ix = usize::MIN;
                let mut val = 0;
                if let Some(ix) = line.rfind(|c: char| c.is_ascii_digit()) {
                    val = line.as_bytes()[ix] - b'0';
                    last_ix = ix;
                }

                let ixs: Vec<_> = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .map(|pat| line.rfind(pat).unwrap_or(usize::MIN))
                .collect();

                for i in 0..9 {
                    if ixs[i as usize] > last_ix {
                        last_ix = ixs[i as usize];
                        val = i + 1;
                    }
                }

                val
            };

            res += 10 * (d1 as u32) + (d2 as u32);
        }

        result!(res)
    }
}
