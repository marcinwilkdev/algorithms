//! Dual pivot quick sort algorithm implementation.

use crate::prelude::*;

/// Struct implementing `Sorter` + `BenchmarkingSorter` using dual pivot quick sort
/// to sort items.
pub struct DualPivotQuicksort;

impl DualPivotQuicksort {
    fn quick_sort<T: Ord + Copy>(slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }

        let (left_pivot_index, right_pivot_index) = DualPivotQuicksort::partition(slice);

        DualPivotQuicksort::quick_sort(&mut slice[..left_pivot_index]);

        DualPivotQuicksort::quick_sort(&mut slice[left_pivot_index + 1..right_pivot_index]);

        if right_pivot_index < slice.len() - 1 {
            DualPivotQuicksort::quick_sort(&mut slice[right_pivot_index + 1..]);
        }
    }

    fn partition<T: Ord + Copy>(slice: &mut [T]) -> (usize, usize) {
        if slice[0] > slice[slice.len() - 1] {
            slice.swap(0, slice.len() - 1);
        }

        let left_pivot = slice[0];
        let right_pivot = slice[slice.len() - 1];

        let mut next_smaller = 1;
        let mut next_larger = slice.len() - 2;

        let mut smaller_count = 0;
        let mut larger_count = 0;

        let mut curr = 1;

        while curr <= next_larger {
            if larger_count > smaller_count {
                if slice[curr] > right_pivot {
                    while slice[next_larger] > right_pivot && next_larger > curr {
                        next_larger -= 1;
                    }

                    slice.swap(curr, next_larger);

                    if slice[curr] < left_pivot {
                        slice.swap(curr, next_smaller);
                        next_smaller += 1;
                    }

                    next_larger -= 1;

                    larger_count += 1;
                } else if slice[curr] < left_pivot {
                    slice.swap(curr, next_smaller);
                    next_smaller += 1;

                    smaller_count += 1;
                }
            } else {
                if slice[curr] < left_pivot {
                    slice.swap(curr, next_smaller);
                    next_smaller += 1;

                    smaller_count += 1;
                } else if slice[curr] > right_pivot {
                    while slice[next_larger] > right_pivot && next_larger > curr {
                        next_larger -= 1;
                    }

                    slice.swap(curr, next_larger);

                    if slice[curr] < left_pivot {
                        slice.swap(curr, next_smaller);
                        next_smaller += 1;
                    }

                    next_larger -= 1;

                    larger_count += 1;
                }
            }

            curr += 1;
        }

        slice.swap(next_smaller - 1, 0);
        slice.swap(next_larger + 1, slice.len() - 1);

        (next_smaller - 1, next_larger + 1)
    }

    pub fn quick_sort_with_benchmark<T: Ord + Copy>(
        slice: &mut [T],
        benchmark: &mut impl Benchmark,
    ) {
        if slice.len() <= 1 {
            return;
        }

        let (left_pivot_index, right_pivot_index) =
            DualPivotQuicksort::partition_with_benchmark(slice, benchmark);

        DualPivotQuicksort::quick_sort_with_benchmark(&mut slice[..left_pivot_index], benchmark);

        if left_pivot_index + 1 < right_pivot_index {
            DualPivotQuicksort::quick_sort_with_benchmark(
                &mut slice[left_pivot_index + 1..right_pivot_index],
                benchmark,
            );
        }

        if right_pivot_index < slice.len() - 1 {
            DualPivotQuicksort::quick_sort_with_benchmark(
                &mut slice[right_pivot_index + 1..],
                benchmark,
            );
        }
    }

    fn partition_with_benchmark<T: Ord + Copy>(
        slice: &mut [T],
        benchmark: &mut impl Benchmark,
    ) -> (usize, usize) {
        if slice[0] > slice[slice.len() - 1] {
            benchmark.add_swap();
            slice.swap(0, slice.len() - 1);
        }

        benchmark.add_cmp();

        let left_pivot = slice[0];
        let right_pivot = slice[slice.len() - 1];

        benchmark.add_swap(); // for pivots initialize
        benchmark.add_swap();

        let mut next_smaller = 1;
        let mut next_larger = slice.len() - 2;

        let mut smaller_count = 0;
        let mut larger_count = 0;

        let mut curr = 1;

        while curr <= next_larger {
            benchmark.add_cmp();
            if larger_count > smaller_count {
                if slice[curr] > right_pivot {
                    while slice[next_larger] > right_pivot && next_larger > curr {
                        benchmark.add_cmp();
                        next_larger -= 1;
                    }
                    benchmark.add_cmp();

                    slice.swap(curr, next_larger);
                    benchmark.add_swap();

                    if slice[curr] < left_pivot {
                        benchmark.add_swap();
                        slice.swap(curr, next_smaller);
                        next_smaller += 1;
                    }
                    benchmark.add_cmp();

                    next_larger -= 1;

                    larger_count += 1;
                } else if slice[curr] < left_pivot {
                    benchmark.add_cmp();
                    slice.swap(curr, next_smaller);
                    benchmark.add_swap();
                    next_smaller += 1;

                    smaller_count += 1;
                } else {
                    benchmark.add_cmp();
                }
            } else {
                if slice[curr] < left_pivot {
                    slice.swap(curr, next_smaller);
                    benchmark.add_swap();
                    next_smaller += 1;

                    smaller_count += 1;
                } else if slice[curr] > right_pivot {
                    benchmark.add_cmp();
                    while slice[next_larger] > right_pivot && next_larger > curr {
                        benchmark.add_cmp();
                        next_larger -= 1;
                    }
                    benchmark.add_cmp();

                    slice.swap(curr, next_larger);
                    benchmark.add_swap();

                    if slice[curr] < left_pivot {
                        slice.swap(curr, next_smaller);
                        benchmark.add_swap();
                        next_smaller += 1;
                    }
                    benchmark.add_cmp();

                    next_larger -= 1;

                    larger_count += 1;
                } else {
                    benchmark.add_cmp();
                }
            }

            curr += 1;
        }

        slice.swap(next_smaller - 1, 0);
        slice.swap(next_larger + 1, slice.len() - 1);

        benchmark.add_swap();
        benchmark.add_swap();

        (next_smaller - 1, next_larger + 1)
    }
}

