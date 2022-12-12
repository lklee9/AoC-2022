use nom::{sequence::separated_pair, character::complete::digit1, bytes::complete::tag, combinator::map_res, combinator::map, IResult, multi::separated_list1};

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("./input/04-in").expect("File not found!");
    let sol1: usize = prb1(&input);
    let sol2: usize = prb2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

struct SectionRange {
    start: usize,
    end: usize
}

impl SectionRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let sec_str = |s| -> IResult<&str, &str> { digit1(s) };
        let section = |s_str| map_res(sec_str, |x: &str| x.parse::<usize>())(s_str);
        let range_str = separated_pair(section, tag("-"), section);
        let mut range_res = map(range_str, |(s,e)| SectionRange{ start: s, end: e });
        return range_res(input);
    }
}

struct SectionPair {
    s1: SectionRange,
    s2: SectionRange
}

impl SectionPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        let pair_raw = separated_pair(SectionRange::parse, tag(","), SectionRange::parse);
        let mut pair_res = map(pair_raw, |(s1,s2)| SectionPair{ s1, s2 });
        return pair_res(input);
    }

    fn contained(&self) -> bool {
        return if self.s1.start > self.s2.start {
            self.s1.end <= self.s2.end
        } else if self.s1.start < self.s2.start {
            self.s1.end >= self.s2.end
        } else {
            true
        }
    }

    fn overlap(&self) -> bool {
        return self.s1.start >= self.s2.start && self.s1.start <= self.s2.end ||
            self.s2.start >= self.s1.start && self.s2.start <= self.s1.end;
    }
}

fn parse_pair_list(input: &str) -> Vec<SectionPair> {
    let mut pair_list_res = separated_list1(tag("\n"), SectionPair::parse);
    let (_rem, l):(&str, Vec<SectionPair>) = pair_list_res(input).expect("failed to parse list");
    return l;
}

fn prb1(input: &str) -> usize {
    let l = parse_pair_list(input);
    return l.iter().filter(|x| x.contained()).count();
}

fn prb2(input: &str) -> usize {
    let l = parse_pair_list(input);
    return l.iter().filter(|x| x.overlap()).count();
}
