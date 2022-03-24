use std::env;

use algorithms::generator::*;

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
        "rand" => RandomGenerator::generate(n),
        "asc" => AscendingGenerator::generate(n),
        "desc" => DescendingGenerator::generate(n),
        _ => panic!("wrong generator mode"),
    };

    let numbers = numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{} {}", n, numbers);
}
