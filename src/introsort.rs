//! Introsort sort algorithm implementation, but instead of heap sort
//! it uses insertion sort for smaller problems.

use crate::prelude::*;

/// Struct implementing `Sorter` + `BenchmarkingSorter` using introsort
/// to sort items. It uses quicksort with insertion sort for smaller problems.
pub struct IntroSort;

impl IntroSort {
    fn intro_sort<T: Ord + Copy>(slice: &mut [T], m: usize) {
        if slice.len() <= 1 {
            return;
        }

        if m == 0 {
            InsertionSort::sort(slice);
            return;
        }

        let pivot_index = IntroSort::partition(slice);

        IntroSort::intro_sort(&mut slice[..pivot_index], m - 1);

        if pivot_index < slice.len() - 1 {
            IntroSort::intro_sort(&mut slice[pivot_index + 1..], m - 1);
        }
    }

    fn partition<T: Ord + Copy>(slice: &mut [T]) -> usize {
        let pivot = slice[slice.len() - 1];

        let mut l = 0;

        for r in 0..slice.len() - 1 {
            if slice[r] < pivot {
                slice.swap(l, r);
                l += 1;
            }
        }

        slice.swap(l, slice.len() - 1);

        l
    }

    fn intro_sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], m: usize, benchmark: &mut impl Benchmark) {
        if slice.len() <= 1 {
            return;
        }

        if m == 0 {
            InsertionSort::sort_with_benchmark(slice, benchmark);
            return;
        }

        let pivot_index = IntroSort::partition_with_benchmark(slice, benchmark);

        IntroSort::intro_sort_with_benchmark(&mut slice[..pivot_index], m - 1, benchmark);

        if pivot_index < slice.len() - 1 {
            IntroSort::intro_sort_with_benchmark(&mut slice[pivot_index + 1..], m - 1, benchmark);
        }
    }

    fn partition_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) -> usize {
        let pivot = slice[slice.len() - 1];

        let mut l = 0;

        for r in 0..slice.len() - 1 {
            benchmark.add_cmp();
            if slice[r] < pivot {
                benchmark.add_swap();
                slice.swap(l, r);
                l += 1;
            }
        }

        benchmark.add_swap();
        slice.swap(l, slice.len() - 1);

        l
    }
}

impl Sorter for IntroSort {
    /// Introsort implementation working on types implementing
    /// `Ord` + `Copy` so it's mostly usefull for primitive types.
    /// It uses quicksort with insertion sort for smaller problems.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    ///
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// IntroSort::sort(&mut slice);
    ///
    /// assert_eq!([1, 2, 3, 5, 8, 9], slice);
    /// ```
    fn sort<T: Ord + Copy>(slice: &mut [T]) {
        let len = slice.len() as f64;

        let m = (2.0 * len.log2()) as usize;

        IntroSort::intro_sort(slice, m);
    }
}

impl BenchmarkingSorter for IntroSort {
    /// Introsort implementation with additional benchmarking capabilities.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::benchmarking::StandardBenchmarker;
    ///
    /// let mut benchmarker = StandardBenchmarker::default();
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// IntroSort::sort_with_benchmark(&mut slice, &mut benchmarker);
    ///
    /// let stats = benchmarker.get_stats();
    ///
    /// assert_eq!(9, stats.comparisons);
    /// assert_eq!(10, stats.swaps);
    /// ```
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        // benchmark.start_timer();
        // timer won't work for now

        let len = slice.len() as f64;

        let m = (2.0 * len.log2()) as usize;

        IntroSort::intro_sort_with_benchmark(slice, m, benchmark);

        // benchmark.stop_timer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn quick_sort_basic_sorting_test() {
        basic_sorting_test::<IntroSort>();
    }

    #[test]
    fn quick_sort_empty_sorting_test() {
        empty_sorting_test::<IntroSort>();
    }

    #[test]
    fn quick_sort_sorted_sorting_test() {
        sorted_sorting_test::<IntroSort>();
    }

    #[test]
    fn quick_sort_sorted_backwards_sorting_test() {
        sorted_backwards_sorting_test::<IntroSort>();
    }
}
