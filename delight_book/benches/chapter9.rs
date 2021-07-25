#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter9::*;
use std::borrow::BorrowMut;

#[bench]
#[allow(overflowing_literals)]
fn bench_multiply_mulmns(b: &mut test::Bencher) {
    let test = [
        // m, n, u...,          v...,          cq...,  cr....
        1, 1, 3, 0, 1, 1,
        1, 2, 7, 1, 3, 0, 7, 0,
        2, 2, 0, 0, 1, 0, 0, 0, 0,
        1, 1, 3, 2, 1, 1,
        1, 1, 3, 3, 1, 0,
        1, 1, 3, 4, 0, 3,
        1, 1, 0, 0xffff, 0, 0,
        1, 1, 0xffff, 1, 0xffff, 0,
        1, 1, 0xffff, 0xffff, 1, 0,
        1, 1, 0xffff, 3, 0x5555, 0,
        2, 1, 0xffff, 0xffff, 1, 0xffff, 0xffff, 0,
        2, 1, 0xffff, 0xffff, 0xffff, 1, 1, 0,
        2, 1, 0xffff, 0xfffe, 0xffff, 0xffff, 0, 0xfffe,
        2, 1, 0x5678, 0x1234, 0x9abc, 0x1e1e, 0, 0x2c70,
        2, 2, 0, 0, 0, 1, 0, 0, 0,
        2, 2, 0, 7, 0, 3, 2, 0, 1,
        2, 2, 5, 7, 0, 3, 2, 5, 1,
        2, 2, 0, 6, 0, 2, 3, 0, 0,
        2, 2, 0x0001, 0x8000, 0x7000, 0x4000, 0x0001, 0x9001, 0x3fff,
        2, 2, 0x789a, 0xbcde, 0x789a, 0xbcde, 1, 0, 0,
        2, 2, 0x789b, 0xbcde, 0x789a, 0xbcde, 1, 1, 0,
        2, 2, 0x7899, 0xbcde, 0x789a, 0xbcde, 0, 0x7899, 0xbcde,
        2, 2, 0xffff, 0xffff, 0xffff, 0xffff, 1, 0, 0,
        2, 2, 0xffff, 0xffff, 0x0000, 0x0001, 0xffff, 0xffff, 0,
        3, 2, 0x89ab, 0x4567, 0x0123, 0x0000, 0x0001, 0x4567, 0x0123, 0x89ab, 0,
        3, 2, 0x0000, 0xfffe, 0x8000, 0xffff, 0x8000, 0xffff, 0x0000, 0xffff, 0x7fff, // Shows that first qhat can = b + 1.
        3, 3, 0x0003, 0x0000, 0x8000, 0x0001, 0x0000, 0x2000, 0x0003, 0, 0, 0x2000, // Adding back step req'd.
        4, 3, 0, 0, 0x8000, 0x7fff, 1, 0, 0x8000, 0xfffe, 0, 2, 0xffff, 0x7fff,  // Add back req'd.
        4, 3, 0, 0xfffe, 0, 0x8000, 0xffff, 0, 0x8000, 0xffff, 0, 0xffff, 0xffff, 0x7fff  // Shows that mult-sub quantity cannot be treated as signed.
    ];

    for i in 0..10 / 2 {
        let m = test[i] as i32;
        let n = test[i + 1] as i32;
        let u = &test[i + 2..];
        let v = &test[i + 2 + m as usize..];
        let mut q:[u8;10]=[0;10];
        let mut r:[u8;10]=[0;10];
        let cq = &test[(i + 2 + m as usize + n as usize)as usize];
        let cr = &test[(i + 2 + m as usize + n as usize+ core::cmp::max(m - n + 1, 1) as usize) as usize];
        assert_eq!(division_divmnu(q.borrow_mut(),r.borrow_mut(),u,v,m,n)!=10,true);
    }
}