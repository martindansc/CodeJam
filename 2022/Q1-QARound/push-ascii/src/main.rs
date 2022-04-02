use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
// --- READ INPUT ---

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

fn read_word<'a>(input: &mut VecDeque<&'a str>) -> Option<&'a str> {
    return input.pop_front();
}

// SOLVE

fn solve(words_input: &mut VecDeque<&str>) {
    let rows = read_number(words_input).expect("Rows should be a number");
    let columns = read_number(words_input).expect("Columns starts should be a number");

    println!();
    for i in 0..=rows * 2 {
        for j in 0..=columns * 2 {
            if i < 2 && j < 2 {
                print!(".");
                continue;
            }

            if i%2 == 0 {
                if j%2 == 0 {
                    print!("+");
                }
                else {
                    print!("-");
                }
            }
            else {
                if j%2 == 0 {
                    print!("|");
                }
                else {
                    print!(".");
                }
            }
        }
        println!();
    }
}


// MAIN

fn main() {
    let whole_file = read_input().unwrap();
    let mut words_input = words(&whole_file);

    let num = read_number(&mut words_input).expect("First should be a number");

    for n in 1..=num {
        print!("Case #{0}: ", n);
        solve(&mut words_input);
    }
}
