use rand::prelude::*;
use std::env;

fn main() {
    let n = env::args()
        .nth(1)
        .expect("you have to give n as an argument")
        .parse::<usize>()
        .expect("n has to be integer");

    let mut numbers = (0..n)
        .map(|_| rand::thread_rng().gen_range(0..10))
        .collect::<Vec<_>>();

    for i in 0..numbers.len() - 1 {
        numbers[i + 1] += numbers[i];
    }

    let numbers = numbers
        .into_iter()
        .rev()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{} {}", n, numbers);
}
