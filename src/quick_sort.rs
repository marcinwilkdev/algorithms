//! Quick sort algorithm implementation.

use crate::prelude::*;

/// Struct implementing `Sorter` + `BenchmarkingSorter` using quick sort
/// to sort items.
pub struct QuickSort;

impl QuickSort {
    fn quick_sort<T: Ord + Copy>(slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }

        let pivot_index = QuickSort::partition(slice);

        QuickSort::quick_sort(&mut slice[..pivot_index]);

        if pivot_index < slice.len() - 1 {
            QuickSort::quick_sort(&mut slice[pivot_index + 1..]);
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
}

impl Sorter for QuickSort {
    /// Quick sort implementation working on types implementing
    /// `Ord` + `Copy` so it's mostly usefull for primitive types.
    ///
    /// Examples:
    /// ```
    /// use algorithms::prelude::*;
    /// use algorithms::quick_sort::QuickSort;
    ///
    /// let mut slice = [1, 8, 2, 3, 9, 5];
    ///
    /// QuickSort::sort(&mut slice);
    ///
    /// assert_eq!([1, 2, 3, 5, 8, 9], slice);
    /// ```
    fn sort<T: Ord + Copy>(slice: &mut [T]) {
        QuickSort::quick_sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn quick_sort_basic_sorting_test() {
        basic_sorting_test::<QuickSort>();
    }

    #[test]
    fn quick_sort_empty_sorting_test() {
        empty_sorting_test::<QuickSort>();
    }

    #[test]
    fn quick_sort_sorted_sorting_test() {
        sorted_sorting_test::<QuickSort>();
    }

    #[test]
    fn quick_sort_sorted_backwards_sorting_test() {
        sorted_backwards_sorting_test::<QuickSort>();
    }
}
