use color_eyre::eyre::Result;

use aoc2023::Aoc;

// use clap::{Parser, ValueEnum};

macro_rules! run_day {
    ($day: ident) => {
        let s = stringify!($day).to_ascii_lowercase();

        println!("=== {s} ===");
        println!("dummy input:");
        aoc2023::$day.run(&format!("input/{s}.dummy.txt"))?;
        println!("personal input:");
        aoc2023::$day.run(&format!("input/{s}.txt"))?;
    };
}

fn main() -> Result<()> {
    run_day!(Day01);
    run_day!(Day02);

    Ok(())
}
