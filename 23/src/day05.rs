use crate::{Aoc, Day05, Display, FileRep, Result};

use eyre::{eyre, Report};

use std::str::FromStr;

impl Aoc for Day05 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let input = &input.string;
        let (seeds, maps) = parse(input)?;

        let mut res = usize::MAX;
        for &seed in &seeds {
            let mut stack: Vec<usize> = vec![seed];
            for map in &maps {
                let src = stack[stack.len()-1];
                let mut dst = None;
                for &[d,s,l] in map {
                    if s <= src && src < s + l {
                        dst = Some(src-s+d);
                        break;
                    }
                }
                stack.push(dst.unwrap_or(src));
            }

            res = res.min(stack.pop().unwrap());
        }

        result!(res)
    }
    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let input = &input.string;
        let (seeds, mut maps) = parse(input)?;

        let mut last_map = maps.pop().unwrap();

        last_map.sort_by_key(|arr| arr[0]);
        let [d0, s0, l0] = last_map[0];

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
        result!(0)
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

    let maps: Vec<Vec<[usize;3]>> = blocks
        .map::<Result<_>, _>(|s| parse_block(s))
        .collect::<Result<_>>()?;

    Ok((seeds, maps))
}

fn parse_block(block: &str) -> Result<Vec<[usize;3]>> {
    block
        .split('\n')
        .skip(1)
        .map::<Result<_>, _>(parse_list_array)
        .collect()
}

fn parse_list(list: &str) -> Result<Vec<usize>>
{
    list
        .split_ascii_whitespace()
        .map::<Result<_>, _>(|s| Ok(s.parse()?))
        .collect()
}

fn parse_list_array<const N: usize>(list: &str) -> Result<[usize;N]> {
    let vec = list
        .split_ascii_whitespace()
        .map::<Result<_>, _>(|s| Ok(s.parse::<usize>()?))
        .collect::<Result<Vec<_>>>()?;
    Ok(vec[..].try_into()?)
}

// fn parse_list_usize(s: &str) -> Result<[usize;3]> {
//     let mut res: Vec<usize> = Vec::with_capacity(3);
//
//     for x in s.split_ascii_whitespace() {
//         res.push(x.parse::<usize>()?);
//     }
//
//     Ok(res.try_into())
// }
