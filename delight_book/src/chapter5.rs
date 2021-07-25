use c8;
use rand::Rng;
use rand::SeedableRng;
use rand_core::RngCore;
use rand_isaac::IsaacRng;
#[cfg(target_arch = "x86_64")]
use std::{borrow::{Borrow,BorrowMut}, sync::atomic::{Ordering,AtomicUsize}};
#[cfg(target_arch = "riscv64")]
use core::{borrow::{Borrow,BorrowMut}, sync::atomic::{Ordering,AtomicUsize}};

static SEED: AtomicUsize = AtomicUsize::new(0);

pub fn counts_divide_and_conquer(mut x: i32) -> i32 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    x
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts() {
    assert_eq!(counts_divide_and_conquer(1), 1);
    assert_eq!(counts_divide_and_conquer(2), 1);
}

/// The first assignment to x is based on the first two terms of the rather surprising
/// formula
/// b3b2b1b0 => x-⌊x/2⌋-⌊x/4⌋-⌊x/8⌋
pub fn counts_pop(mut x: i64) -> i64 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x >> 8);
    x = x + (x >> 16);
    return x & 0x0000003F;
}

pub fn counts_rotatel(mut x: u32, n: u32) -> u32 {
    if n > 63 {
        ()
    }
    return (x.wrapping_shl(n)) | (x.wrapping_shr(32 - n));
}

pub fn counts_pop0(mut x: i64) -> i64 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    return x;
}

pub fn counts_pop1(mut x: i64) -> i64 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x >> 8);
    x = x + (x >> 16);
    return x & 0x0000003F;
}
/* Note: an alternative to the last three executable lines above is:
   return x*0x01010101 >> 24;
if your machine has a fast multiplier (suggested by Jari Kirma). */
pub fn counts_pop2(mut x: i64) -> i64 {
    let mut n = (x >> 1) & 033333333333;       // Count bits in
    x = x - n;                                      // each 3-bit
    n = (n >> 1) & 033333333333;                    // field.
    x = x - n;
    x = (x + (x >> 3)) & 030707070707;              // 6-bit sums.
    return (x % 63 as i64) as i64;                  // Add 6-bit sums.
}
/* An alternative to the "return" statement above is:
   return ((x * 0404040404) >> 26) +  // Add 6-bit sums.
           (x >> 30);
which runs faster on most machines (suggested by Norbert Juffa). */
pub fn counts_pop3(mut x: i64) -> i64 {
    let mut n = (x >> 1) & 0x77777777;        // Count bits in
    x = x - n;                                     // each 4-bit
    n = (n >> 1) & 0x77777777;                     // field.
    x = x - n;
    n = (n >> 1) & 0x77777777;
    x = x - n;
    x = (x + (x >> 4)) & 0x0F0F0F0F;               // Get byte sums.
    x = x * 0x01010101;                            // Add the bytes.
    return x >> 24;
}

pub fn counts_pop4(mut x: i64) -> i64 {
    let mut n: i64 = 0;
    while (x != 0) {
        n = n + 1;
        x = x & (x - 1);
    }
    return n;
}

pub fn counts_pop5(mut x: u32) -> i32 {
    let mut sum: i32 = x as i32;                // sum = x;
    for i in 1..32 {                       // while (x != 0) {
        x = counts_rotatel(x, 1);            //    x = x >> 1;
        sum = sum.wrapping_add(x as i32);   //    sum = sum - x;
    }                                           // }
    return -sum;                                // return sum;
}

pub fn counts_pop5a(mut x: i64) -> i64 {
    let mut sum = x;
    while x != 0 {
        x = x >> 1;
        sum = sum - x;
    }
    return sum;
}

