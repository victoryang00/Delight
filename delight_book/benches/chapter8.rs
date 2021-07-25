#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter8::*;
use std::borrow::{BorrowMut, Borrow};

fn multiply_check(result: &mut [u8], u: &mut [u8], v: &mut [u8], m: i32, n: i32, correct: &mut [u8]) -> i32 {
    let mut errors = 0;
    for i in 0..m + n {
        if correct[i as usize] != result[i as usize] {
            errors = errors + 1;
            println!("Error, m = {}, n = {}, u = ", m, n);
            for j in 0..m { println!(" {}", u[j as usize]); }
            println!(" v =");
            for j in 0..n { println!(" {}", v[j as usize]); }
            println!("\nShould get:");
            for j in 0..n + m { println!(" {}", correct[j as usize]); }
            println!("\n       Got:");
            for j in 0..n + m { println!(" {}", result[j as usize]); }
            println!("\n");
            break;
        }
    }
    errors
}

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulmns(b: &mut test::Bencher) {
    let mut test: [u8; 6] = [1, 1, 7, 3, 21, 0];
    let mut test1: [u8; 6] = [1, 1, 7, 3, 21, 0];

    b.iter(|| {
        let n = test.len();
        for i in 0..n / 4 {
            let m = test[i];
            let n = test[i + 1];
            let mut u: &mut [u8] = test[((i + 2) as usize)..].borrow_mut();
            let mut v: &mut [u8] = test1[((i + m as usize + 2) as usize)..].borrow_mut();
            let result: &mut [u8] = &mut [0; 500];
            multiply_mulmns(result, u, v, m as i32, n as i32);
            assert_eq!(multiply_check(result, u, v, m as i32, n as i32, &mut [0; 500]), 1);
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulinv(b: &mut test::Bencher) {
    let mut test: [i32; 28] = [1, 1, 3, 0xAAAAAAAB, 5, 0xCCCCCCCD,
        7, 0xB6DB6DB7, 9, 0x38E38E39, 11, 0xBA2E8BA3,
        13, 0xC4EC4EC5, 15, 0xEEEEEEEF, 25, 0xC28F5c29,
        125, 0x26E978D5, -1, 0xFFFFFFFF, -3, 0x55555555,
        -5, 0x33333333, -7, 0x49249249];

    b.iter(|| {
        let n = test.len();
        for i in 0..n / 4 {
            assert_eq!(multiply_mulinv(test[i]), test[i + 1]);
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulinv2(b: &mut test::Bencher) {
    let mut test: [i32; 28] = [1, 1, 3, 0xAAAAAAAB, 5, 0xCCCCCCCD,
        7, 0xB6DB6DB7, 9, 0x38E38E39, 11, 0xBA2E8BA3,
        13, 0xC4EC4EC5, 15, 0xEEEEEEEF, 25, 0xC28F5c29,
        125, 0x26E978D5, -1, 0xFFFFFFFF, -3, 0x55555555,
        -5, 0x33333333, -7, 0x49249249];

    b.iter(|| {
        let n = test.len();
        for i in 0..n / 4 {
            assert_eq!(multiply_mulinv2(test[i]), test[i + 1]);
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulmnu(b: &mut test::Bencher) {
    let mut test: [u8; 86] = [1, 1, 7, 3, 21, 0,
        1, 1, 2, 0xFFFF, 0xFFFE, 0x0001, // 2*FFFF = 0001_FFFE.
        1, 1, 0xFFFF, 0xFFFF, 1, 0xFFFE,
        1, 2, 7, 5, 6, 35, 42, 0,
        1, 2, 65000, 63000, 64000, 0xBDC0, 0x8414, 0xF7F5,
        1, 3, 65535, 31000, 32000, 33000, 0x86E8, 0xFC17, 0xFC17, 0x80E7,
        2, 3, 400, 300, 500, 100, 200, 0x0D40, 0xE633, 0xADB2, 0xEA61, 0,
        2, 3, 400, 65535, 500, 100, 65534, 0x0D40, 0x9A4F, 0xFE70, 0x01F5, 0xFFFD,
        4, 4, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
        1, 0, 0, 0, 65534, 65535, 65535, 65535];
    let mut test1: [u8; 86] = [1, 1, 7, 3, 21, 0,
        1, 1, 2, 0xFFFF, 0xFFFE, 0x0001, // 2*FFFF = 0001_FFFE.
        1, 1, 0xFFFF, 0xFFFF, 1, 0xFFFE,
        1, 2, 7, 5, 6, 35, 42, 0,
        1, 2, 65000, 63000, 64000, 0xBDC0, 0x8414, 0xF7F5,
        1, 3, 65535, 31000, 32000, 33000, 0x86E8, 0xFC17, 0xFC17, 0x80E7,
        2, 3, 400, 300, 500, 100, 200, 0x0D40, 0xE633, 0xADB2, 0xEA61, 0,
        2, 3, 400, 65535, 500, 100, 65534, 0x0D40, 0x9A4F, 0xFE70, 0x01F5, 0xFFFD,
        4, 4, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
        1, 0, 0, 0, 65534, 65535, 65535, 65535];
    let mut test2: [u8; 86] = [1, 1, 7, 3, 21, 0,
        1, 1, 2, 0xFFFF, 0xFFFE, 0x0001, // 2*FFFF = 0001_FFFE.
        1, 1, 0xFFFF, 0xFFFF, 1, 0xFFFE,
        1, 2, 7, 5, 6, 35, 42, 0,
        1, 2, 65000, 63000, 64000, 0xBDC0, 0x8414, 0xF7F5,
        1, 3, 65535, 31000, 32000, 33000, 0x86E8, 0xFC17, 0xFC17, 0x80E7,
        2, 3, 400, 300, 500, 100, 200, 0x0D40, 0xE633, 0xADB2, 0xEA61, 0,
        2, 3, 400, 65535, 500, 100, 65534, 0x0D40, 0x9A4F, 0xFE70, 0x01F5, 0xFFFD,
        4, 4, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
        1, 0, 0, 0, 65534, 65535, 65535, 65535];
    let mut test3: [u8; 86] = [1, 1, 7, 3, 21, 0,
        1, 1, 2, 0xFFFF, 0xFFFE, 0x0001, // 2*FFFF = 0001_FFFE.
        1, 1, 0xFFFF, 0xFFFF, 1, 0xFFFE,
        1, 2, 7, 5, 6, 35, 42, 0,
        1, 2, 65000, 63000, 64000, 0xBDC0, 0x8414, 0xF7F5,
        1, 3, 65535, 31000, 32000, 33000, 0x86E8, 0xFC17, 0xFC17, 0x80E7,
        2, 3, 400, 300, 500, 100, 200, 0x0D40, 0xE633, 0xADB2, 0xEA61, 0,
        2, 3, 400, 65535, 500, 100, 65534, 0x0D40, 0x9A4F, 0xFE70, 0x01F5, 0xFFFD,
        4, 4, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
        1, 0, 0, 0, 65534, 65535, 65535, 65535];
    b.iter(|| {
        let n = test.len();
        let mut i = 0;
        loop {
            if i >= n / 2 {
                break;
            }
            let m = test[i];
            let n = test[i + 1];
            let mut u: &mut [u8] = test[((i + 2) as usize)..].borrow_mut();
            let mut v: &mut [u8] = test1[((i + m as usize + 2) as usize)..].borrow_mut();
            let result: &mut [u8] = &mut [0; 500];
            multiply_mulmnu(result, u, v, m as i32, n as i32);
            assert_eq!(multiply_check(result, u, v, m as i32, n as i32, test2[((i + 2) as usize)..].borrow_mut()), 1);
            multiply_mulmnu(result, v, u, m as i32, n as i32);
            assert_eq!(multiply_check(result, v, u, m as i32, n as i32, test3[((i + 2) as usize)..].borrow_mut()), 1);
            i = i + 2 + 2 * (m + n) as usize;
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulmnu1(b: &mut test::Bencher) {
    let mut test: [[[i32; 1]; 8]; 4] = [[[0],[1], [0],[1], [0],[0],[0],[1]],                    /* 1*1 = 1 */
        [[0],[0], [0xFFFFFFFF],[0xFFFFFFFF], [0],[0],[0],[0]],  /* 0*big = 0 */
        [[0],[7], [0],[3], [0],[0],[0],[21]],                   /* 7*3 = 21 */
        [[0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFE],[0x00000000],[0x00000001]]];
    let mut test1: [[[i32; 1]; 8]; 4] = [[[0],[1], [0],[1], [0],[0],[0],[1]],                    /* 1*1 = 1 */
        [[0],[0], [0xFFFFFFFF],[0xFFFFFFFF], [0],[0],[0],[0]],  /* 0*big = 0 */
        [[0],[7], [0],[3], [0],[0],[0],[21]],                   /* 7*3 = 21 */
        [[0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFE],[0x00000000],[0x00000001]]];
    let mut test2: [[[i32; 1]; 8]; 4] = [[[0],[1], [0],[1], [0],[0],[0],[1]],                    /* 1*1 = 1 */
        [[0],[0], [0xFFFFFFFF],[0xFFFFFFFF], [0],[0],[0],[0]],  /* 0*big = 0 */
        [[0],[7], [0],[3], [0],[0],[0],[21]],                   /* 7*3 = 21 */
        [[0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFE],[0x00000000],[0x00000001]]];
    let mut test3: [[[i32; 1]; 8]; 4] = [[[0],[1], [0],[1], [0],[0],[0],[1]],                    /* 1*1 = 1 */
        [[0],[0], [0xFFFFFFFF],[0xFFFFFFFF], [0],[0],[0],[0]],  /* 0*big = 0 */
        [[0],[7], [0],[3], [0],[0],[0],[21]],                   /* 7*3 = 21 */
        [[0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFF], [0xFFFFFFFF],[0xFFFFFFFE],[0x00000000],[0x00000001]]];
    b.iter(|| {
        let n = test.len();
        for i in 0..n/32 {
            let result: &mut [i32] = &mut [0; 4];
            multiply_mulqdu1(result, &mut test[i][0], &mut test1[i][2]);
            multiply_mulqdu1(result, &mut test2[i][2], &mut test3[i][0]);
        }
    });
}
