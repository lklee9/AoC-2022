use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{IResult, bytes::complete::tag};

use crate::{Solution, SolutionPair};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let cal_list: String = read_to_string("./input/1/q1").expect("File not found!");
    let l = split_cal_list(&cal_list).expect("parse error");
    let (_,v) = find_most_cal(&l);
    let sol1: usize = v;
    let sol2: usize = find_top_n_cal(&l, 3);

    (Solution::from(sol1), Solution::from(sol2))
}

fn split_cal_list(cal_list: &str) -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error + '_>> {
    let parse_int = |s: &str| {
        return s.parse::<usize>();
    };
    let int_parser = |s| -> IResult<&str, &str> {
        digit1(s)
    };
    let parse_list = map_res(int_parser, parse_int);
    let split_block = separated_list1(tag("\n"), parse_list);
    let mut split_list = separated_list1(tag("\n\n"), split_block);
    let (_, vals): (&str, Vec<Vec<usize>>) = split_list(cal_list)?;
    return Ok(vals);
}

fn find_most_cal(cal_list: &Vec<Vec<usize>>) -> (usize, usize) {
    let total_list: Vec<usize> = cal_list
        .iter().map(|x| x.iter().sum()).collect();
    let idx_max = total_list
        .iter()
        .enumerate()
        .max_by(|(_,a),(_,b)| a.cmp(b))
        .map(|(i,v)| (i,*v)).expect("cannot find max");
    return idx_max;
}

fn find_top_n_cal(cal_list: &Vec<Vec<usize>>, n: usize) -> usize {
    let total_list: Vec<usize> = cal_list
        .iter().map(|x| x.iter().sum()).collect();

    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(n);
    for t in total_list {
        if heap.len() < n {
            heap.push(Reverse(t));
            continue;
        }
        let min_item = heap.peek().expect("no item in heap?");
        if *min_item > Reverse(t) {
            heap.pop();
            heap.push(Reverse(t));
        }
    }
    return heap.drain().map(|Reverse(x): Reverse<usize>| -> usize {x}).sum();
}
