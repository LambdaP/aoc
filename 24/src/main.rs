use color_eyre::eyre::Result;

use aoc2024::Aoc;

// use clap::{Parser, ValueEnum};

macro_rules! run_day {
    ($day: ident) => {
        let s = stringify!($day).to_ascii_lowercase();

        println!("=== {s} ===");
        println!("personal input:");
        aoc2024::$day.run(&format!("input/{s}.txt"))?;
    };
}

fn main() -> Result<()> {
    run_day!(Day01);

    Ok(())
}
