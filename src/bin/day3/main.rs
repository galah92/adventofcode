use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("src/bin/day3/input.txt")?;
    let lines = std::io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let item_priorities = lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.bytes().collect::<HashSet<_>>();
            let item = right.bytes().find(|c| left.contains(c)).unwrap();
            item
        })
        .map(priority)
        .sum::<usize>();
    println!("{item_priorities}");

    let group_priorities = lines
        .iter()
        .map(|line| {
            line.bytes().fold(vec![0; 53], |mut v, b| {
                v[priority(b)] = 1;
                v
            })
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            let v = chunks
                .reduce(|a, b| a.iter().zip(b).map(|(x, y)| x + y).collect())
                .expect("must have exactly 3 chunks");
            v.iter()
                .position(|&c| c == 3)
                .expect("each group have a single common item")
        })
        .sum::<usize>();
    println!("{group_priorities}");

    Ok(())
}

fn priority(item: u8) -> usize {
    match item {
        b'a'..=b'z' => 1 + (item - b'a') as usize,
        b'A'..=b'Z' => 1 + 26 + (item - b'A') as usize,
        _ => unreachable!(),
    }
}
