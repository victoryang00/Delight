#![feature(test)]

extern crate test;
extern crate delight_book;

use delight_book::chapter6::*;

#[bench]
fn bench_search_zbytel1(b: &mut test::Bencher) {
    let test = vec![0x00000000, 0, 0x00000001, 0];

    let test2 = vec![0x00000000, 0, 0x00000001, 0];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel1(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel1(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel1(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_zbytel2(b: &mut test::Bencher) {
    let test = vec![0x00000000, 0, 0x00000001, 0];

    let test2 = vec![0x00000000, 0, 0x00000001, 0];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel2(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel2(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel2(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_zbytel3(b: &mut test::Bencher) {
    let test = vec![];

    let test2 = vec![];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_zbytel3a(b: &mut test::Bencher) {
    let test = vec![0x00000000, 0, 0x00000001, 0, 0x00800000, 0, 0x00FFFFFF, 0, 0x01000000, 1, 0x0100FFFF, 1, 0x7F000000, 1];

    let test2 = vec![0x00000000, 0, 0x00000001, 0, 0x09000000, 0, 0x09FFFFFF, 0, 0x0A010000, 1, 0x1909FFFF, 1];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3a(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3a(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel3a(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_zbytel4(b: &mut test::Bencher) {
    let test = vec![0x00000000, 0, 0x00000001, 0, 0x00800000, 0, 0x00FFFFFF, 0, 0x01000000, 1, 0x0100FFFF, 1, 0x7F000000, 1, 0x8000FFFF, 1,
                    0x01010000, 2, 0x010100FF, 2, 0x7F7F0000, 2, 0xFFFF00FF, 2, 0x01010100, 3, 0x01017F00, 3, 0x7F7F8000, 3, 0xFFFFFF00, 3, 0x01010101, 4, 0x80808080, 4, 0x7F7F7F7F, 4, 0xFFFFFFFF, 4];

    let test2 = vec![];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel4(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel4(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_zbytel4(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_valle9(b: &mut test::Bencher) {
    let test = vec![0x00000000, 0, 0x00000001, 0];

    let test2 = vec![0x00000000, 0, 0x00000001, 0];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valle9(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valle9(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valle9(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
fn bench_search_valupcase(b: &mut test::Bencher) {
    let test = vec![];

    let test2 = vec![];

    let test3 = vec![];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valupcase(test[i]), test[i + 1]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valupcase(test2[i]), test2[i + 1]);
        }
        let n = test3.len() / 4;
        for i in (0..n).step_by(2) {
            assert_eq!(search_valupcase(test3[i]), test3[i + 1]);
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_search_fminstr1(b: &mut test::Bencher) {
    /* This array, for testing "fminstr1," has a test number x, the
    length of the shortest string of 1's in x, and its position (distance
    from the left or MSB). */
    let test = vec![0, 0, 32, 1, 1, 31, 15, 4, 28, 0x80000000, 1, 0,
                    0x0F0F0F0F, 4, 4, 0xF0F0F0F0, 4, 0, 0x55555555, 1, 1, 0xF0000000, 4, 0,
                    0xF0E07060, 2, 25, 0xFFFF0000, 16, 0, 0xFFFE0000, 15, 0,
                    0xFFFF8000, 17, 0, 0xB57EEFDF, 1, 0, 0xFFFEFFFF, 15, 0,
                    0xFFFF7FFF, 15, 17, 0xfffffffe, 31, 0, 0x7fffffff, 31, 1,
                    0x7FFFFFFE, 30, 1, -1, 32, 0, 0xfefdfdff, 6, 8];
    /* This array, for testing "fminstr1," has a test number x, the
    length of the shortest string of 1's in x, and its position (distance
    from the left or MSB). */
    let test2 = vec![0, 1, 0, 32, 1, 1, 1, 31, 15, 6, 5, 32, 15, 5, 4, 32,
                     15, 4, 4, 28, 15, 3, 4, 28, 15, 2, 4, 28, 15, 1, 4, 28, 0x80000000, 1, 1, 0,
                     0x80000000, 2, 1, 32, 0x80000000, 3, 2, 32, 0xE0000000, 1, 3, 0,
                     0xE0000000, 2, 3, 0, 0xE0000000, 3, 3, 0, 0xE0000000, 4, 3, 32,
                     0x0F0F0F0F, 1, 4, 4, 0x0F0F0F0F, 2, 4, 4, 0x0F0F0F0F, 3, 4, 4,
                     0x0F0F0F0F, 4, 4, 4, 0x0F0F0F0F, 5, 4, 32, 0x0F0F80FC, 1, 4, 4,
                     0x0F0F80FC, 2, 4, 4, 0x0F0F80FC, 3, 4, 4, 0x0F0F80FC, 5, 5, 12,
                     0x0F0F80FC, 6, 6, 24, 0x0F0F80FC, 7, 6, 32, 0x0F0F80FC, 8, 7, 32,
                     0x12345678, 1, 1, 3, 0x12345678, 2, 2, 10, 0x12345678, 3, 4, 25,
                     0x12345678, 4, 4, 25, 0x12345678, 5, 4, 32, 0x12345678, 6, 5, 32,
                     0xF8FFF7FF, 10, 11, 21, 0xF8FFF7FF, 11, 11, 21, 0xF8FFF7FF, 12, 12, 8,
                     0xF8FFF7FF, 13, 12, 32, 0x7FFFFFFF, 1, 31, 1, 0x7FFFFFFF, 30, 31, 1,
                     0x7FFFFFFF, 31, 31, 1, 0x7FFFFFFF, 32, 31, 32, 0xFFFFFFFE, 1, 31, 0,
                     0xFFFFFFFE, 30, 31, 0, 0xFFFFFFFE, 31, 31, 0, 0xFFFFFFFE, 32, 31, 32,
                     0xFFFFFFFF, 1, 32, 0, 0xFFFFFFFF, 31, 32, 0, 0xFFFFFFFF, 32, 32, 0,
                     0xFFFFFFFF, 33, 32, 32, 0xFFFFFFFF, 99, 98, 32];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(3) {
            let mut pos =0;
            let len = search_fminstr1(test[i],&mut pos);
            assert_eq!(len,test[i+1]);
            assert_eq!(pos,test[i+2]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(3) {
            let mut pos =0;
            let len = search_fminstr1(test2[i],&mut pos);
            assert_eq!(len,test2[i+1]);
            assert_eq!(pos,test2[i+2]);
        }
    });
}

#[bench]
#[allow(overflowing_literals)]
fn bench_search_fminstr11(b: &mut test::Bencher) {
    /* This array, for testing "fminstr11," has a test number x, the
    length of the shortest string of 1's in x, and its position (distance
    from the left or MSB). */
    let test = vec![0, 0, 32, 1, 1, 31, 15, 4, 28, 0x80000000, 1, 0,
                    0x0F0F0F0F, 4, 4, 0xF0F0F0F0, 4, 0, 0x55555555, 1, 1, 0xF0000000, 4, 0,
                    0xF0E07060, 2, 25, 0xFFFF0000, 16, 0, 0xFFFE0000, 15, 0,
                    0xFFFF8000, 17, 0, 0xB57EEFDF, 1, 0, 0xFFFEFFFF, 15, 0,
                    0xFFFF7FFF, 15, 17, 0xfffffffe, 31, 0, 0x7fffffff, 31, 1,
                    0x7FFFFFFE, 30, 1, -1, 32, 0, 0xfefdfdff, 6, 8];
    /* This array, for testing "fminstr11," has a test number x, the
    length of the shortest string of 1's in x, and its position (distance
    from the left or MSB). */
    let test2 = vec![0, 1, 0, 32, 1, 1, 1, 31, 15, 6, 5, 32, 15, 5, 4, 32,
                     15, 4, 4, 28, 15, 3, 4, 28, 15, 2, 4, 28, 15, 1, 4, 28, 0x80000000, 1, 1, 0,
                     0x80000000, 2, 1, 32, 0x80000000, 3, 2, 32, 0xE0000000, 1, 3, 0,
                     0xE0000000, 2, 3, 0, 0xE0000000, 3, 3, 0, 0xE0000000, 4, 3, 32,
                     0x0F0F0F0F, 1, 4, 4, 0x0F0F0F0F, 2, 4, 4, 0x0F0F0F0F, 3, 4, 4,
                     0x0F0F0F0F, 4, 4, 4, 0x0F0F0F0F, 5, 4, 32, 0x0F0F80FC, 1, 4, 4,
                     0x0F0F80FC, 2, 4, 4, 0x0F0F80FC, 3, 4, 4, 0x0F0F80FC, 5, 5, 12,
                     0x0F0F80FC, 6, 6, 24, 0x0F0F80FC, 7, 6, 32, 0x0F0F80FC, 8, 7, 32,
                     0x12345678, 1, 1, 3, 0x12345678, 2, 2, 10, 0x12345678, 3, 4, 25,
                     0x12345678, 4, 4, 25, 0x12345678, 5, 4, 32, 0x12345678, 6, 5, 32,
                     0xF8FFF7FF, 10, 11, 21, 0xF8FFF7FF, 11, 11, 21, 0xF8FFF7FF, 12, 12, 8,
                     0xF8FFF7FF, 13, 12, 32, 0x7FFFFFFF, 1, 31, 1, 0x7FFFFFFF, 30, 31, 1,
                     0x7FFFFFFF, 31, 31, 1, 0x7FFFFFFF, 32, 31, 32, 0xFFFFFFFE, 1, 31, 0,
                     0xFFFFFFFE, 30, 31, 0, 0xFFFFFFFE, 31, 31, 0, 0xFFFFFFFE, 32, 31, 32,
                     0xFFFFFFFF, 1, 32, 0, 0xFFFFFFFF, 31, 32, 0, 0xFFFFFFFF, 32, 32, 0,
                     0xFFFFFFFF, 33, 32, 32, 0xFFFFFFFF, 99, 98, 32];

    b.iter(|| {
        let n = test.len() / 4;
        for i in (0..n).step_by(3) {
            let mut pos =0;
            let len = search_fminstr11(test[i],&mut pos);
            assert_eq!(len,test[i+1]);
            assert_eq!(pos,test[i+2]);
        }
        let n = test2.len() / 4;
        for i in (0..n).step_by(3) {
            let mut pos =0;
            let len = search_fminstr11(test2[i],&mut pos);
            assert_eq!(len,test2[i+1]);
            assert_eq!(pos,test2[i+2]);
        }
    });
}