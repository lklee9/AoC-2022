use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use bit_set::BitSet;
use nom::{multi::separated_list1, character::{is_alphabetic, complete::alpha1}, bytes::complete::tag, IResult};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("./input/03-in").expect("File not found!");
    let sol1: usize = prb1(&input);
    let sol2: usize = prb2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn priority(c: char) -> usize {
    assert!(c.is_ascii_alphabetic());
    return if c.is_ascii_uppercase() {
        c as usize - 38
    } else {
        c as usize - 96
    };
}

fn find_sum_shared_items(rucksack: &str) -> usize {
    let sack1_bound = ( rucksack.len() - 1 ) / 2;
    let sack2_bound = rucksack.len() / 2;

    let mut i = 0;
    let mut sack1 = BitSet::with_capacity(52);
    let mut sack2 = BitSet::with_capacity(52);
    for c in rucksack.chars() {
        let p = priority(c);
        if i <= sack1_bound {
            sack1.insert(p);
        }
        if i >= sack2_bound {
            sack2.insert(p);
        }
        i += 1;
    }
    let mut total = 0;
    for x in sack1.intersection(&sack2) {
        total += x;
    }
    return total;
}

fn parse_rucksack_str(input: &str) -> Vec<&str> {
    let item = |x| -> IResult<&str, &str> { alpha1(x) };
    let mut rucksacks = separated_list1(tag("\n"), item);
    let (_rem, val) = rucksacks(input).expect("cannot parse input");
    return val;
}

fn prb1(input: &str) -> usize {
    let val = parse_rucksack_str(input);
    let mut total = 0;
    for rucksack in val {
        total += find_sum_shared_items(rucksack);
    }
    return total;
}

///////

fn rucksack_to_bitset(rucksack: &str) -> BitSet {
    let mut set = BitSet::with_capacity(52);
    for c in rucksack.chars() {
        set.insert(priority(c));
    }
    return set;
}

fn prb2(input: &str) -> usize {
    let rucksacks = parse_rucksack_str(input);
    let mut total = 0;
    for group in rucksacks.chunks(3) {
        let mut set = rucksack_to_bitset(group[0]);
        for r in group {
            set.intersect_with(&rucksack_to_bitset(r));
        }
        for i in set.iter() {
            total += i;
        }
    }
    return total;
}
