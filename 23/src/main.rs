use color_eyre::eyre::Result;

use aoc2023::Aoc;

// use clap::{Parser, ValueEnum};

macro_rules! run_day {
    ($day: ident) => {
        let s = stringify!($day).to_ascii_lowercase();

        println!("=== {s} ===");
        println!("personal input:");
        aoc2023::$day.run(&format!("input/{s}.txt"))?;
    };
}

fn main() -> Result<()> {
    run_day!(Day01);
    run_day!(Day02);
    // run_day!(Day03);
    run_day!(Day04);
    // run_day!(Day05);
    // run_day!(Day06);
    run_day!(Day10);
    run_day!(Day11);

    Ok(())
}
