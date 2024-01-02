use std::env;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(
        args.len() == 3,
        "Expected 2 arguments, got {}. Usage: cargo run <day> <part>\nExample: cargo run 1 1",
        args.len() - 1
    );

    let day = &args[1];
    let part = &args[2];

    match day.as_str() {
        "1" => match part.as_str() {
            "1" => utils::profile(solutions::day1_1::main),
            "2" => utils::profile(solutions::day1_2::main),
            _ => panic!("Invalid part: {}", part),
        },
        "2" => match part.as_str() {
            "1" => utils::profile(solutions::day2_1::main),
            "2" => utils::profile(solutions::day2_2::main),
            _ => panic!("Invalid part: {}", part),
        },
        "3" => match part.as_str() {
            "1" => utils::profile(solutions::day3_1::main),
            "2" => panic!("Day 3 Part 2 is not implemented yet!"),
            _ => panic!("Invalid part: {}", part),
        },
        day if (3..=25).contains(&day.parse::<usize>().unwrap_or_default()) => {
            panic!("Day {} is not implemented yet!", day)
        }
        _ => panic!("Invalid day: {}!", day),
    }
}
