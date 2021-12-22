mod aoc;
mod aoc2021;

use clap::Parser;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::iter::*;
use std::vec;
use aoc::Day;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // Year of Advent of Code
    #[clap(short, long)]
    year: i8,
    // Day of the advent within the year
    #[clap(short, long)]
    day: i8,
    // Part within the day
    #[clap(short, long)]
    part: i8,
    // Whether or not to use sample data.
    #[clap(short, long)]
    sample: bool,
}

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(aoc2021::Day01 {})
    ];

    let args = Args::parse();

    let mut test_qualifier: String = String::from("actual");
    if args.sample {
        test_qualifier = String::from("sample");
    }
    println!(
        "Running year 20{}, day {:02}, part {:02} ({} input)", 
        args.year, args.day, args.part, test_qualifier);

    let file_name = match args.sample {
        true => format!("day{:02}-sample", args.day),
        false => format!("day{:02}", args.day)
    };
    let pwd = env::var("PWD").expect("Can't get PWD");
    let file_location = format!("{}/input/20{}/{}.txt", pwd, args.year, file_name);

    println!("Reading from {}", file_location);
    let input_file = File::open(&file_location).expect("Oops");
    let mut reader = BufReader::new(input_file);

    let day = days.into_iter()
        .find(|day| day.year() == args.year && day.day() == args.day)
        .expect("No impl found.");

    if args.part == 1 {
        day.part1(&mut reader);
    } else {
        day.part2(&mut reader);
    }
}
