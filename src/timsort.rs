//! Timsort algorithm implementation.

use crate::prelude::*;

/// Struct implementing `Sorter` + `BenchmarkingSorter` using timsort
/// to sort items. It uses mergesort with insertion sort for smaller problems.
pub struct TimSort;

impl TimSort {
    fn tim_sort<T: Ord + Copy>(slice: &mut [T]) {
        if slice.len() <= 10 {
            InsertionSort::sort(slice);
            return;
        }

        let slice_mid = slice.len() / 2;

        TimSort::tim_sort(&mut slice[..slice_mid]);
        TimSort::tim_sort(&mut slice[slice_mid..]);
        TimSort::merge(slice, slice_mid);
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

    fn tim_sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        if slice.len() <= 10 {
            InsertionSort::sort_with_benchmark(slice, benchmark);
            return;
        }

        let slice_mid = slice.len() / 2;

        TimSort::tim_sort_with_benchmark(&mut slice[..slice_mid], benchmark);
        TimSort::tim_sort_with_benchmark(&mut slice[slice_mid..], benchmark);
        TimSort::merge_with_benchmark(slice, slice_mid, benchmark);
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

            benchmark.add_swap();
        }
    }
}

impl Sorter for TimSort {
    /// Timsort implementation working on types implementing
    /// `Ord` + `Copy` so it's mostly usefull for primitive types.
    /// It uses mergesort with insertion sort for smaller problems.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    ///
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// TimSort::sort(&mut slice);
    ///
    /// assert_eq!([1, 2, 3, 5, 8, 9], slice);
    /// ```
    fn sort<T: Ord + Copy>(slice: &mut [T]) {
        TimSort::tim_sort(slice);
    }
}

impl BenchmarkingSorter for TimSort {
    /// Timsort implementation with additional benchmarking capabilities.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::benchmarking::StandardBenchmarker;
    ///
    /// let mut benchmarker = StandardBenchmarker::default();
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// TimSort::sort_with_benchmark(&mut slice, &mut benchmarker);
    ///
    /// let stats = benchmarker.get_stats();
    ///
    /// assert_eq!(4, stats.comparisons);
    /// assert_eq!(4, stats.swaps);
    /// ```
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        // benchmark.start_timer();
        // timer won't work for now

        TimSort::tim_sort_with_benchmark(slice, benchmark);

        // benchmark.stop_timer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn quick_sort_basic_sorting_test() {
        basic_sorting_test::<TimSort>();
    }

    #[test]
    fn quick_sort_empty_sorting_test() {
        empty_sorting_test::<TimSort>();
    }

    #[test]
    fn quick_sort_sorted_sorting_test() {
        sorted_sorting_test::<TimSort>();
    }

    #[test]
    fn quick_sort_sorted_backwards_sorting_test() {
        sorted_backwards_sorting_test::<TimSort>();
    }
}
