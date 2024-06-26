mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod helpers;

use clap::Parser;
use eyre::Result;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    single: bool,
    #[arg(long)]
    day1: bool,
    #[arg(long)]
    day2: bool,
    #[arg(long)]
    day3: bool,
    #[arg(long)]
    day4: bool,
    #[arg(long)]
    day5: bool,
    #[arg(long)]
    day6: bool,
    #[arg(long)]
    day7: bool,
    #[arg(long)]
    day8: bool,
    #[arg(long)]
    day9: bool,
    #[arg(long)]
    day10: bool,
    #[arg(long)]
    day11: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    if !args.single || args.day1 {
        // Day 1 Part 1
        let day1_part1 = day1::day1(false, None);
        println!("Day 1 Part 1: {}", day1_part1);
        // Day 2 Part 2
        let day1_part2 = day1::day1(true, None);
        println!("Day 1 Part 2: {}", day1_part2);
    }
    if !args.single || args.day2 {
        // Day 2 part 1
        let day2_part1 = day2::day2_part1(None)?;
        println!("Day 2 Part 1: {}", day2_part1);
        // Day 2 part 2
        let day2_part2 = day2::day2_part2(None)?;
        println!("Day 2 Part 2: {}", day2_part2);
    }
    if !args.single || args.day3 {
        // Day 3 part 1
        let day3_part1 = day3::day3(false, None);
        println!("Day 3 Part 1: {}", day3_part1);
        // Day 3 part 2
        let day3_part2 = day3::day3(true, None);
        println!("Day 3 Part 2: {}", day3_part2);
    }
    if !args.single || args.day4 {
        // Day 4 part 1
        let day4_part1 = day4::day4(false, None)?;
        println!("Day 4 Part 1: {}", day4_part1);
        // Day 4 part 2
        let day4_part2 = day4::day4(true, None)?;
        println!("Day 4 Part 2: {}", day4_part2);
    }
    if !args.single || args.day5 {
        // Day 5
        let day5 = day5::day5(None)?;
        println!("Day 5 Part 1: {}", day5.0);
        println!("Day 5 Part 2: {}", day5.1)
    }
    if !args.single || args.day6 {
        // Day 6
        let day6 = day6::day6(None)?;
        println!("Day 6 Part 1: {}", day6.0);
        println!("Day 6 Part 2: {}", day6.1);
    }
    if !args.single || args.day7 {
        // Day 7
        let day7 = day7::day7(None)?;
        println!("Day 7 Part 1: {}", day7.0);
        println!("Day 7 Part 2: {}", day7.1);
    }
    if !args.single || args.day8 {
        // Day 8
        let day8 = day8::day8(None)?;
        println!("Day 8 Part 1: {}", day8.0);
        // TODO finish this
        println!("Day 8 Part 2: {}", day8.1);
    }
    
    // if !args.single || args.day9 {
    //     // Day 9
    //     let day9 = day9::day9(None).await;
    //     println!("Day 9 Part 1: {}", day9.0);
    //     // let day9 = day9::day9(None, true).await;
    //     println!("Day 9 Part 2: {}", day9.1);
    // }
    //
    // if !args.single || args.day10 {
    //     // Day 10
    //     let day10 = day10::day10(None).await;
    //     println!("Day 10 Part 1: {}", day10.0);
    //     println!("Day 10 Part 2: {}", day10.1);
    // }
    //
    // if !args.single || args.day11 {
    //     // Day 11
    //     let day11_part1 = day11::day11(None, 1);
    //     let day11_part2 = day11::day11(None, 999_999);
    //     println!("Day 11 Part 1: {}", day11_part1);
    //     println!("Day 11 Part 2: {}", day11_part2);
    // }

    Ok(())
}
