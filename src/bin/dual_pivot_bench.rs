use std::fs::File;
use std::io::Write;

use algorithms::benchmarking::*;
use algorithms::generator::*;
use algorithms::prelude::*;

fn main() {
    let k = std::env::args()
        .nth(1)
        .expect("there has to be a k given")
        .parse()
        .expect("k isn't valid number");

    let mut quick_final_stats = Vec::with_capacity(10);
    let mut dual_pivot_final_stats = Vec::with_capacity(10);

    for i in 1..=10 {
        let n = 100 * i;

        let mut quick_results = Vec::with_capacity(k);
        let mut dual_pivot_results = Vec::with_capacity(k);

        for _ in 0..k {
            let random_array = RandomGenerator::generate(n);

            let mut quick_benchmark = StandardBenchmarker::default();
            let mut dual_pivot_benchmark = StandardBenchmarker::default();

            QuickSort::sort_with_benchmark(&mut random_array.clone(), &mut quick_benchmark);
            DualPivotQuicksort::sort_with_benchmark(&mut random_array.clone(), &mut dual_pivot_benchmark);

            quick_results.push(quick_benchmark.get_stats());
            dual_pivot_results.push(dual_pivot_benchmark.get_stats());
        }

        let quick_comps_and_swaps = get_avg_stats(&quick_results, k);
        let dual_pivot_comps_and_swaps = get_avg_stats(&dual_pivot_results, k);

        quick_final_stats.push(quick_comps_and_swaps);
        dual_pivot_final_stats.push(dual_pivot_comps_and_swaps);
    }

    print_final_stats_to_file("quick", StatType::Comps, &quick_final_stats);
    print_final_stats_to_file("quick", StatType::Swaps, &quick_final_stats);
    print_final_stats_to_file("dual_pivot", StatType::Comps, &dual_pivot_final_stats);
    print_final_stats_to_file("dual_pivot", StatType::Swaps, &dual_pivot_final_stats);

    let (const_swaps_quicksort, const_comps_quicksort) = calculate_const_in_front_of_n_lg_n(&quick_final_stats);
    let (const_swaps_dual_pivot, const_comps_dual_pivot) = calculate_const_in_front_of_n_lg_n(&dual_pivot_final_stats);

    println!("Const in front of n*log(n) in quicksort swaps: {}", const_swaps_quicksort);
    println!("Const in front of n*log(n) in quicksort comps: {}", const_comps_quicksort);
    println!("Const in front of n*log(n) in dual pivot swaps: {}", const_swaps_dual_pivot);
    println!("Const in front of n*log(n) in dual pivot comps: {}", const_comps_dual_pivot);
}

fn get_avg_stats(stats: &[Stats], k: usize) -> (f64, f64) {
    let (mut insertion_comps, mut insertion_swaps) = stats
        .iter()
        .map(
            |Stats {
                 comparisons, swaps, ..
             }| (*comparisons, *swaps),
        )
        .fold((0.0, 0.0), |(total_comps, total_swaps), (comps, swaps)| {
            (total_comps + comps as f64, total_swaps + swaps as f64)
        });

    insertion_comps /= k as f64;
    insertion_swaps /= k as f64;

    (insertion_comps, insertion_swaps)
}

fn print_final_stats_to_file(algorithm: &str, type_of_stat: StatType, final_stats: &[(f64, f64)]) {
    let type_of_stat_string = match type_of_stat {
        StatType::Comps => "comps",
        StatType::Swaps => "swaps",
    };

    let filename = format!("{}_{}", algorithm, type_of_stat_string);

    let mut file = File::create(filename).expect("problem creating file");

    let file_content = final_stats
        .iter()
        .enumerate()
        .map(|(i, (comps, swaps))| match type_of_stat {
            StatType::Comps => format!("{} {}", 100 * (i + 1), comps),
            StatType::Swaps => format!("{} {}", 100 * (i + 1), swaps),
        })
        .collect::<Vec<_>>()
        .join("\n");

    file.write_all(file_content.as_bytes())
        .expect("problem writing to file");
}

enum StatType {
    Comps,
    Swaps,
}

fn calculate_const_in_front_of_n_lg_n(stats: &[(f64, f64)]) -> (f64, f64) {
    assert!(stats.len() >= 10, "stats too short");

    let mut sum_comps_const = 0.0;
    let mut sum_swaps_const = 0.0;

    for i in 0..10 {
        let n = (100 * (i + 1)) as f64;

        let (comps, swaps) = stats[i];

        let n_log_n = n.log2() * n;

        sum_swaps_const += swaps / n_log_n;
        sum_comps_const += comps / n_log_n;
    }

    sum_swaps_const /= 10.0;
    sum_comps_const /= 10.0;

    (sum_swaps_const, sum_comps_const)
}
