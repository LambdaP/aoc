use std::io;
use std::io::Read;
use std::env;
use std::string::String;
use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

type Solver = fn(&[&[u8]]) -> anyhow::Result<usize>;

fn main() -> Result<()> {
    let day = env::args().nth(1).expect("no args");

    let (part1, part2) : (Solver, Solver) = match &day as &str {
        "--day1" => (day1::part1, day1::part2),
        "--day2" => (day2::part1, day2::part2),
        "--day3" => (day3::part1, day3::part2),
        "--day4" => (day4::part1, day4::part2),
        "--day5" => (day5::part1, day5::part2),
        "--day6" => (day6::part1, day6::part2),
        "--day7" => (day7::part1, day7::part2),
        "--day8" => (day8::part1, day8::part2),
        "--day9" => (day9::part1, day9::part2),
        "--day10" => (day10::part1, day10::part2),
        _      => { eprintln!("error: {} is not a valid option", day); panic!() }
    };

    let lines = stdin_bytes()?;

    let mut lines = lines
            .split(|&x| x == b'\n')
            .collect::<Vec<&[u8]>>();

    if let Some(l) = lines.last() {
        if l.len() == 0 {
            lines.pop();
        }
    }

    let lines = lines;

    let res1 = part1(&lines)?;
    println!("{}", res1);

    let res2 = part2(&lines)?;
    println!("{}", res2);

    Ok(())
}

pub fn stdin_bytes() -> Result<Vec<u8>> {
    let mut res: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut res)?;
    Ok(res)
}

pub fn stdin_utf8() -> Result<String> {
    let mut res: String = String::new();
    io::stdin().read_to_string(&mut res)?;
    Ok(res)
}
