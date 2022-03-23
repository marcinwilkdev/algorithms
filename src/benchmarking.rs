//! Collection of structs and traits necessary for benchmarking algorithms.

use std::time::{Duration, Instant};

/// Struct representing result of benchmarking. Has to be returned
/// by benchmarker after finished benchmarking.

pub struct Stats {
    pub comparisons: usize,
    pub swaps: usize,
    pub duration: Duration,
}

/// Trait for benchmarkers which are used to benchmark algorithms.

pub trait Benchmark {
    /// Invoke this method where comparison is made in algorithm.
    fn add_cmp(&mut self);
    /// Invoke this method where swap is made in algorithm.
    fn add_swap(&mut self);
    /// Invoke this method at the start of benchmarking.
    fn start_timer(&mut self);
    /// Invoke this method at the end of benchmarking to save processing time.
    fn stop_timer(&mut self);
    /// Invoke this method to get `Stats` struct after all benchmarking is done.
    fn get_stats(&mut self) -> Stats;
}

/// Most basic implementation of `Benchmark` trait. Used
/// in all benchmarking examples.

#[derive(Default)]
pub struct StandardBenchmarker {
    comparisons: usize,
    swaps: usize,
    timer: Option<Instant>,
    duration: Option<Duration>,
}

impl Benchmark for StandardBenchmarker {
    fn add_cmp(&mut self) {
        self.comparisons += 1;
    }

    fn add_swap(&mut self) {
        self.swaps += 1;
    }

    fn start_timer(&mut self) {
        assert!(self.timer.is_none(), "timer already started");

        self.timer = Some(Instant::now());
    }

    fn stop_timer(&mut self) {
        assert!(self.timer.is_some(), "timer has to be started first");

        let now = Instant::now();

        self.duration = Some(now - self.timer.unwrap());
    }

    fn get_stats(&mut self) -> Stats {
        assert!(self.duration.is_some(), "time wasn't measured yet");

        Stats {
            duration: self.duration.take().unwrap(),
            swaps: self.swaps,
            comparisons: self.comparisons,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[test]
    fn standard_benchmarker_works() {
        let mut benchmarker = StandardBenchmarker::default();

        benchmarker.start_timer();

        benchmarker.add_swap();
        benchmarker.add_cmp();
        benchmarker.add_swap();
        benchmarker.add_cmp();
        benchmarker.add_cmp();

        std::thread::sleep_ms(1);

        benchmarker.stop_timer();

        let stats = benchmarker.get_stats();

        assert_eq!(2, stats.swaps);
        assert_eq!(3, stats.comparisons);
        assert!(stats.duration >= Duration::from_millis(1));
    }
}
