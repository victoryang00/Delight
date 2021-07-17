#![feature(test)]

extern crate test;
extern crate delight_book;

use delight_book::chapter2::*;

#[bench]
fn bench_ntz(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz1(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz1(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz2(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz2(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz3(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz3(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz4(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz4(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz5_nozero(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![ 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];

    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz5(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz6(b: &mut test::Bencher) {
    let TEST:Vec<u32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0,
                         8, 3, 9, 0, 16, 4, 32, 5, 64, 6, 128, 7, 255, 0, 256, 8, 512, 9, 1024, 10,
                         2048, 11, 4096, 12, 8192, 13, 16384, 14, 32768, 15, 65536, 16,
                         0x20000, 17, 0x40000, 18, 0x80000, 19, 0x100000, 20, 0x200000, 21,
                         0x400000, 22, 0x800000, 23, 0x1000000, 24, 0x2000000, 25,
                         0x4000000, 26, 0x8000000, 27, 0x10000000, 28, 0x20000000, 29,
                         0x40000000, 30, 0x80000000, 31, 0xFFFFFFF0, 4, 0x3000FF00, 8,
                         0xC0000000, 30, 0x60000000, 29, 0x00011000, 12];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz6(TEST[2*i]),TEST[2*i+1])
        }
    })
}
#[bench]
fn bench_ntz7_easy(b: &mut test::Bencher) {
    let TEST:Vec<i32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];

    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz7(TEST[2 * i] as i32), TEST[2*i+1] as i32)
        }
    })
}
#[bench]
fn bench_ntz8_easy(b: &mut test::Bencher) {
    let TEST:Vec<i32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];

    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz8(TEST[2*i] as i32),TEST[2*i+1] as i32)
        }
    })
}
#[bench]
fn bench_ntz9_easy(b: &mut test::Bencher) {
    let TEST:Vec<i32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];

    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz9(TEST[2*i] as i32),TEST[2*i+1] as i32)
        }
    })
}
#[bench]
fn bench_ntz10_easy(b: &mut test::Bencher) {
    let TEST:Vec<i32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];

    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz10(TEST[2*i] as i32),TEST[2*i+1] as i32)
        }
    })
}
#[bench]
fn bench_ntz11_easy(b: &mut test::Bencher) {
    let TEST:Vec<i32> = vec![0, 32, 1, 0, 2, 1, 3, 0, 4, 2, 5, 0, 6, 1, 7, 0 ];
    b.iter(|| {
        let n = TEST.len()/2;
        for i in 0..n {
            assert_eq!(ntz11(TEST[2*i] as i32),TEST[2*i+1] as i32)
        }
    })
}
