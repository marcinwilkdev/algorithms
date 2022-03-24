//! Merge sort algorithm implementation.

use crate::benchmarking::Benchmark;

/// Merge sort implementation working on types implementing
/// `Ord` + `Copy` so it's mostly usefull for primitive types.
///
/// Examples:
/// ```
/// use algorithms::merge_sort;
///
/// let mut slice = [1, 8, 2, 3, 9, 5];
///
/// merge_sort::merge_sort(&mut slice);
///
/// assert_eq!([1, 2, 3, 5, 8, 9], slice);
/// ```

pub fn merge_sort<T>(slice: &mut [T])
where
    T: Ord + Copy,
{
    if slice.len() <= 1 {
        return;
    }

    let slice_mid = slice.len() / 2;

    merge_sort(&mut slice[..slice_mid]);
    merge_sort(&mut slice[slice_mid..]);
    merge(slice, slice_mid);
}

fn merge<T>(slice: &mut [T], slice_mid: usize)
where
    T: Ord + Copy,
{
    let first_slice = slice[..slice_mid].to_vec();
    let second_slice = slice[slice_mid..].to_vec();

    let mut first_slice_index = 0;
    let mut second_slice_index = 0;

    for i in 0..slice.len() {
        if first_slice_index < first_slice.len() {
            if second_slice_index < second_slice.len() {
                if first_slice[first_slice_index] <= second_slice[second_slice_index] {
                    slice[i] = first_slice[first_slice_index];
                    first_slice_index += 1;
                } else {
                    slice[i] = second_slice[second_slice_index];
                    second_slice_index += 1;
                }
            } else {
                slice[i] = first_slice[first_slice_index];
                first_slice_index += 1;
            }
        } else {
            slice[i] = second_slice[second_slice_index];
            second_slice_index += 1;
        }
    }
}

/// Merge sort implementation with additional benchmarking capabilities.
/// You have to start, and stop timer outside of function call because
/// of its recursive structure.
///
/// Examples:
/// ```
/// use algorithms::merge_sort;
/// use algorithms::benchmarking::{Benchmark, StandardBenchmarker};
///
/// let mut benchmarker = StandardBenchmarker::default();
/// let mut slice = [1, 8, 2, 3, 9, 5];
///
/// benchmarker.start_timer();
///
/// merge_sort::merge_sort_with_benchmark(&mut slice, &mut benchmarker);
///
/// benchmarker.stop_timer();
///
/// let stats = benchmarker.get_stats();
///
/// assert_eq!(9, stats.comparisons);
/// assert_eq!(0, stats.swaps);
/// ```

pub fn merge_sort_with_benchmark<T>(slice: &mut [T], benchmark: &mut impl Benchmark)
where
    T: Ord + Copy,
{
    if slice.len() <= 1 {
        return;
    }

    let slice_mid = slice.len() / 2;

    merge_sort_with_benchmark(&mut slice[..slice_mid], benchmark);
    merge_sort_with_benchmark(&mut slice[slice_mid..], benchmark);
    merge_with_benchmark(slice, slice_mid, benchmark);
}

fn merge_with_benchmark<T>(slice: &mut [T], slice_mid: usize, benchmark: &mut impl Benchmark)
where
    T: Ord + Copy,
{
    let first_slice = slice[..slice_mid].to_vec();
    let second_slice = slice[slice_mid..].to_vec();

    let mut first_slice_index = 0;
    let mut second_slice_index = 0;

    for i in 0..slice.len() {
        if first_slice_index < first_slice.len() {
            if second_slice_index < second_slice.len() {
                benchmark.add_cmp();

                if first_slice[first_slice_index] <= second_slice[second_slice_index] {
                    slice[i] = first_slice[first_slice_index];
                    first_slice_index += 1;
                } else {
                    slice[i] = second_slice[second_slice_index];
                    second_slice_index += 1;
                }
            } else {
                slice[i] = first_slice[first_slice_index];
                first_slice_index += 1;
            }
        } else {
            slice[i] = second_slice[second_slice_index];
            second_slice_index += 1;
        }
    }
}
