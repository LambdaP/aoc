use crate::{Aoc, Day06, Display, FileRep, Result};
use eyre::{eyre, Report};

impl Aoc for Day06 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let input = &input.string;

        let vals = parse(input)?;

        for (&t,&d) in vals[0].iter().zip(vals[1].iter()) {
            let t = t as f64;
            let d = d as f64;
            let delta: f64 = t * t - 4f64 * d;
            let len = ((t + delta.sqrt())/2f64).floor() - ((t - delta.sqrt())/2f64).floor();
            println!("{t:?}, {d:?}, {:?}", len);
        }


        let res = vals[0]
            .iter()
            .zip(vals[1].iter())
            .map(|(t,d)| (((t * t - 4 * d) as f64).sqrt() - 1f64).ceil())
            .product::<f64>();

        result!(res)
    }
    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        println!("todo!");
        result!(0)
    }
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    let mut lines = input.lines().into_iter();

    let mut res = vec![];

    let times = lines.next().unwrap().strip_prefix("Time:").unwrap().trim();

    res.push(parse_list(times)?);

    let distances = lines.next().unwrap().strip_prefix("Distance:").unwrap().trim();

    res.push(parse_list(distances)?);

    Ok(res)
}

fn parse_list(list: &str) -> Result<Vec<u32>>
{
    list
        .split_ascii_whitespace()
        .map::<Result<_>, _>(|s| Ok(s.parse()?))
        .collect()
}
