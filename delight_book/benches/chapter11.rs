#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter11::*;

#[bench]
fn bench_elementary(b: &mut test::Bencher) {
    let test = [0, 0, 1, 1, 2, 1, 3, 1, 4, 2, 5, 2,
        6, 2, 7, 2, 8, 2, 9, 3, 10, 3, 11, 3, 12, 3, 13, 3, 14, 3,
        15, 3, 16, 4, 17, 4, 18, 4, 19, 4, 20, 4, 21, 4, 22, 4, 23, 4,
        24, 4, 25, 5, 26, 5, 27, 5, 28, 5, 29, 5, 30, 5, 31, 5,
        32, 5, 33, 5, 34, 5, 35, 5, 36, 6, 37, 6, 38, 6, 39, 6, 40, 6,
        99, 9, 100, 10, 101, 10, 289, 17, 65535, 255, 65536, 256,
        65537, 256, 1073741823, 32767, 1073741824, 32768,
        1073741825, 32768, 0x80000000, 46340, 0xFFFFFFFF, 65535];

    b.iter(|| {
        for i in (0..test.len()/2).step_by (2){
            assert_eq!(elementary_isqrt1(test[i]),test[i+1]);
            assert_eq!(elementary_isqrt2(test[i]),test[i+1]);
            assert_eq!(elementary_isqrt3(test[i]),test[i+1]);
            assert_eq!(elementary_isqrt4(test[i]),test[i+1]);
            assert_eq!(elementary_isqrt5(test[i]),test[i+1]);
        }
    })
}