pub fn counts_pop6(mut x: i64) -> i64 {
    // Table lookup.
    let table = [
        0 as char, 1 as char, 1 as char, 2 as char, 1 as char, 2 as char, 2 as char, 3 as char, 1 as char, 2 as char, 2 as char, 3 as char, 2 as char, 3 as char, 3 as char, 4 as char,
        1 as char, 2 as char, 2 as char, 3 as char, 2 as char, 3 as char, 3 as char, 4 as char, 2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char,
        1 as char, 2 as char, 2 as char, 3 as char, 2 as char, 3 as char, 3 as char, 4 as char, 2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        1 as char, 2 as char, 2 as char, 3 as char, 2 as char, 3 as char, 3 as char, 4 as char, 2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char, 4 as char, 5 as char, 5 as char, 6 as char, 5 as char, 6 as char, 6 as char, 7 as char,
        1 as char, 2 as char, 2 as char, 3 as char, 2 as char, 3 as char, 3 as char, 4 as char, 2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char, 4 as char, 5 as char, 5 as char, 6 as char, 5 as char, 6 as char, 6 as char, 7 as char,
        2 as char, 3 as char, 3 as char, 4 as char, 3 as char, 4 as char, 4 as char, 5 as char, 3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char,
        3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char, 4 as char, 5 as char, 5 as char, 6 as char, 5 as char, 6 as char, 6 as char, 7 as char,
        3 as char, 4 as char, 4 as char, 5 as char, 4 as char, 5 as char, 5 as char, 6 as char, 4 as char, 5 as char, 5 as char, 6 as char, 5 as char, 6 as char, 6 as char, 7 as char,
        4 as char, 5 as char, 5 as char, 6 as char, 5 as char, 6 as char, 6 as char, 7 as char, 5 as char, 6 as char, 6 as char, 7 as char, 6 as char, 7 as char, 7 as char, 8 as char
    ];

    return table[(x & 0xFF) as usize] as i64 +
        table[((x >> 8) & 0xFF) as usize] as i64 +
        table[((x >> 16) & 0xFF) as usize] as i64 +
        table[(x >> 24) as usize] as i64;
}

// The following works only for 8-bit quantities.
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn counts_pop7(mut x: i32) -> i32 {
    x = x * 0x08040201;    // Make 4 copies.
    x = x >> 3;          // So next step hits proper bits.
    x = x & 0x11111111;  // Every 4th bit.
    x = x.wrapping_mul(0x11111111);    // Sum the digits (each 0 or 1).
    x = x >> 28;         // Position the result.
    return x;
}

// The following works only for 7-bit quantities.
pub fn counts_pop8(mut x: i32) -> i32 {
    x = x.wrapping_mul(0x02040810);    // Make 4 copies, left-adjusted.
    x = x & 0x11111111;  // Every 4th bit.
    x = x.wrapping_mul(0x11111111);    // Sum the digits (each 0 or 1).
    x = x >> 28;         // Position the result.
    return x;
}

