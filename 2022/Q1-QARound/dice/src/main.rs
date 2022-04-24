use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
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

fn solve(words_input: &mut VecDeque<&str>) {
    let mut ordered = BinaryHeap::new();
    let n = read_number(words_input).expect("Should read a number");

    for _i in 0..n {
        let read = read_number(words_input).expect("Should read a number");
        ordered.push(-1 * read);
    }

    let mut max = 0;
    while ordered.len() > 0 {
        let next = -1 * ordered.pop().expect("Must be a number");
        if next > max {
            max = max + 1;
        }
    }
    print!("{}", max);
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
