use rand::prelude::*;
use std::env;

fn main() {
    let mode = env::args()
        .nth(1)
        .expect("you have to give mode as an argument");

    let n = env::args()
        .nth(2)
        .expect("you have to give n as an argument")
        .parse::<usize>()
        .expect("n has to be integer");

    let numbers = match &mode[..] {
        "rand" => (0..n)
            .map(|_| rand::thread_rng().gen_range(0..2 * n - 1))
            .collect(),
        "asc" => {
            let mut numbers = (0..n)
                .map(|_| rand::thread_rng().gen_range(0..10))
                .collect::<Vec<_>>();

            for i in 0..numbers.len() - 1 {
                numbers[i + 1] += numbers[i];
            }

            numbers
        }
        "desc" => {
            let mut numbers = (0..n)
                .map(|_| rand::thread_rng().gen_range(0..10))
                .collect::<Vec<usize>>();

            for i in 0..numbers.len() - 1 {
                numbers[i + 1] += numbers[i];
            }

            numbers.into_iter().rev().collect()
        }
        _ => panic!("wrong generator mode"),
    };

    let numbers = numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{} {}", n, numbers);
}
