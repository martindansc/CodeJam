use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn flatten<T>(nested: Vec<Vec<T>>) -> VecDeque<T> {
    nested.into_iter().flatten().collect()
}

fn read_input() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn words<'a>(s: &'a str) -> VecDeque<&'a str> {
    flatten(s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect())
}

fn read_number<'a>(input: &mut VecDeque<&'a str>) -> Option<i32> {
    match input.pop_front() {
        Some(value) => return Some(value.trim().parse().expect("I was expecting a number")),
        None => return None
    }
}

fn read_word<'a>(mut input: &mut VecDeque<&'a str>) -> Option<&'a str> {
    return input.pop_front();
}

#[derive(PartialEq, Eq, Hash)]
enum Station {
    A,
    B,
}

#[derive(PartialEq, Eq, Hash)]
struct Travel {
    station: Station,
    out_time: i32
}

fn solve(words_input: &mut VecDeque<&str>) {
    let starts_from_total = read_number(words_input).expect("A starts should be a number");
    let starts_from_a = read_number(words_input).expect("A starts should be a number");
    let starts_from_b = read_number(words_input).expect("B starts should be a number");
    print!("{0} {1}", starts_from_a, starts_from_b);
}

fn main() {
    let whole_file = read_input().unwrap();
    let mut words_input = words(&whole_file);

    let num = read_number(&mut words_input).expect("First should be a number");

    for n in 1..=num {
        print!("Case #{0}: ", n);
        solve(&mut words_input);
        println!();
    }
}
