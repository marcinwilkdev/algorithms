//! Generator trait with few implementations helpful with testing algorithms.

use rand::prelude::*;
use rand_pcg::Pcg64Mcg;

/// Main trait which has to be implemented by every data generator.
pub trait Generator {
    fn generate(n: usize) -> Vec<u32>;
}

/// Generator generating values at random with range from `0` to `2*n - 1`.
pub struct RandomGenerator;

impl Generator for RandomGenerator {
    fn generate(n: usize) -> Vec<u32> {
        (0..n)
            .map(|_| Pcg64Mcg::from_entropy().gen_range(0..2 * n - 1))
            .map(|number| number.try_into().expect("value range too big"))
            .collect()
    }
}

/// Generator generating values in ascending order with step chosen
/// at random from `0` to `10` exclusive.
pub struct AscendingGenerator;

impl Generator for AscendingGenerator {
    fn generate(n: usize) -> Vec<u32> {
        let mut numbers = (0..n)
            .map(|_| Pcg64Mcg::from_entropy().gen_range(0..10))
            .collect::<Vec<_>>();

        for i in 0..numbers.len() - 1 {
            numbers[i + 1] += numbers[i];
        }

        numbers
    }
}

/// Generator generating values in descending order with step chosen
/// at random from `0` to `10` exclusive.
pub struct DescendingGenerator;

impl Generator for DescendingGenerator {
    fn generate(n: usize) -> Vec<u32> {
        let mut numbers = (0..n)
            .map(|_| Pcg64Mcg::from_entropy().gen_range(0..10))
            .map(|number| number.try_into().expect("value range too big"))
            .collect::<Vec<u32>>();

        for i in 0..numbers.len() - 1 {
            numbers[i + 1] += numbers[i];
        }

        numbers.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_generator_works() {
        let values = RandomGenerator::generate(100);

        assert!(*values.iter().max().unwrap() < 2 * 100 - 1);
        assert_eq!(values.len(), 100);
    }

    #[test]
    fn ascending_generator_works() {
        let values = AscendingGenerator::generate(100);

        for i in 0..values.len() - 1 {
            if values[i] > values[i+1] {
                panic!("Array isn't ascending");
            }
        }

        assert_eq!(values.len(), 100);
    }

    #[test]
    fn descending_generator_works() {
        let values = DescendingGenerator::generate(100);

        for i in 0..values.len() - 1 {
            if values[i] < values[i+1] {
                panic!("Array isn't descending");
            }
        }

        assert_eq!(values.len(), 100);
    }
}
