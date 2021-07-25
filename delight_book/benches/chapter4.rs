#![feature(test)]
extern crate test;
extern crate delight_book;

use delight_book::chapter4::*;

#[bench]
fn bench_boundsAdd(b: &mut test::Bencher) {
    let TEST = [[0x00000000, 0x00000001, 0x00000002, 0x00000003, 0x00000002, 0x00000004],
        [0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff]];
    b.iter(|| {
        let n = TEST.len();
        for i in 0..n {
            assert_eq!(boundsAddu(TEST[i][0], TEST[i][1], TEST[i][2], TEST[i][3]), (TEST[i][4], TEST[i][5]));
            assert_eq!(boundsAdds(TEST[i][0], TEST[i][1], TEST[i][2], TEST[i][3]), (TEST[i][4], TEST[i][5]));
        }
    })
}

#[bench]
fn bench_bounds_maxand(b: &mut test::Bencher) {
    let n: i64 = 5;                       // Size of problem.
    let nn: i64 = 1 << n;                 // 2**n.
    b.iter(|| {
        for a in 0..nn {
            for b in 0..nn {
                for c in 0..nn {
                    for d in c..nn {
                        let rmax = bounds_brute(a, b, c, d);        // Correct result.
                        let r1 = bounds_maxAND(a, b, c, d);
                        let r2 = !bounds_minOR(!b, !a, !d, !c);     // Algebraic method.
                        assert_eq!(r1 != rmax || r2 != rmax, true);
                    }
                }
            }
        }
    })
}