impl Sorter for DualPivotQuicksort {
    /// Dual pivot quick sort implementation working on types implementing
    /// `Ord` + `Copy` so it's mostly usefull for primitive types.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    ///
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// DualPivotQuicksort::sort(&mut slice);
    ///
    /// assert_eq!([1, 2, 3, 5, 8, 9], slice);
    /// ```
    fn sort<T: Ord + Copy>(slice: &mut [T]) {
        DualPivotQuicksort::quick_sort(slice);
    }
}

impl BenchmarkingSorter for DualPivotQuicksort {
    /// Dual pivot quick sort implementation with additional benchmarking capabilities.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::benchmarking::StandardBenchmarker;
    ///
    /// let mut benchmarker = StandardBenchmarker::default();
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// DualPivotQuicksort::sort_with_benchmark(&mut slice, &mut benchmarker);
    ///
    /// let stats = benchmarker.get_stats();
    ///
    /// assert_eq!(4, stats.comparisons);
    /// assert_eq!(7, stats.swaps);
    /// ```
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        benchmark.start_timer();

        DualPivotQuicksort::quick_sort_with_benchmark(slice, benchmark);

        benchmark.stop_timer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn dual_pivot_quick_sort_basic_sorting_test() {
        basic_sorting_test::<DualPivotQuicksort>();
    }

    #[test]
    fn dual_pivot_quick_sort_empty_sorting_test() {
        empty_sorting_test::<DualPivotQuicksort>();
    }

    #[test]
    fn dual_pivot_quick_sort_sorted_sorting_test() {
        sorted_sorting_test::<DualPivotQuicksort>();
    }

    #[test]
    fn dual_pivot_quick_sort_sorted_backwards_sorting_test() {
        sorted_backwards_sorting_test::<DualPivotQuicksort>();
    }
}
