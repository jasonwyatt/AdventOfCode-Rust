use std::io::BufReader;
use std::fs::File;

pub trait Day {
    fn year(&self) -> i8;
    fn day(&self) -> i8;
    fn part1(&self, reader: &mut BufReader<File>);
    fn part2(&self, reader: &mut BufReader<File>);
}