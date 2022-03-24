pub use crate::benchmarking::Benchmark;

pub trait Sorter {
    fn sort<T: Ord + Copy>(slice: &mut [T]);
}

pub trait BenchmarkingSorter {
    fn sort_with_benchmark<T: Ord + Copy>(slice: &mut [T], benchmark: &mut impl Benchmark);
}
