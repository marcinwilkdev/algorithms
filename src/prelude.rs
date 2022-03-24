//! Convenience re-export of common members and declaration of
//! sorting traits.

pub use crate::benchmarking::Benchmark;
pub use crate::insertion_sort::InsertionSort;
pub use crate::merge_sort::MergeSort;
pub use crate::quick_sort::QuickSort;

/// Trait for sorting algorithms.

pub trait Sorter {
    fn sort<T: Ord + Copy>(slice: &mut [T]);
}

/// Trait for sorting algorithms with benchmarking capabilities.

pub trait BenchmarkingSorter {
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark);
}
