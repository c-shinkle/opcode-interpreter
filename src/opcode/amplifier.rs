use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use super::errors::OpcodeError::MissingOutput;
use super::errors::Result;
use super::interpreter::interpret;
use super::parse;
use super::permutations::PERMUTATIONS;

use std::ops::Range;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn single_threaded_compute_max_signal(codes_string: &str) -> Result<i32> {
    let codes = parse::imperative(codes_string)?;
    let mut current = i32::MIN;
    for perm in PERMUTATIONS {
        current = current.max(amplify(&codes, perm)?);
    }
    Ok(current)
}

pub fn multi_threaded_compute_max_signal(codes_string: &str) -> Result<i32> {
    let arc_codes = Arc::new(parse::imperative(codes_string)?);

    const N: usize = 3;
    let perm_ranges: [Range<usize>; N] = divide_ranges();
    let mut handles: Vec<JoinHandle<Result<i32>>> = Vec::with_capacity(N);

    for range in perm_ranges {
        let codes = arc_codes.clone();
        let handle = spawn(move || {
            let mut current = i32::MIN;
            let permutations: [[i32; 5]; PERMUTATIONS.len() / N] =
                PERMUTATIONS[range].try_into().unwrap();
            for perm in permutations.into_iter() {
                current = current.max(amplify(&codes, perm)?);
            }
            Ok(current)
        });
        handles.push(handle);
    }

    let mut current = i32::MIN;
    for handle in handles {
        current = current.max(handle.join().unwrap()?);
    }

    Ok(current)
}

fn divide_ranges<const N: usize>() -> [Range<usize>; N] {
    let len = PERMUTATIONS.len();
    let mut ranges = Vec::with_capacity(N);
    for i in 0..N {
        ranges.push(Range {
            start: len / N * i,
            end: len / N * (i + 1),
        });
    }
    ranges.try_into().unwrap()
}

pub fn rayon_compute_max_signal(codes_string: &str) -> Result<i32> {
    let codes = parse::imperative(codes_string)?;

    PERMUTATIONS
        .into_par_iter()
        .map(|permutation| amplify(&codes, permutation))
        .try_fold(|| i32::MIN, |acc, signal| Ok(acc.max(signal?)))
        .try_reduce(|| 0, |a, b| Ok(a.max(b)))
}

fn amplify(codes: &[i32], phase_sequence: [i32; 5]) -> Result<i32> {
    let mut signal = 0;
    let mut output = None;
    for phase in phase_sequence {
        interpret(&mut codes.to_owned(), vec![signal, phase], &mut output)?;
        signal = output.ok_or(MissingOutput)?;
    }
    Ok(signal)
}

#[cfg(test)]
mod tests {
    use crate::opcode::permutations::PERMUTATIONS;

    use super::{divide_ranges, single_threaded_compute_max_signal};

    #[test]
    fn day_7_part_a() {
        let codes_string = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

        let actual = single_threaded_compute_max_signal(codes_string);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 43210);
    }

    #[test]
    fn day_7_part_b() {
        let codes_string =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";

        let actual = single_threaded_compute_max_signal(codes_string);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 54321);
    }

    #[test]
    fn day_7_part_c() {
        let codes_string = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

        let actual = single_threaded_compute_max_signal(codes_string);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 65210);
    }

    #[test]
    fn ranges_2() {
        let len = PERMUTATIONS.len();
        let expected = [0..len / 2, len / 2..len];
        let actual = divide_ranges();
        assert_eq!(actual, expected);
    }

    #[test]
    fn ranges_4() {
        let len = PERMUTATIONS.len();
        let expected = [
            0..len / 4,
            len / 4..len / 2,
            len / 2..3 * len / 4,
            3 * len / 4..len,
        ];
        let actual = divide_ranges();
        assert_eq!(actual, expected);
    }
}
