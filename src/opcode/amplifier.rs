use super::errors::OpcodeError::MissingOutput;
use super::errors::Result;
use super::interpreter::interpret;
use super::parse;
use super::permutations::PERMUTATIONS;

#[allow(dead_code)]
fn compute_max_signal(codes_string: &str) -> Result<i32> {
    let original = parse::imperative(codes_string)?;

    let mut max = i32::MIN;
    for perm in PERMUTATIONS {
        max = max.max(amplify(&original, &perm)?);
    }
    Ok(max)

    // PERMUTATIONS
    //     .iter()
    //     .map(|perm| amplify(&original, perm))
    //     .try_fold(i32::MIN, |acc, perm| Ok(acc.max(perm?)))
}

fn amplify(codes: &[i32], phase_sequence: &[i32; 5]) -> Result<i32> {
    let mut signal = 0;
    let mut output: Option<i32> = None;

    for phase in phase_sequence {
        let input = vec![signal, *phase];
        interpret(&mut codes.to_owned(), input, &mut output)?;
        signal = output.ok_or(MissingOutput)?;
    }

    Ok(signal)
}
