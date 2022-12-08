use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::sequence::separated_pair;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::is_space;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

enum Hand { R, P, S }

fn parse_hand(hand_str: &str) -> Result<Hand, &'static str> {
    match hand_str {
        "A" | "X" => Ok(Hand::R),
        "B" | "Y" => Ok(Hand::P),
        "C" | "Z" => Ok(Hand::S),
        _ => Err("Not a valid hand!")
    }
}

fn parse_strat(strat_str: &str) -> Vec<(Hand,Hand)> {
    let is_hand = |s: &str| -> IResult<&str, &str> { is_a(s) };
    let hand = |x: &str| map_res(is_a(x), |x: &str| parse_hand(x));
    let round = separated_pair(hand("ABC"), tag(" "), hand("XYZ"));
    let rounds = separated_list1(tag("\n"), round);
    let (rem, val) = rounds(strat_str).expect("Parsing stratergy failed");
    println!("remining txt: {:?}", rem);
    return val;
}
