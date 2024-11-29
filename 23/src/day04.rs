use crate::{Aoc, Day04, Display, InputRep, Result};

use eyre::{eyre, Report};

use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq)]
struct Card {
    id: usize,
    win: Vec<u8>,
    nos: Vec<u8>,
}

impl FromStr for Card {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut card: Card = Default::default();

        let s = s.strip_prefix("Card").ok_or_else(|| eyre!("Parse error"))?;
        let s = s.trim_start();
        let (id_s, s) = s.split_once(':').ok_or_else(|| eyre!("Parse error"))?;
        card.id = id_s.parse()?;

        let (win_s, s) = s.split_once('|').ok_or_else(|| eyre!("Parse error"))?;
        let win_s = win_s.trim();
        card.win = win_s
            .split_ascii_whitespace()
            .map::<Result<u8>, _>(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;

        let s = s.trim();
        card.nos = s
            .split_ascii_whitespace()
            .map::<Result<u8>, _>(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;

        Ok(card)
    }
}

impl Card {
    fn n_winning(&self) -> usize {
        self.nos.iter().filter(|n| self.win.contains(n)).count()
    }
}

impl Aoc for Day04 {
    fn part1(&self, input: &InputRep) -> Result<Box<dyn Display>> {
        let lines = input.lines();
        let mut res = 0;

        for line in lines {
            let n = line.parse::<Card>()?.n_winning();
            res += (1 << n) >> 1;
        }

        result!(res)
    }
    fn part2(&self, input: &InputRep) -> Result<Box<dyn Display>> {
        let lines = input.lines();
        let mut copies: Vec<u32> = vec![0; lines.len() + 1];
        for line in lines {
            let card: Card = line.parse()?;

            let id = card.id;
            copies[id] += 1;

            let n = card.n_winning();

            for i in 0..n {
                copies[id + i + 1] += copies[id];
            }
        }

        result!(copies.iter().sum::<u32>())
    }
}
