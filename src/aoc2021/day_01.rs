use crate::aoc::Day;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct Day01 {}

impl Day for Day01 {
    fn year(&self) -> i8 {
        return 21;
    }

    fn day(&self) -> i8 {
        return 1;
    }

    fn part1(&self, reader: &mut BufReader<File>) {
        let mut line = String::new();
        let mut count = 0;
        let mut last = 0;
        let mut i = 0;
        while reader.read_line(&mut line).expect("Couldn't Read") > 0 {
            let num_value: u32 = line.trim().parse().unwrap();
            line.clear();

            if i > 0 && num_value > last {
                count += 1;
            }
            last = num_value;
            i += 1;
        }
        println!("{}", count);
    }

    fn part2(&self, reader: &mut BufReader<File>) {
        let mut line = String::new();
        let mut lines: Vec<u32> = Vec::new();
        while reader.read_line(&mut line).expect("Couldn't Read") > 0 {
            lines.push(line.trim().parse().unwrap());
            line.clear();
        }

        let mut count = 0;
        let mut last = 0;
        let mut i = 0;

        lines.windows(3)
            .for_each(
                |items| {
                    let sum = items[0] + items[1] + items[2];
                    if i > 0 && sum > last {
                        count += 1;
                    }
                    i += 1;
                    last = sum;
                });


        println!("{}", count);
    }
}