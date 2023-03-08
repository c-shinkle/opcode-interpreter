#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use opcode_interpreter::opcode::amplifier::{
        multi_threaded_compute_max_signal, single_threaded_compute_max_signal,
    };
    use opcode_interpreter::opcode::interpreter::interpret;
    use opcode_interpreter::opcode::parse::{functional_parse, imperative};
    use opcode_interpreter::opcode::stringify::{itertools_join, precompute_capacity};
    use std::fs;
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

    #[bench]
    fn day_5_part_2(b: &mut Bencher) {
        let codes_string = fs::read_to_string("res/day_5").unwrap();
        let original = imperative(&codes_string).unwrap();
        b.iter(|| {
            let mut codes = original.clone();
            let _ = black_box(
                interpret(&mut codes, vec![5], &mut None).expect("Should return 12410607"),
            );
        });
    }

    #[bench]
    fn single_thread_max_signal(b: &mut Bencher) {
        let codes_string = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        b.iter(|| {
            let _ = black_box(
                single_threaded_compute_max_signal(codes_string).expect("Should return 65210"),
            );
        });
    }

    #[bench]
    fn two_thread_max_signal(b: &mut Bencher) {
        let codes_string = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        b.iter(|| {
            let _ = black_box(
                multi_threaded_compute_max_signal(codes_string).expect("Should return 65210"),
            );
        });
    }
}
