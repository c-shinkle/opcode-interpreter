#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use opcode_interpreter::opcode::interpreter::{
        itertools_join, precompute_capacity,
    };
    use test::{Bencher, black_box};

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
}
