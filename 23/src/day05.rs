use crate::{Aoc, Day05, InputRep, Result};

use eyre::eyre;

impl Aoc for Day05 {
    type Output = usize;
    fn part1(&self, input: &InputRep) -> Result<Self::Output> {
        let input = input.as_str();
        let (seeds, maps) = parse(input)?;

        let mut res = usize::MAX;
        for &seed in &seeds {
            let mut stack: Vec<usize> = vec![seed];
            for map in &maps {
                let src = stack[stack.len() - 1];
                let mut dst = None;
                for &[d, s, l] in map {
                    if s <= src && src < s + l {
                        dst = Some(src - s + d);
                        break;
                    }
                }
                stack.push(dst.unwrap_or(src));
            }

            res = res.min(stack.pop().unwrap());
        }

        Ok(res)
    }
    fn part2(&self, input: &InputRep) -> Result<Self::Output> {
        let input = input.as_str();
        let (_seeds, mut maps) = parse(input)?;

        let mut last_map = maps.pop().unwrap();

        last_map.sort_by_key(|arr| arr[0]);
        let [_d0, _s0, _l0] = last_map[0];

        // let mut penultimate_map = maps.pop().unwrap();
        //
        // for &[d,s,l] in &penultimate_map {
        // }
        //
        //
        //
        // maps.iter_mut().for_each(|map| map.sort_by_key(|&[d,s,_]| ((d as isize) - (s as isize), s)));
        //
        // for map in &maps[0] {
        //         println!("{:?}--{:?}: delta is {:?}", map[1], map[1]+map[2], (map[0] as isize) - (map[1] as isize));
        // }

        println!("todo!");
        Ok(0)
    }
}

fn parse(file: &str) -> Result<(Vec<usize>, Vec<Vec<[usize; 3]>>)> {
    let mut blocks = file.trim().split("\n\n");

    let seeds = blocks
        .next()
        .ok_or_else(|| eyre!("parse error"))?
        .strip_prefix("seeds: ")
        .ok_or_else(|| eyre!("parse error"))?;
    let seeds = parse_list(seeds)?;

    let maps: Vec<Vec<[usize; 3]>> = blocks
        .map::<Result<_>, _>(parse_block)
        .collect::<Result<_>>()?;

    Ok((seeds, maps))
}

fn parse_block(block: &str) -> Result<Vec<[usize; 3]>> {
    block
        .split('\n')
        .skip(1)
        .map::<Result<_>, _>(parse_list_array)
        .collect()
}

fn parse_list(list: &str) -> Result<Vec<usize>> {
    list.split_ascii_whitespace()
        .map::<Result<_>, _>(|s| Ok(s.parse()?))
        .collect()
}

fn parse_list_array<const N: usize>(list: &str) -> Result<[usize; N]> {
    let vec = list
        .split_ascii_whitespace()
        .map::<Result<_>, _>(|s| Ok(s.parse::<usize>()?))
        .collect::<Result<Vec<_>>>()?;
    Ok(vec[..].try_into()?)
}
