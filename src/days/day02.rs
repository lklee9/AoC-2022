use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::sequence::separated_pair;
use nom::bytes::complete::{is_a, tag};
use nom::{combinator::map, multi::separated_list1};
use nom::IResult;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input: String = read_to_string("./input/02-in").expect("File not found!");
    let sol1: usize = prb1(&input);
    let sol2: usize = prb2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn prb1(input: &str) -> usize {
    let rounds = parse_strat(input);
    let me_p1 = false;
    let mut score = 0;
    for round in rounds {
        score += round.score(me_p1) + round.get(me_p1).score();
    }
    return score;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Hand { R, P, S }

impl Hand {
    fn score(&self) -> usize {
        return match self {
            Hand::R => 1,
            Hand::P => 2,
            Hand::S => 3,
        };
    }

    fn parse(hand_str: &str) -> Option<Self> {
        match hand_str {
            "A" | "X" => Some(Hand::R),
            "B" | "Y" => Some(Hand::P),
            "C" | "Z" => Some(Hand::S),
            _ => None
        }
    }

    fn from_u8(val: u8) -> Hand {
        match val {
            0 => Hand::R,
            1 => Hand::P,
            2 => Hand::S,
            _ => panic!("unrecognised hand idx")
        }
    }
}

struct Round {
    p1: Hand,
    p2: Hand
}

impl Round {
    fn parse(input: &str) -> IResult<&str, Self> {
        let two_hand = separated_pair(is_a("ABC"), tag(" "), is_a("XYZ"));
        return map(two_hand, |(x, y)| Round {
            p1: Hand::parse(x).unwrap(),
            p2: Hand::parse(y).unwrap()
        })(input);
    }
    fn get(&self, get_p1: bool) -> Hand {
        return match get_p1 {
            true => self.p1,
            false => self.p2
        };
    }
    fn score(&self, score_p1: bool) -> usize {
        let (me,you) = if score_p1 {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        };
        return if me == you {
            3
        } else if ( *me as u8 + 1 ) % 3 == *you as u8 {
            0
        } else { 6 };
    }
}

fn parse_strat(strat_str: &str) -> Vec<Round> {
    let mut rounds = separated_list1(tag("\n"), Round::parse);
    let (rem, val) = rounds(strat_str).expect("Parsing stratergy failed");
    println!("remining txt: {:?}", rem);
    return val;
}

/////////////////

enum Result { W, L, D }

impl Result {
    fn parse(input: &str) -> Result {
        return match input {
            "X" => Result::L,
            "Y" => Result::D,
            "Z" => Result::W,
            _ => panic!("unrecognised goal result")
        }
    }
}

fn get_strategy(hand: Hand, goal_result:  &Result) -> Hand {
    let hand_idx = match goal_result {
        Result::W => ( hand as u8 + 1 ) % 3,
        Result::L => ( hand as u8 + 2 ) % 3,
        Result::D => hand as u8
    };
    return Hand::from_u8(hand_idx);
}

struct RoundStrat {
    h: Hand,
    r: Result
}

impl RoundStrat {
    fn parse(input: &str) -> IResult<&str, Self> {
        let round = separated_pair(is_a("ABC"), tag(" "), is_a("XYZ"));
        return map(round, |(h, r)| RoundStrat {
            h: Hand::parse(h).unwrap(),
            r: Result::parse(r)
        })(input);
    }
    fn score(self) -> usize {
        let res = get_strategy(self.h, &self.r);
        return res.score() + match self.r {
            Result::W => 6,
            Result::D => 3,
            Result::L => 0,
        };
    }
}

fn prb2(input: &str) -> usize {
    let mut rounds = separated_list1(tag("\n"), RoundStrat::parse);
    let (rem, val) = rounds(input).expect("Parsing stratergy failed");
    println!("remining txt: {:?}", rem);
    let mut score = 0;
    for round in val {
        score += round.score();
    }
    return score;
}
