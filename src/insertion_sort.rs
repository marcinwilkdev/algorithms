//! Insertion sort algorithm implementation.

use crate::benchmarking::Benchmark;

/// Insertion sort implementation working on types implementing
/// `Ord` + `Copy` so it's mostly usefull for primitive types.
///
/// Examples:
/// ```
/// use algorithms::insertion_sort;
///
/// let mut slice = [1, 8, 2, 3, 9, 5];
///
/// insertion_sort::insertion_sort(&mut slice);
///
/// assert_eq!([1, 2, 3, 5, 8, 9], slice);
/// ```

pub fn insertion_sort<T>(slice: &mut [T])
where
    T: Ord + Copy,
{
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

/// Insertion sort implementation with additional benchmarking capabilities.
///
/// Examples:
/// ```
/// use algorithms::insertion_sort;
/// use algorithms::benchmarking::{Benchmark, StandardBenchmarker};
///
/// let mut benchmarker = StandardBenchmarker::default();
/// let mut slice = [1, 8, 2, 3, 9, 5];
///
/// insertion_sort::insertion_sort_with_benchmark(&mut slice, &mut benchmarker);
///
/// let stats = benchmarker.get_stats();
///
/// assert_eq!(4, stats.comparisons);
/// assert_eq!(4, stats.swaps);
/// ```

pub fn insertion_sort_with_benchmark<T>(slice: &mut [T], benchmark: &mut impl Benchmark)
where
    T: Ord + Copy,
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_zero_size_works() {
        let mut slice: [(); 0] = [];

        insertion_sort(&mut slice);

        let expected: [(); 0] = [];

        assert_eq!(expected, slice);
    }
}
