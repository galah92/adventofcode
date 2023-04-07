use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/bin/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elf_calories = 0;
    let mut elf_carrying_max = 0;
    let mut heap = BinaryHeap::new();
    for _ in 0..3 {
        heap.push(Reverse(0));
    }

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elf_carrying_max = std::cmp::max(elf_carrying_max, elf_calories);
            heap.push(Reverse(elf_calories));
            heap.pop();
            elf_calories = 0;
        } else {
            let calories = line.parse::<i32>().unwrap();
            elf_calories += calories;
        }
    }

    println!("{elf_carrying_max}");

    let top_3_elves_carrying_max = heap.iter().map(|x| x.0).sum::<i32>();
    println!("{top_3_elves_carrying_max}");
}
