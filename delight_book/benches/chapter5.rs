#![feature(test)]

extern crate test;
extern crate delight_book;

use delight_book::chapter5::*;
use delight_book::*;
use std::mem::transmute;

/// https://blog.knoldus.com/safe-way-to-access-private-fields-in-rust/

/// mod delight_book{
///     #[derive(Default)]
///     pub struct c8{
///     }
///     impl c8 {
///         pub fn new(value:c8) -> c8{
///             assert!(value<=c8::MAX.0 && value >=c8::MIN.0);
///             c8(value)
///         }
///     }
/// }
///
/// struct local_c8{}
#[bench]
fn bench_counts(b: &mut test::Bencher) {
    b.iter(|| {
        for i in 0..100 {
            assert_eq!(counts_divide_and_conquer(1), 1);
        }
    })
}

#[bench]
fn bench_counts_pop(b: &mut test::Bencher) {
    b.iter(|| {
        for i in 0..100 {
            assert_eq!(counts_pop(1), 1);
        }
    })
}

#[bench]
fn bench_counts_pop_array(b: &mut test::Bencher) {
    b.iter(|| {
        for i in 0..100 {
            assert_eq!(counts_pop_array(vec![1, 2], 1), 1);
        }
    })
}

#[bench]
fn bench_counts_pop_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop1_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop1(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop2_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop2(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop3_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop3(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop4_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop4(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop5_hard(b: &mut test::Bencher) {
    let TEST = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                    8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                    0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                    0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                    0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                    0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop5(TEST[2 * i]), TEST[2 * i + 1] as i32)
        }
    });
}

#[bench]
fn bench_counts_pop5a_hard(b: &mut test::Bencher) {
    let TEST = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                    8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                    0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                    0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                    0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                    0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop5a(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop6_hard(b: &mut test::Bencher) {
    let TEST: Vec<i64> = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                              8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                              0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7, 0xff, 8,
                              0x4000, 1, 0x4001, 2, 0x7000, 3, 0x7fff, 15,
                              0x55555555, 16, 0xAAAAAAAA, 16, 0xFF000000, 8, 0xC0C0C0C0, 8,
                              0x0FFFFFF0, 24, 0x80000000, 1, 0xFFFFFFFF, 32];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop6(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop7_hard(b: &mut test::Bencher) {
    let TEST = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                    8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                    0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop7(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_counts_pop8_hard(b: &mut test::Bencher) {
    let TEST = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                    8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                    0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop8(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop9_hard(b: &mut test::Bencher) {
    let TEST = vec![0, 0, 1, 1, 2, 1, 3, 2, 4, 1, 5, 2, 6, 2, 7, 3,
                    8, 1, 9, 2, 10, 2, 11, 3, 12, 2, 13, 3, 14, 3, 15, 4, 16, 1, 17, 2,
                    0x3F, 6, 0x40, 1, 0x41, 2, 0x7f, 7, 0x80, 1, 0x81, 2, 0xfe, 7];
    b.iter(|| {
        let n = TEST.len() / 2;
        for i in 0..n {
            assert_eq!(counts_pop9(TEST[2 * i]), TEST[2 * i + 1])
        }
    });
}

#[bench]
fn bench_counts_pop_array_hard(b: &mut test::Bencher) {
    let n = 10000;
    let mut A: Vec<i64> = Vec::with_capacity(n);
    for i in 0..n { A.push(0xffffffff); }

    let mut s1 = 0;
    for i in 0..n {
        s1 = s1 + counts_pop(A[i]);
    }

    let s2 = counts_pop_array(A, n as i64);
    assert_eq!(s1, s2);
}

#[bench]
#[allow(overflowing_literals)]
fn bench_counts_popDiff(b: &mut test::Bencher) {
    let TEST = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0x3F, 0x40, 0x41, 0x7f, 0x80, 0x81, 0xfe, 0xff,
                    0x4000, 0x4001, 0x7000, 0x7fff, 0x55555555, 0xAAAAAAAA, 0xFF000000, 0xC0C0C0C0, 0x0FFFFFF0, 0x80000000, 0xFFFFFFFE, 0xFFFFFFFF];
    b.iter(|| {
        let n = TEST.len() / 4;
        for x in 0..n {
            for y in 0..n {
                assert_eq!(counts_popDiff(TEST[x], TEST[y]), (counts_pop(TEST[x] as i64) - counts_pop(TEST[y] as i64)) as i32);
            }
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_coutns_popCmpr(b: &mut test::Bencher) {
    let TEST = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0x3F, 0x40, 0x41, 0x7f, 0x80, 0x81, 0xfe, 0xff,
                    0x4000, 0x4001, 0x7000, 0x7fff, 0x55555555, 0xAAAAAAAA, 0xFF000000, 0xC0C0C0C0, 0x0FFFFFF0, 0x80000000, 0xFFFFFFFE, 0xFFFFFFFF];
    b.iter(|| {
        let n = TEST.len() / 4;
        for x in 0..n {
            for y in 0..n {
                let a = counts_pop(TEST[x]);
                let b = counts_pop(TEST[y]);
                let c = counts_popCmpr(TEST[x] as i32, TEST[y] as i32);
                assert_eq!(a > b && c > 0 || a < b && c < 0 || a == b && c == 0, true);
            }
        }
    });
}