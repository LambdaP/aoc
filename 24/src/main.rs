use color_eyre::eyre::Result;

use aoc2024::Aoc;

macro_rules! run_day {
    ($day: ident) => {
        let s = stringify!($day).to_ascii_lowercase();

        println!("=== {s} ===");
        println!("personal input:");
        aoc2024::$day.run_file(&format!("input/{s}.txt"))?;
    };
}

fn main() -> Result<()> {
    run_day!(Day01);
    run_day!(Day02);
    run_day!(Day03);
    run_day!(Day04);
    run_day!(Day05);
    // run_day!(Day06);
    run_day!(Day07);

    Ok(())
}
