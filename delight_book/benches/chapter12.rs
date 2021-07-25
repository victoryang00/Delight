#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter12::*;

#[bench]
fn bench_unusual_divbm2(b: &mut test::Bencher) {
    let test: [i64; 123] = [0,1,0, 1,1,1, 1,-1,3, -1,1,3, -1,-1,1,
        0,3,0, 1,3,0, 2,3,0, 3,3,1, 4,3,1, 5,3,1, 6,3,6, 7,3,6, 8,3,6,
        9,3,7, -1,3,3, -2,3,3, -3,3,3, -4,3,2,
        0,-3,0, 1,-3,0, 2,-3,0,
        3,-3,3, 4,-3,3, -1,-3,1, -2,-3,1, -3,-3,1, -4,-3,6, -5,-3,6, -6,-3,6,
        21,4,5, 21,3,27, 21,-4,15, 21,-3,9,
        76,5,19, 77,5,19, 78,5,19, 79,5,19, 80,5,16, 84,5,16, 85,5,17];

    b.iter(|| {
        for i in 0..123/3 {
            assert_eq!(unusual_divbm2(test[i],test[i+1]), 0);
        }
    });
}