use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let dir_sizes = get_dir_sizes(lines);

    let sum = dir_sizes.iter().filter(|s| s <= &&100_000).sum::<u64>();
    println!("{sum}");

    let root_size = dir_sizes.iter().max().unwrap();
    let threshold = 30_000_000 - (70_000_000 - root_size);
    let to_delete = dir_sizes.iter().filter(|s| s >= &&threshold).min().unwrap();
    println!("{to_delete}");
}

fn get_dir_sizes(lines: impl Iterator<Item = Line>) -> Vec<u64> {
    let mut stack = vec![0];
    let mut sizes = Vec::new();

    for line in lines {
        match line {
            Line::Command(Command::Ls) => (), // nothing to do
            Line::Command(Command::Cd(path)) => match path.as_str() {
                "/" => (), // nothing to do as happens only at the beginning
                ".." => {
                    let dir = stack.pop().unwrap();
                    sizes.push(dir); // done travering dir so add it to sizes
                    *stack.last_mut().unwrap() += dir; // add dir size to parent dir
                }
                _ => stack.push(0),
            },
            Line::Entry(Entry::Dir(_)) => (), // push dir to stack when cd'ing into it
            Line::Entry(Entry::File(size, _path)) => {
                *stack.last_mut().unwrap() += size;
            }
        }
    }

    while stack.len() > 1 {
        let dir = stack.pop().unwrap();
        sizes.push(dir); // done trave so add it to sizes
        *stack.last_mut().unwrap() += dir; // add dir size to parent dir
    }
    let root = stack.pop().unwrap();
    sizes.push(root);

    sizes
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_path(s: &str) -> IResult<&str, String> {
    map(
        take_while1(|c| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        |s: &str| s.into(),
    )(s)
}

fn parse_line(s: &str) -> IResult<&str, Line> {
    alt((
        map(
            preceded(
                tag("$ "),
                alt((
                    map(tag("ls"), |_| Command::Ls),
                    map(preceded(tag("cd "), parse_path), Command::Cd),
                )),
            ),
            Line::Command,
        ),
        map(
            alt((
                map(
                    separated_pair(nom::character::complete::u64, tag(" "), parse_path),
                    |(size, path)| Entry::File(size, path),
                ),
                map(preceded(tag("dir "), parse_path), Entry::Dir),
            )),
            Line::Entry,
        ),
    ))(s)
}
