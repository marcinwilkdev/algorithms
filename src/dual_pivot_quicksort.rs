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
        let mut left_pivot = slice[0];
        let mut right_pivot = slice[slice.len() - 1];

        let mut small_count = 0;
        let mut large_count = 0;

        if left_pivot > right_pivot {
            slice.swap(0, slice.len() - 1);

            left_pivot = slice[0];
            right_pivot = slice[slice.len() - 1];
        }

        let mut first_part_index = 0;
        let mut second_part_index = slice.len() - 1;

        let mut i = 1;

        while i < second_part_index {
            if large_count > small_count {
                if slice[i] > right_pivot {
                    second_part_index -= 1;

                    while slice[second_part_index] > right_pivot {
                        second_part_index -= 1;
                    }

                    slice.swap(second_part_index, i);

                    large_count += 1;
                } else if slice[i] < left_pivot {
                    first_part_index += 1;
                    slice.swap(first_part_index, i);

                    small_count += 1;
                }
            } else {
                if slice[i] < left_pivot {
                    first_part_index += 1;
                    slice.swap(first_part_index, i);

                    small_count += 1;
                } else if slice[i] > right_pivot {
                    second_part_index -= 1;

                    while slice[second_part_index] > right_pivot {
                        second_part_index -= 1;
                    }

                    slice.swap(second_part_index, i);

                    large_count += 1;
                }
            }

            i += 1;
        }

        slice.swap(first_part_index, 0);
        slice.swap(second_part_index, slice.len() - 1);

        (first_part_index, second_part_index)
    }

    fn quick_sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark) {
        if slice.len() <= 1 {
            return;
        }

        let (left_pivot_index, right_pivot_index) =
            DualPivotQuicksort::partition_with_benchmark(slice, benchmark);

        DualPivotQuicksort::quick_sort_with_benchmark(&mut slice[..left_pivot_index], benchmark);

        DualPivotQuicksort::quick_sort_with_benchmark(
            &mut slice[left_pivot_index + 1..right_pivot_index],
            benchmark,
        );

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
        let mut left_pivot = slice[0];
        let mut right_pivot = slice[slice.len() - 1];

        let mut small_count = 0;
        let mut large_count = 0;

        if left_pivot > right_pivot {
            slice.swap(0, slice.len() - 1);

            left_pivot = slice[0];
            right_pivot = slice[slice.len() - 1];
        }

        let mut first_part_index = 0;
        let mut second_part_index = slice.len() - 1;

        let mut i = 1;

        while i < second_part_index {
            if large_count > small_count {
                benchmark.add_cmp();
                if slice[i] > right_pivot {
                    second_part_index -= 1;

                    while slice[second_part_index] > right_pivot {
                        second_part_index -= 1;

                        benchmark.add_cmp();
                    }
                    benchmark.add_cmp();

                    slice.swap(second_part_index, i);
                    benchmark.add_swap();

                    large_count += 1;
                } else if slice[i] < left_pivot {
                    first_part_index += 1;
                    slice.swap(first_part_index, i);
                    benchmark.add_swap();

                    small_count += 1;

                    benchmark.add_cmp();
                } else {
                    benchmark.add_cmp();
                }
            } else {
                benchmark.add_cmp();
                if slice[i] < left_pivot {
                    first_part_index += 1;
                    slice.swap(first_part_index, i);
                    benchmark.add_swap();

                    small_count += 1;
                } else if slice[i] > right_pivot {
                    second_part_index -= 1;

                    while slice[second_part_index] > right_pivot {
                        second_part_index -= 1;
                        benchmark.add_cmp();
                    }

                    benchmark.add_cmp();

                    slice.swap(second_part_index, i);
                    benchmark.add_swap();

                    large_count += 1;

                    benchmark.add_cmp();
                } else {
                    benchmark.add_cmp();
                }
            }

            i += 1;
        }

        benchmark.add_swap();
        benchmark.add_swap();
        slice.swap(first_part_index, 0);
        slice.swap(second_part_index, slice.len() - 1);

        (first_part_index, second_part_index)
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
    /// assert_eq!(6, stats.comparisons);
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
