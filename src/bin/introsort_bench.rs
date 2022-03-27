use std::fs::File;
use std::io::Write;

use algorithms::benchmarking::*;
use algorithms::generator::*;
use algorithms::prelude::*;

fn main() {
    // let introsort_param = std::env::args()
    //     .nth(1)
    //     .expect("there has to be a k given")
    //     .parse()
    //     .expect("k isn't valid number");

    let mut merge_final_stats = Vec::with_capacity(10);
    let mut timsort_final_stats = Vec::with_capacity(10);

    for i in 1..=10 {
        let n = 1000 * i;

        let mut merge_results = Vec::with_capacity(10);
        let mut timsort_results = Vec::with_capacity(10);

        for _ in 0..10 {
            let random_array = RandomGenerator::generate(n);

            let mut merge_benchmark = StandardBenchmarker::default();
            let mut timsort_benchmark = StandardBenchmarker::default();

            MergeSort::sort_with_benchmark(&mut random_array.clone(), &mut merge_benchmark);
            TimSort::sort_with_benchmark(&mut random_array.clone(), &mut timsort_benchmark);

            merge_results.push(merge_benchmark.get_stats());
            timsort_results.push(timsort_benchmark.get_stats());
        }

        let merge_comps_and_swaps = get_avg_stats(&merge_results, 10);
        let timsort_comps_and_swaps = get_avg_stats(&timsort_results, 10);

        merge_final_stats.push(merge_comps_and_swaps);
        timsort_final_stats.push(timsort_comps_and_swaps);
    }

    print_final_stats_to_file("merge", StatType::Comps, &merge_final_stats);
    print_final_stats_to_file("merge", StatType::Swaps, &merge_final_stats);
    print_final_stats_to_file("timsort", StatType::Comps, &timsort_final_stats);
    print_final_stats_to_file("timsort", StatType::Swaps, &timsort_final_stats);
}

fn get_avg_stats(stats: &[Stats], k: usize) -> (f64, f64) {
    let (mut comps, mut swaps) = stats
        .iter()
        .map(
            |Stats {
                 comparisons, swaps, ..
             }| (*comparisons, *swaps),
        )
        .fold((0.0, 0.0), |(total_comps, total_swaps), (comps, swaps)| {
            (total_comps + comps as f64, total_swaps + swaps as f64)
        });

    comps /= k as f64;
    swaps /= k as f64;

    (comps, swaps)
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
            StatType::Comps => format!("{} {}", 1000 * (i + 1), comps),
            StatType::Swaps => format!("{} {}", 1000 * (i + 1), swaps),
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
