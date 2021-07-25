#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter17::*;

#[bench]
fn bench_float_rsqrt(b: &mut test::Bencher) {
    let test: [f32; 2] = [1f32,0.99830806f32];

    b.iter(|| unsafe {
        for i in 0..1 {
            assert_eq!(float_rsqrt(test[i]),test[i+1]);
        }
    });
}