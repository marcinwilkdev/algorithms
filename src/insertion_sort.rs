//! Insertion sort algorithm implementation.

use crate::prelude::*;

/// Struct implementing `Sorter` + `BenchmarkingSorter` using insertion sort
/// to sort items.
pub struct InsertionSort;

impl Sorter for InsertionSort {
    /// Insertion sort implementation working on types implementing
    /// `Ord` + `Copy` so it's mostly usefull for primitive types.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::insertion_sort::InsertionSort;
    ///
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// InsertionSort::sort(&mut slice);
    ///
    /// assert_eq!([1, 2, 3, 5, 8, 9], slice);
    /// ```
    fn sort<T: Ord + Copy>(slice: &mut [T]) {
        for i in 0..slice.len() {
            let mut j = i;
            let elem = slice[i];

            while j > 0 && slice[j - 1] > elem {
                slice[j] = slice[j - 1];
                j -= 1;
            }

            slice[j] = elem;
        }
    }
}

impl BenchmarkingSorter for InsertionSort {
    /// Insertion sort implementation with additional benchmarking capabilities.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::insertion_sort::InsertionSort;
    /// use algorithms::benchmarking::StandardBenchmarker;
    ///
    /// let mut benchmarker = StandardBenchmarker::default();
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// InsertionSort::sort_with_benchmark(&mut slice, &mut benchmarker);
    ///
    /// let stats = benchmarker.get_stats();
    ///
    /// assert_eq!(4, stats.comparisons);
    /// assert_eq!(4, stats.swaps);
    /// ```
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        benchmark.start_timer();

        for i in 0..slice.len() {
            let mut j = i;
            let elem = slice[i];

            while j > 0 && slice[j - 1] > elem {
                slice[j] = slice[j - 1];
                j -= 1;

                benchmark.add_cmp();
                benchmark.add_swap();
            }

            slice[j] = elem;
        }

        benchmark.stop_timer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn insertion_sort_basic_sorting_test() {
        basic_sorting_test::<InsertionSort>();
    }

    #[test]
    fn insertion_sort_empty_sorting_test() {
        empty_sorting_test::<InsertionSort>();
    }

    #[test]
    fn insertion_sort_sorted_sorting_test() {
        sorted_sorting_test::<InsertionSort>();
    }

    #[test]
    fn insertion_sort_sorted_backwards_sorting_test() {
        sorted_backwards_sorting_test::<InsertionSort>();
    }
}
