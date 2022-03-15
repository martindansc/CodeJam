use priority_queue::PriorityQueue;
use std::io;

fn read_number() -> i32 {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let x: i32 = input_line.trim().parse().expect("Input not an integer");
    return x;
}

fn main() {
    let mut pq = PriorityQueue::new();

    let mut a_trains : [i32; 2] = [0, 0];
    a_trains[0] = a_trains[0] + 1; 

    pq.push("Apples", 5);
    pq.push("Bananas", 8);
    pq.push("Strawberries", 23);

    let num = read_number();

    println!("{}", num);


    for (item, _) in pq.into_sorted_iter() {
        println!("{}", item);
    }
}
