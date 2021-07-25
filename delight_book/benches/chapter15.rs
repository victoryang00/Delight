#![feature(test)]
extern crate test;
extern crate delight_book;
extern crate rand;

use delight_book::chapter15::*;
use rand::Rng;

#[bench]
fn bench_hamming_code(b: &mut test::Bencher) {
    b.iter(||
        for i in 0..(39 * 40) / 2 + 1 {
            let us = rand::thread_rng().gen::<i64>() * 3;          // Generate random information bits
            // (rand() always has the msb = 0).
            let mut ps = hamming_checkbits(us);               // Compute their 6 check bits
            ps = ps | (hamming_parity(us ^ ps) << 6); // and prepend the overall
            // parity bit.
            let mut ur = us;                  // Set up the received data.
            let mut pr = ps;
            let e = hamming_perturb(&mut pr, &mut ur);    // Alter 0, 1, or 2 bits of pr and ur.
            let mut uc = ur;
            let c = hamming_correct(pr, &mut uc);     // Correct ur if 1 error occurred.

            println!("{}  {}   {}  {}   ", ps, us, pr, ur); // Program
            println!("{}   {}   {}\n", e, c, uc);                 // trace.

            assert_eq!(e ,c);

            assert_eq!(e <= 1 && uc != us,false)
        });
}