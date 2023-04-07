use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = include_str!("input.txt").lines();

    let crate_lines = lines
        .by_ref()
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_rest, crate_line)| crate_line)
        })
        .collect::<Vec<_>>();
    let mut piles1 = transpose_rev(crate_lines);
    let mut piles2 = piles1.clone();

    assert!(lines.next().unwrap().is_empty());
    let instructions = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect::<Vec<_>>();

    for ins in instructions.iter() {
        for _ in 0..ins.quantity {
            let elem = piles1[ins.src].pop().unwrap();
            piles1[ins.dst].push(elem);
        }
    }
    let result: String = piles1.iter().map(|pile| pile.last().unwrap().0).collect();
    println!("{result}");

    for ins in instructions.iter() {
        let elements = (0..ins.quantity)
            .map(|_| piles2[ins.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        for elem in elements {
            piles2[ins.dst].push(elem);
        }
    }
    let result: String = piles2.iter().map(|pile| pile.last().unwrap().0).collect();
    println!("{result}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Crate(char);

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}
