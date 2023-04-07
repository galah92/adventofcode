use std::{io::BufRead, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("src/bin/day4/input.txt")?;
    let reader = std::io::BufReader::new(file);
    let lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let num_contains = lines
        .iter()
        .filter_map(|line| RangePair::from_str(line).ok())
        .filter(|range_pair| range_pair.contain())
        .count();
    println!("{num_contains}");

    let num_overlaps = lines
        .iter()
        .filter_map(|line| RangePair::from_str(line).ok())
        .filter(|range_pair| range_pair.overlap())
        .count();
    println!("{num_overlaps}");

    Ok(())
}

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn overlaps(&self, other: &Self) -> bool;
}

impl InclusiveRangeExt for std::ops::RangeInclusive<usize> {
    fn contains_range(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}

struct RangePair(
    std::ops::RangeInclusive<usize>,
    std::ops::RangeInclusive<usize>,
);

impl FromStr for RangePair {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let first = parts.next().unwrap();
        let mut first_parts = first.split('-');
        let start = first_parts.next().unwrap().parse().unwrap();
        let end = first_parts.next().unwrap().parse().unwrap();
        let first = start..=end;

        let second = parts.next().unwrap();
        let mut second_parts = second.split('-');
        let start = second_parts.next().unwrap().parse().unwrap();
        let end = second_parts.next().unwrap().parse().unwrap();
        let second = start..=end;

        Ok(RangePair(first, second))
    }
}

impl RangePair {
    fn contain(&self) -> bool {
        self.0.contains_range(&self.1) || self.1.contains_range(&self.0)
    }

    fn overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}
