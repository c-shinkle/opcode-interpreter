use itertools::Itertools;
use std::fmt::Write;

#[inline(always)]
pub fn itertools_join(codes: &[i32]) -> String {
    codes.iter().join(",")
}

pub fn precompute_capacity(codes: &[i32]) -> String {
    let mut chars = String::with_capacity(calc_capacity(codes));
    let mut codes_iter = codes.iter();
    write!(&mut chars, "{}", codes_iter.next().unwrap()).unwrap();
    for code in codes_iter {
        chars.push(',');
        write!(&mut chars, "{code}").unwrap();
    }
    chars
}

fn calc_capacity(codes: &[i32]) -> usize {
    codes.iter().copied().fold(0, |acc: usize, mut val: i32| {
        let is_negative = val.is_negative();
        if is_negative {
            val *= -1;
        }
        acc + (is_negative as usize + (val.checked_ilog10().unwrap_or(0) + 1) as usize)
    })
}

#[cfg(test)]
mod tests {
    use super::calc_capacity;

    #[test]
    fn calc_capacity_1_2_3() {
        let actual = calc_capacity(&[1, 2, 3]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn calc_capacity_10_20_30() {
        let actual = calc_capacity(&[10, 20, 30]);
        assert_eq!(actual, 6);
    }

    #[test]
    fn calc_capacity_neg_1_2_3() {
        let actual = calc_capacity(&[-1, -2, -3]);
        assert_eq!(actual, 6);
    }

    #[test]
    fn calc_capacity_neg_1_0_1() {
        let actual = calc_capacity(&[-1, 0, 1]);
        assert_eq!(actual, 4);
    }
}