// The following works only for 15-bit quantities.
pub fn counts_pop9(mut x: i64) -> i64 {
    let mut y = x.wrapping_mul(0x0002000400080010);
    y = y & 0x1111111111111111;
    y = y.wrapping_mul(0x1111111111111111);
    y = y >> 60;
    return y;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts_pop() {
    assert_eq!(counts_pop(1), 1);
    assert_eq!(counts_pop(2), 1);
    assert_eq!(counts_pop(3), 2);
    assert_eq!(counts_pop1(1), 1);
    assert_eq!(counts_pop1(2), 1);
    assert_eq!(counts_pop1(3), 2);
    assert_eq!(counts_pop2(1), 1);
    assert_eq!(counts_pop2(2), 1);
    assert_eq!(counts_pop2(3), 2);
    assert_eq!(counts_pop3(1), 1);
    assert_eq!(counts_pop3(2), 1);
    assert_eq!(counts_pop3(3), 2);
    assert_eq!(counts_pop4(1), 1);
    assert_eq!(counts_pop4(2), 1);
    assert_eq!(counts_pop4(3), 2);
    assert_eq!(counts_pop5(1), 1);
    assert_eq!(counts_pop5(2), 1);
    assert_eq!(counts_pop5(3), 2);
    assert_eq!(counts_pop5a(1), 1);
    assert_eq!(counts_pop5a(2), 1);
    assert_eq!(counts_pop5a(3), 2);
    assert_eq!(counts_pop6(1), 1);
    assert_eq!(counts_pop6(2), 1);
    assert_eq!(counts_pop6(3), 2);
    assert_eq!(counts_pop7(1), 1);
    assert_eq!(counts_pop7(2), 1);
    assert_eq!(counts_pop7(3), 2);
    assert_eq!(counts_pop8(1), 1);
    assert_eq!(counts_pop8(2), 1);
    assert_eq!(counts_pop8(3), 2);
    assert_eq!(counts_pop9(1), 1);
    assert_eq!(counts_pop9(2), 1);
    assert_eq!(counts_pop9(3), 2);
}

pub fn counts_popDiff(mut x: i32, mut y: i32) -> i32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    y = !y;
    y = y - ((y >> 1) & 0x55555555);
    y = (y & 0x33333333) + ((y >> 2) & 0x33333333);
    x = x + y;
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = x + (x >> 8);
    x = x + (x >> 16);
    return (x & 0x0000007F) - 32;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts_popDiff() {
    assert_eq!(counts_popDiff(1, 1), 0);
}


/// fours  twos    onnes  a_i  a_i+1
///   |     |        ↓     ↓    ↓
///   |     |    +--------------+
///   |     |    |     CAS1     | a_i+2 a_i+3
///   |     |    +--------------+   |     |
///   |     |      |       ↓        ↓     ↓
///   |     |      |        +--------------+
///   |    +--------------+↙|     CAS2     | a_i+4 a_i+5
///   |    |     CAS2     | +--------------+   |     |
///   |    +--------------+             ↓      ↓     ↓
///   |     |            |             +--------------+
///   |     |            |             |     CAS4     | a_i+6 a_i+7
///   |     |            |             +--↙-----------+   |     |
///   |     |            +----→+--------------+    ↓      ↓     ↓
///   |     |                  |     CAS6     | +--------------+
///   |     |                  +--------------+↙|     CAS4     |
///   ↓     ↓     ↓------------+         ↓      +-------↓------+
///   +--------------+                  twos           ones
///   |     CAS2     |
///   +---↓-------↓--+
///     eights  fours
///
pub fn counts_popArray1(A: & mut [i32], n: i32) -> i32 {
    let mut tot = 0;
    for i in 0..n {
        tot = tot + counts_pop(A[i as usize] as i64);
    }
    tot as i32
}

macro_rules! CSA {
    ($h:expr,$l:expr,$a:expr,$b:expr,$c:expr) => {
        let u = $a ^ $b;
        let v = $c;
        $h = ($a & $b) | (u & v); $l = u ^ v;
    }
}

/* This is Harley's basic method. It combines groups of three array
elements into two words to which pop(x) is applied. The running time,
ignoring loop control and loads, is 7 elementary ops plus 2 pop counts
for each 3 array elements, i.e., (7 + 2p)/3 ops per word, where p is the
number of ops for one population count. For p = 15 (code of Fig. 5-2,
inlined) this is 37/3 = 12.33 ops per word. */
pub fn counts_popArray2(A: & mut [i32], n: i32) -> i32 {
    let mut tot1 = 0;
    let mut tot2 = 0;
    let mut i: usize = 0;
    let mut twos = 0;
    let mut ones = 0;
    loop {
        i += 3;
        if i <= (n - 3) as usize {
            break;
        }
        CSA!(twos, ones, A[i], A[i+1], A[i+2]);
        tot1 = tot1 + counts_pop(ones as i64);
        tot2 = tot2 + counts_pop(twos as i64);
    }
    for t in i..(n as usize) {  // Add in the last
        tot1 = tot1 + counts_pop(A[t] as i64);
    } // 0, 1, or 2 elements.

    return (2 * tot2 + tot1) as i32;
}

/* This is Harley's basic method but used in a different way (at Seal's
suggestion) than that of the above function. It brings in values from
the array two at a time and combines them with "ones" and "twos". The
array is assumed to have at least one element.
   The running time, ignoring loop control and loads, is 6 elementary
ops plus 1 pop count for each 2 array elements, i.e., (6 + p)/2 ops per
word, where p is the number of ops for one population count. For p = 15
(code of Fig. 5-2, inlined) this is 21/2 = 10.5 ops per word. */
pub fn counts_popArray3(A: & mut [i32], n: i32) -> i32 {
    let mut tot = 0;                     // Initialize.
    let mut ones = 0;
    let mut twos = 0;
    let mut i: usize = 0;
    loop {
        i += 2;
        if i <= (n - 2) as usize {
            break;
        }
        CSA!(twos, ones, ones, A[i], A[i+1]);
        tot = tot + counts_pop(twos as i64);
    }
    tot = 2 * tot + counts_pop(ones as i64);

    if n & 1 != 0 {   // If there's a last one,
        tot = tot + counts_pop(A[i] as i64);
    } // add it in.

    return tot as i32;
}

/* This is similar to the above but it brings in array elements 4 at a
time and combines them with "ones", "twos", and "fours". Harley gave
this algorithm. The array is assumed to have at least three elements.
   The running time, ignoring loop control and loads, is 16 elementary
ops plus 1 pop count for each 4 array elements, i.e., (16 + p)/4 ops per
word, where p is the number of ops for one population count. For p = 15
(code of Fig. 5-2, inlined) this is 31/4 = 7.75 ops per word. */
pub fn counts_popArray4(A: & mut [i32], n: i32) -> i32 {
    let mut tot = 0;                     // Initialize.
    let mut twos = 0;
    let mut twosA = 0;
    let mut twosB = 0;
    let mut fours = 0;
    let mut ones = 0;
    let mut i: usize = 0;
    loop {
        i += 4;
        if i <= (n - 4) as usize {
            break;
        }
        CSA!(twosA, ones, ones, A[i], A[i+1]);
        CSA!(twosB, ones, ones, A[i+2], A[i+3]);
        CSA!(fours, twos, twos, twosA, twosB);
        tot = tot + counts_pop(fours as i64);
    }
    tot = 4 * tot + 2 * counts_pop(twos as i64) + counts_pop(ones as i64);

    for t in i..(n as usize) {   // Simply add in the last
        tot = tot + counts_pop(A[t] as i64);
    }    // 0, 1, 2, or 3 elements.
    return tot as i32;
}

/* At the risk of being a bore, the function below is similar to that
above, but it brings in array elements 8 at a time and combines them
with "ones", "twos", "fours", and "eights". The array is assumed to have
at least seven elements.
   The running time, ignoring loop control and loads, is 36 elementary
ops plus 1 pop count for each 8 array elements, i.e., (36 + p)/8 ops per
word, where p is the number of ops for one population count. For p = 15
(code of Fig. 5-2, inlined) this is 51/8 = 6.375 ops per word. */
pub fn counts_popArray5(A: & mut [i32], n: i32) -> i32 {
    let mut ones = 0;
    let mut twos = 0;
    let mut twosA = 0;
    let mut twosB = 0;
    let mut fours = 0;
    let mut foursA = 0;
    let mut foursB = 0;
    let mut eights = 0;
    let mut tot = 0;
    let mut i: usize = 0;
    loop {
        i += 8;
        if i <= (n - 8) as usize {
            break;
        }
        CSA!(twosA, ones, ones, A[i], A[i+1]);
        CSA!(twosB, ones, ones, A[i+2], A[i+3]);
        CSA!(foursA, twos, twos, twosA, twosB);
        CSA!(twosA, ones, ones, A[i+4], A[i+5]);
        CSA!(twosB, ones, ones, A[i+6], A[i+7]);
        CSA!(foursB, twos, twos, twosA, twosB);
        CSA!(eights, fours, fours, foursA, foursB);
        tot = tot + counts_pop(eights as i64);
    }
    tot = 8 * tot + 4 * counts_pop(fours as i64) + 2 * counts_pop(twos as i64) + counts_pop(ones as i64);

    for t in i..(n as usize) {     // Simply add in the last
        tot = tot + counts_pop(A[t] as i64);
    }    // 0 to 7 elements.
    return tot as i32;
}

/* This function generalizes the pattern illustrated by the function
above, with the result that the bits in an n-word array can be counted
with ceil(log2(n+3)) evaluations of population count.
   The inner loop (with the CSA) is done very close to 2 times for each
outer loop iteration. This is based on both a mathematical calculation
(which shows something less than 9/4 times) and instrumentation (e.g.,
for n = 10,000 the inner loop is executed 9983 times). The inner loop
compiles into 19 instructions, mostly housekeeping (shifts, adds loads,
stores). This results in a time of 2*19 + 19 = 57 instructions for each
outer loop iteration, or 28.5 instructions per word of the array.
This is worse than the naive method, and MUCH worse than the above
program, which compiles into 8.0 instructions/word. So this routine is
a bad idea unless the time to do a population count on one word is very
large (greater than 30 instructions anyway).
   Haven't timed the other loop, but it is executed only about log2(n)
times and so isn't so important.
   Therefore, if this method is to be useful, the housekeeping steps
must be greatly reduced. It may be possible to do this by "unwinding"
the first few inner loop iterations. Not sure how good the result would
be. */
pub fn counts_popArray6(A: &mut [i32], n: i32) -> i32 {
    let mut tot = 0;
    let mut i: usize = 0;
    let mut k = 0;
    let mut z = 0;
    let mut hi = 0;
    let mut lo = 0;
    let mut nrow = [0; 30];
    let mut sum = [[0, 0]; 30];

    nrow[0] = 1;                 // a fake 0-element.

    loop {
        i += 2;
        if i <= (n - 2) as usize {
            break;
        }
        sum[0][1] = A[i];
        z = A[i + 1];
        k = 0;
        loop {
            CSA!(z, sum[k][0], sum[k][0], sum[k][1], z);
            nrow[k] = 1;
            k = k + 1;
            if nrow[k] != 2 {
                break;
            }
        }
        sum[k][nrow[k]] = z;
        nrow[k] = nrow[k] + 1;
    }

    if i == (n - 1) as usize {            // If there's one more in
        sum[0][1] = A[i];         // the array, put it in
        nrow[0] = 2;              // sum[0][1].
    }

    /* Make a pass over the "sum" array compressing all
    rows that have two entries to the same row but having
    only one entry, while adding an entry to the
    subsequent row. This can make the subsequent row have,
    in effect, three entries, which we similarly compress.
    Compute the total during this pass. When an empty row
    is encountered, we're done. */
    tot = 0;
    hi = 0;
    loop {
        k += 1;
        if nrow[k] == 0 {
            break;
        }
        if nrow[k] == 1 { z = 0; } else { z = sum[k][1]; } // (Is 2.)

        CSA!(hi, lo, sum[k][0], z, hi);
        tot = tot + (counts_pop(lo as i64) << k);
    }

    tot = tot + (counts_pop(hi as i64) << k);

    return tot as i32;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
#[allow(overflowing_literals)]
fn test_counts_popArray6() {
    let mut A = [0;101];
    let mut A1 = [0;101];
    A[0]=0xFFFFFFFF;
    A1[0]=0xFFFFFFFF;
    A[1]=5;
    A1[1]=5;
    for i in 2..101 {
        let result = counts_gen_i32();
        A[i]=result;
        A1[i]=result as i64;
    }

    let s1 = counts_pop_array(A1.borrow_mut(), 101);
    assert_eq!(counts_popArray1(A.borrow_mut(), 101), s1 as i32);
    // assert_eq!(counts_popArray2(A.clone(),101) , s1 as i32);
    // assert_eq!(counts_popArray3(A.clone(),101) , s1 as i32);
    // assert_eq!(counts_popArray4(A.clone(),101) , s1 as i32);
    // assert_eq!(counts_popArray5(A.clone(),101) , s1 as i32);
    // assert_eq!(counts_popArray6(A,101), s1 as i32);
}

pub fn counts_pop_array(A: & mut [i64], n: i64) -> i64 {
    let mut s = 0;
    let mut x;
    for i in (0..n).step_by(31) {
        #[cfg(target_arch = "riscv64")]
            let lim = core::cmp::min(n, i + 31);
        #[cfg(target_arch = "x86_64")]
            let lim = std::cmp::min(n, i + 31);
        let mut s8 = 0;
        for j in i..lim {
            x = A[j as usize];
            x = x - ((x >> 1) & 0x55555555);
            x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
            x = (x + (x >> 4)) & 0x0F0F0F0F;
            s8 = s8 + x;
        }
        x = (s8 & 0x00FF00FF) + ((s8 >> 8) & 0x00FF00FF);
        x = (x & 0x0000ffff) + (x >> 16);
        s = s + x;
    }
    return s;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts_pop_aray() {
    assert_eq!(counts_pop_array([1, 2].borrow_mut(), 1), 1);
}

pub fn counts_popCmpr(mut xp: i32, mut yp: i32) -> i32 {
    let mut x = xp & !yp;                // Clear bits where
    let mut y = yp & !xp;                // both are 1.
    loop {
        if x == 0 { return y | -y; }
        if y == 0 { return 1; }
        x = x & (x - 1);          // Clear one bit
        y = y & (y - 1);          // from each.
    }
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts_popCmpr() {
    assert_eq!(counts_popCmpr(0, 1), -1);
}

/* Given a word size n, this program computes, by simulation, the
average minimal population count of two random n-bit words each of which
has been AND'ed with the complement of the other so that the words are
never both 1 in the same bit position. This is necessary to estimate the
running time (number of loop iterations) of an algorithm that determines
which of two words has the lower population count.
   The true averages, obtained by running an exhaustive test, are:

   n    true average
   2    0.125
   3    0.28125
   4    0.453125
   5    0.634766
   6    0.823242
   7    1.016846
   8    1.214478
   9    1.415382
  10    1.619015
  11    1.824965
  12    2.032918
  13    2.242623
  14    2.453878
  15    2.666517
  16    2.880401
  17    3.095413 12 min on Thinkpad model T30
  24    4.625    Simulated, 100,000,000 trials
  32    6.186    Simulated, 100,000,000 trials
*/
pub fn counts_popCmprAvgSim() -> f32 {
    let n = 10;

    let ntrials = 10000;

    let mask = 2 * (1 << (n - 1)) - 1;
    let mut tot = 0;

    for i in 0..ntrials {
        let xp: i32 = counts_gen_i32() & mask;
        let yp: i32 = counts_gen_i32() & mask;
        let x = xp & !yp;
        let y = yp & !xp;
        let nbx = counts_pop(x as i64);
        let nby = counts_pop(y as i64);
        let mut nb = 0;
        if nbx < nby { nb = nbx } else { nb = nby };
        tot = tot + nb;
    }
    tot as f32 / ntrials as f32
}

pub fn counts_parity1(mut x: u32) -> u32 {
    let mut y = x ^ (x >> 1);
    y = y ^ (y >> 2);
    y = y ^ (y >> 4);
    y = y ^ (y >> 8);
    y = y ^ (y >> 16);
    return y & 1;
}

pub fn counts_parity1a(mut x: u32) -> u32 {
    let mut y = x ^ (x >> 16);
    y = y ^ (y >> 8);
    y = y ^ (y >> 4);
    y = 0x6996 >> (y & 0xF);     // Falk Hueffner's trick.
    return y & 1;
}

pub fn counts_parity2(mut x: u32) -> u32 {
    x = x ^ (x >> 1);
    x = (x ^ (x >> 2)) & 0x11111111;
    x = x * 0x11111111;
    let p = (x >> 28) & 1;
    return p;
}

#[allow(overflowing_literals)]
pub fn counts_parity3(mut x: i32) -> i32 {
    let y = (x.wrapping_mul(0x10204081)) & 0x888888FF;
    return (y % 1920) & 0xFF;      // Returns a byte with even parity.
}

pub fn counts_parity4(mut x: u32) -> u32 {
    let mut y = (x * 0x00204081) | 0x3DB6DB00;
    y = (y % 1152) & 0xFF;
    return y ^ 0x80;             // Change to even parity so test2 can be used.
}

pub fn counts_gen_i32() -> i32 {
    IsaacRng::gen::<i32>(&mut IsaacRng::seed_from_u64(IsaacRng::next_u64(
        &mut IsaacRng::seed_from_u64(SEED.fetch_add(1, Ordering::Relaxed) as u64),
    )))
}
