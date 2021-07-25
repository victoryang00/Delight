#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter3::*;

#[bench]
#[allow(overflowing_literals)]
fn bench_crc_clp2(b: &mut test::Bencher) {
    let TEST: Vec<i32> = vec![0, 0, 1, 1, 2, 2, 3, 4, 4, 4, 5, 8, 7, 8, 8, 8,
                              9, 16, 15, 16, 16, 16, 0xffff, 0x10000, 0x7fffffff, 0x80000000,
                              0x80000000, 0x80000000, 0x80000001, 0,
                              0xffffffff, 0];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(crc_clp2(TEST[2 * i] ), TEST[2 * i + 1]);
        }
    })
}