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

    let mut insertion_final_stats = Vec::with_capacity(10);
    let mut merge_final_stats = Vec::with_capacity(10);
    let mut quick_final_stats = Vec::with_capacity(10);

    for i in 1..=10 {
        let n = 100 * i;

        let mut insertion_results = Vec::with_capacity(k);
        let mut merge_results = Vec::with_capacity(k);
        let mut quick_results = Vec::with_capacity(k);

        for _ in 0..k {
            let random_array = RandomGenerator::generate(n);

            let mut merge_benchmark = StandardBenchmarker::default();
            let mut quick_benchmark = StandardBenchmarker::default();

            MergeSort::sort_with_benchmark(&mut random_array.clone(), &mut merge_benchmark);
            QuickSort::sort_with_benchmark(&mut random_array.clone(), &mut quick_benchmark);

            merge_results.push(merge_benchmark.get_stats());
            quick_results.push(quick_benchmark.get_stats());

            if n < 500 {
                let mut insertion_benchmark = StandardBenchmarker::default();
                InsertionSort::sort_with_benchmark(
                    &mut random_array.clone(),
                    &mut insertion_benchmark,
                );
                insertion_results.push(insertion_benchmark.get_stats());
            }
        }

        let merge_comps_and_swaps = get_avg_stats(&merge_results, k);
        let quick_comps_and_swaps = get_avg_stats(&quick_results, k);

        merge_final_stats.push(merge_comps_and_swaps);
        quick_final_stats.push(quick_comps_and_swaps);

        if n < 500 {
            let insertion_comps_and_swaps = get_avg_stats(&insertion_results, k);
            insertion_final_stats.push(insertion_comps_and_swaps);
        }
    }

    print_final_stats_to_file("insertion", StatType::Comps, &insertion_final_stats);
    print_final_stats_to_file("insertion", StatType::Swaps, &insertion_final_stats);
    print_final_stats_to_file("merge", StatType::Comps, &merge_final_stats);
    print_final_stats_to_file("merge", StatType::Swaps, &merge_final_stats);
    print_final_stats_to_file("quick", StatType::Comps, &quick_final_stats);
    print_final_stats_to_file("quick", StatType::Swaps, &quick_final_stats);
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
