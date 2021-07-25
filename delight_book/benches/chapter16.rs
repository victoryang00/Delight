#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter16::*;
use std::borrow::BorrowMut;

#[bench]
fn bench_logic_inc_from_xy(b: &mut test::Bencher) {
    let mut I = 0;
    let mut W = 0;
    b.iter(|| {
        let n = 30;
        let N: i64 = 1 << 2 * n;
        let mut x = 0;
        let mut y = 0;
        for _ in 0..N {
            hilbert_logic_inc_from_xy(x, y, n, &mut I, &mut W);
            if W == 0 {
                x = x + I;
            } else {
                y = y + I;
            }
        }
    })
}

#[bench]
fn bench_hilbert_logic_xy_from_s(b: &mut test::Bencher) {
    b.iter(|| {
        let n = 15;
        let N: i64 = 1 << 2 * n;
        let mut x = 0;
        let mut y = 0;
        for s in 0..N {
            hilbert_logic_xy_from_s(s as i32, n, &mut x, &mut y);
        }
    })
}

#[bench]
fn bench_glsxy(b: &mut test::Bencher) {
    b.iter(|| {
        let n = 30;
        let N = 1 << (2 * n);
        let mut xy1 = addressOfXY { s: 0, xp: &mut 0, yp: &mut 0 };
        let mut xy3 = addressOfXY { s: 0, xp: &mut 0, yp: &mut 0 };
        assert_eq!(xy1.xp, xy3.xp);
        assert_eq!(xy1.yp, xy3.yp);
        for s in 0..N {
            xy1 = hilbert_glsxy(s, n, xy1.xp.borrow_mut(), xy1.yp.borrow_mut());
            xy3 = hilbert_glsxy3(s, n, xy3.xp.borrow_mut(), xy3.yp.borrow_mut());
        }
    })
}

#[bench]
fn bench_lams(b: &mut test::Bencher) {
    b.iter(|| {
        let n = 10;
        let N: u64 = 1 << n;
        for x in 0..N {
            for y in 0..N {
                assert_eq!(hilbert_lams1(x, y, n), hilbert_lams(x, y, n));
            }
        }
    })
}

#[bench]
fn bench_lamxy(b: &mut test::Bencher) {
    let mut x = 0;
    let mut y = 0;
    b.iter(|| {
        let n = 10;
        let N = 1 << 2 * n;
        for s in 0..N {
            assert_eq!(hilbert_lamxy(s, n, &mut x, &mut y), hilbert_lamxy1(s, n, &mut x, &mut y));
        }
    })
}
