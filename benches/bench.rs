#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use opcode_interpreter::opcode::parse::{functional_parse, imperative};
    use opcode_interpreter::opcode::stringify::{itertools_join, precompute_capacity};
    use test::{black_box, Bencher};

    #[bench]
    fn itertools_join_10000(b: &mut Bencher) {
        let codes = (0..10000).collect::<Vec<i32>>();
        b.iter(|| {
            black_box(itertools_join(&codes));
        });
    }

    #[bench]
    fn precompute_capacity_10000(b: &mut Bencher) {
        let codes = (0..10000).collect::<Vec<i32>>();
        b.iter(|| {
            black_box(precompute_capacity(&codes));
        });
    }

    #[bench]
    fn imperative_parse_10000(b: &mut Bencher) {
        let mut codes_string = precompute_capacity(&(0..10000).collect::<Vec<i32>>());
        codes_string.insert_str(24444, ",x");
        b.iter(|| {
            let _ = black_box(imperative(&codes_string));
        });
    }

    #[bench]
    fn functional_parse_10000(b: &mut Bencher) {
        let mut codes_string = precompute_capacity(&(0..10000).collect::<Vec<i32>>());
        codes_string.insert_str(24444, ",x");
        b.iter(|| {
            let _ = black_box(functional_parse(&codes_string));
        });
    }
}
