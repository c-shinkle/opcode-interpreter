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

pub fn rayon_compute_max_signal(codes_string: &str) -> Result<i32> {
    let codes = parse::imperative(codes_string)?;
    let results: Vec<Result<i32>> = PERMUTATIONS
        .into_par_iter()
        .map(|phase_sequence| amplify(&codes, phase_sequence))
        .collect();
    let mut current = i32::MIN;
    for result in results {
        current = current.max(result?);
    }
    Ok(current)
}

pub fn multi_threaded_compute_max_signal(codes_string: &str) -> Result<i32> {
    const N: usize = 3;
    let arc_codes = Arc::new(parse::imperative(codes_string)?);
    let perm_ranges = divide_ranges(N);
    let mut handles: Vec<JoinHandle<Result<i32>>> = Vec::with_capacity(N);

    for range in perm_ranges {
        let codes = arc_codes.clone();
        let handle = spawn(move || {
            let permutations = &PERMUTATIONS[range];
            let mut results: Vec<Result<i32>> = Vec::with_capacity(PERMUTATIONS.len() / N);
            for perm in permutations {
                results.push(amplify(&codes, *perm));
            }
            let mut current = i32::MIN;
            for result in results {
                current = current.max(result?);
            }
            Ok(current)
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(N);
    for handle in handles {
        // TODO extend custom result to handle failed joins
        results.push(handle.join().unwrap());
    }

    let mut current = i32::MIN;
    for result in results {
        current = current.max(result?);
    }

    Ok(current)
}

fn divide_ranges(n: usize) -> Vec<Range<usize>> {
    let len = PERMUTATIONS.len();
    let mut ranges = Vec::with_capacity(n);
    for i in 0..n {
        ranges.push(Range {
            start: len / n * i,
            end: len / n * (i + 1),
        });
    }
    ranges
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
        let actual = divide_ranges(2);
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
        let actual = divide_ranges(4);
        assert_eq!(actual, expected);
    }
}
