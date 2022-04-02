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

fn words(s: &str) -> VecDeque<&str> {
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

// --- SOLVE ---

fn get_min_color(tonners: &Vec<Vec<i32>>, i: usize) -> i32 {
    let mut min: i32 = -1;
    for tonner in tonners {
        let value = tonner[i];
        if min < 0 || value < min {
            min = value;
        }
    }

    return min;
}

fn read_tonner(words_input: &mut VecDeque<&str>) -> Vec<i32> {
    let mut tonner: Vec<i32> = Vec::new();
    for _i in 1..=4 {
        let number = read_number(words_input).expect("Reading number for tonner");
        tonner.push(number);
    }

    return tonner;
}

fn solve(words_input: &mut VecDeque<&str>) {
    let mut tonners: Vec<Vec<i32>> = Vec::new();

    for _i in 1..=3 {
        let tonner = read_tonner(words_input);
        tonners.push(tonner);
    }

    let target = 1000000;
    let mut sum = 0;
    let mut solution: Vec<i32> = Vec::with_capacity(4);

    for i in 0..4 {
        let mut min_color = get_min_color(&tonners, i);
        if sum + min_color > target {
            min_color = target - sum;
        }
        solution.push(min_color);
        sum = sum + min_color;
    }

    if sum < target {
        print!("IMPOSSIBLE");
    }
    else {
        for i in 0..4 {
            print!("{}", solution[i]);
            if i < 3 {
                print!(" ");
            }
        }
    }

}


// --- MAIN ---

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
