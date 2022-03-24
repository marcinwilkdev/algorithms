use rand::prelude::*;
use std::env;

fn main() {
    let n = env::args()
        .nth(1)
        .expect("you have to give n as an argument")
        .parse::<usize>()
        .expect("n has to be integer");

    let numbers = (0..n)
        .map(|_| rand::thread_rng().gen_range(0..2 * n - 1))
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", numbers);
}
