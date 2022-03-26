use std::io::Read;

use algorithms::benchmarking::StandardBenchmarker;
use algorithms::benchmarking::Stats;
use algorithms::prelude::*;

fn main() {
    let mode = std::env::args()
        .nth(1)
        .expect("there has to be a mode given");

    let mut buffer = Vec::new();

    std::io::stdin()
        .read_to_end(&mut buffer)
        .expect("error reading input");

    let input = String::from_utf8(buffer).expect("couldn't parse data");
    let mut numbers_iterator = input
        .split_whitespace()
        .filter_map(|number| number.parse::<i32>().ok());

    let n = numbers_iterator.next().expect("n has to be given");

    let mut numbers = numbers_iterator.take(n as usize).collect::<Vec<_>>();

    let mut benchmark = StandardBenchmarker::default();

    if n < 50 {
        println!("Input array: {:?}", numbers);
    }

    match mode.as_str() {
        "insertion" => InsertionSort::sort_with_benchmark(&mut numbers, &mut benchmark),
        "merge" => MergeSort::sort_with_benchmark(&mut numbers, &mut benchmark),
        "quick" => QuickSort::sort_with_benchmark(&mut numbers, &mut benchmark),
        "dual_pivot" => DualPivotQuicksort::sort_with_benchmark(&mut numbers, &mut benchmark),
        _ => panic!("wrong program mode"),
    }

    if n < 50 {
        println!("Output array: {:?}", numbers);
    }

    let Stats {
        comparisons, swaps, ..
    } = benchmark.get_stats();

    println!("Number of comparisons: {}", comparisons);
    println!("Number of swaps: {}", swaps);

    let mut sorted = true;

    for i in 0..numbers.len() - 1 {
        if numbers[i] > numbers[i + 1] {
            sorted = false;
            break;
        }
    }

    if sorted {
        println!("The array is sorted.");
    } else {
        println!("The array is not sorted.");
    }
}
