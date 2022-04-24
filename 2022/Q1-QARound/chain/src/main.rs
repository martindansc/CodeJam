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

fn read_number<'a>(input: &mut VecDeque<&'a str>) -> Option<i64> {
    match input.pop_front() {
        Some(value) => return Some(value.trim().parse().expect("I was expecting a number")),
        None => return None
    }
}

fn read_word<'a>(mut input: &mut VecDeque<&'a str>) -> Option<&'a str> {
    return input.pop_front();
}

struct Node<'a> {
    id: i64,
    value: i64,
    next: Option<&'a mut Node<'a>>
}

fn solve(words_input: &mut VecDeque<&str>) {
    let n_nodes = read_number(words_input).expect("First should be a number");
    let mut node_list = Vec::<Node>::new();

    for i in 0..n_nodes {
        let value = read_number(words_input).expect("Should read a number");

        let mut next: Node = Node {
            id: i,
            value: value,
            next: None
        };

        node_list.push(next);
    }

    for i in 0..n_nodes {
        let points_to = read_number(words_input).expect("Should read a number");
       
        let mut current_node = node_list.get_mut(i as usize).unwrap();
        let pointed_node =  node_list.get_mut(points_to as usize);
        current_node.next = pointed_node;
    }
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
