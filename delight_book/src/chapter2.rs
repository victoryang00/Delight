use crate::*;

const SLOW_MUL: bool = true;

/// turn off the rightmost 1-bit in a word, producing 0 if none
pub fn basics_get_and(x: c8) -> c8 {
    x & (x - c8(1))
}

/// turn on the rightmost 0-bit in a word, producing all 1’s if none
pub fn basics_get_or(x: c8) -> c8 {
    x | (x + c8(1))
}

/// turn off the trailing 1’s in a word, producing x if none
pub fn basics_all_right_1_to_0(x: c8) -> c8 {
    x & (x + c8(1))
}

/// turn on the trailing 0’s in a word, producing x if none
pub fn basics_all_right_0_to_1(x: c8) -> c8 {
    x | (x - c8(1))
}

/// create a word with a single 1-bit at the position of the rightmost 0-bit in x
pub fn basics_single_0_right_0(x: c8) -> c8 {
    !x & (x + c8(1))
}

/// create a word with a single 0-bit at the position of the rightmost 1-bit in x, producing all 1’s if none
pub fn basics_single_0_right_1(x: c8) -> c8 {
    !x | (x - c8(1))
}

/// create a word with 1’s at the positions of the trailing 0’s in x, and 0’s elsewhere, producing 0 if none, have 3 of them
pub fn basics_trailing_0_1(x: c8) -> c8 {
    !(x | (c8(0) - x))
}

pub fn basics_trailing_0_2(x: c8) -> c8 {
    !x & (x - c8(1))
}

pub fn basics_trailing_0_3(x: c8) -> c8 {
    (x & (c8(0) - x)) - c8(1)
}

/// create a word with 0’s at the positions of the trailing 1’s in x, and 0’s elsewhere, producing all 1’s if none
pub fn basics_trailing_1(x: c8) -> c8 {
    !x | (x + c8(1))
}

/// isolate the rightmost 1-bit, producing 0 if none (e.g.,
pub fn basics_isolate_1(x: c8) -> c8 {
    x & (c8(0) - x)
}

/// create a word with 1’s at the positions of the rightmost 1-bit and the trailing 0’s in x, producing all 1’s if no 1-bit, and the integer 1 if no trailing 0’s
pub fn basics_get_right0_trailing1(x: c8) -> c8 {
    x ^ (x - c8(1))
}

/// create a word with 1’s at the positions of the rightmost 0-bit and the trailing 1’s in x, producing all 1’s if no 0-bit, and the integer 1 if no trailing 1’s
pub fn basics_get_right1_trailing0(x: c8) -> c8 {
    x ^ (x + c8(1))
}

/// turn off the rightmost contiguous string of 1’s
pub fn basics_turnoff_rightmost1(x: c8) -> c8 {
    ((x | (x - c8(1))) + c8(1)) & x
}

pub fn basics_turnoff_rightmost2(x: c8) -> c8 {
    ((x & (c8(0) - x)) + x) & x
}

/// De Morgan’s laws
pub fn basics_demorgan_1(x: c8, y: c8) -> bool {
    !(x & y) == !x | !y
}

pub fn basics_demorgan_2(x: c8, y: c8) -> bool {
    !(x | y) == !x & !y
}

pub fn basics_demorgan_3(x: c8) -> bool {
    !(x + c8(1)) == !x - c8(1)
}

pub fn basics_demorgan_4(x: c8) -> bool {
    !(x - c8(1)) == !x + c8(1)
}

pub fn basics_demorgan_5(x: c8) -> bool {
    !(c8(0) - x) == x - c8(1)
}

pub fn basics_demorgan_6(x: c8, y: c8) -> bool {
    (!(x ^ y) == !x ^ y) && ((x.xnor(y)) == (!x ^ y))
}

pub fn basics_demorgan_7(x: c8, y: c8) -> bool {
    (!(x.xnor(y)) == !x.xnor(y)) && ((x ^ y) == (!x.xnor(y)))
}

pub fn basics_demorgan_8(x: c8, y: c8) -> bool {
    !(x + y) == !x - y
}

pub fn basics_demorgan_9(x: c8, y: c8) -> bool {
    !(x - y) == !x + y
}

#[test]
fn test_basics1() {
    assert_eq!(basics_get_and(c8(0b01011000)), c8(0b001010000));
    assert_eq!(basics_get_or(c8(0b10100111)), c8(0b10101111));
    assert_eq!(basics_all_right_1_to_0(c8(0b10100111)), c8(0b10100000));
    assert_eq!(basics_all_right_0_to_1(c8(0b10101000)), c8(0b10101111));
    assert_eq!(basics_single_0_right_0(c8(0b10100111)), c8(0b00001000));
    assert_eq!(basics_single_0_right_1(c8(0b10101000)), c8(0b11110111));
    assert_eq!(basics_trailing_0_1(c8(0b01011000)), c8(0b00000111));
    assert_eq!(basics_trailing_0_2(c8(0b01011000)), c8(0b00000111));
    assert_eq!(basics_trailing_0_3(c8(0b01011000)), c8(0b00000111));
    assert_eq!(basics_trailing_1(c8(0b10100111)), c8(0b11111000));
    assert_eq!(basics_isolate_1(c8(0b01011000)), c8(0b00001000));
    assert_eq!(basics_get_right0_trailing1(c8(0b01011000)), c8(0b00001111));
    assert_eq!(basics_get_right1_trailing0(c8(0b01010111)), c8(0b00001111));
    assert_eq!(basics_turnoff_rightmost1(c8(0b01011100)), c8(0b01000000));
    assert_eq!(basics_turnoff_rightmost2(c8(0b01011100)), c8(0b01000000));
    assert!(basics_demorgan_1(c8(0b01011100), c8(0b01000000)));
    assert!(basics_demorgan_2(c8(0b01011100), c8(0b01000000)));
    assert!(basics_demorgan_3(c8(0b01011100)));
    assert!(basics_demorgan_4(c8(0b01011100)));
    assert!(basics_demorgan_5(c8(0b01011100)));
    assert!(basics_demorgan_6(c8(0b01011100), c8(0b01000000)));
    assert!(basics_demorgan_7(c8(0b01011100), c8(0b01000000)));
    assert!(basics_demorgan_8(c8(0b01011100), c8(0b01000000)));
    assert!(basics_demorgan_9(c8(0b01011100), c8(0b01000000)));
}

/// If division is slow but you have a fast way to compute the number of trailing
/// zeros function ntz(x), the number of leading zeros function nlz(x), or population
/// count (pop(x) is the number of 1-bits in x)
pub fn ntz(mut x: u32) -> u32 {
    if x == 0 { return 32; }
    let mut n = 1;
    if (x & 0x0000FFFF) == 0 {
        n = n + 16;
        x = x >> 16;
    }
    if (x & 0x000000FF) == 0 {
        n = n + 8;
        x = x >> 8;
    }
    if (x & 0x0000000F) == 0 {
        n = n + 4;
        x = x >> 4;
    }
    if (x & 0x00000003) == 0 {
        n = n + 2;
        x = x >> 2;
    }
    n - (x & 1)
}

pub fn nlz(mut x: u32) -> u32 {
    if x == 0 { return 32; }
    let mut n = 0;
    if x <= 0x0000FFFF {
        n = n + 16;
        x = x << 16;
    }
    if x <= 0x00FFFFFF {
        n = n + 8;
        x = x << 8;
    }
    if x <= 0x0FFFFFFF {
        n = n + 4;
        x = x << 4;
    }
    if x <= 0x3FFFFFFF {
        n = n + 2;
        x = x << 2;
    }
    if x <= 0x7FFFFFFF { n = n + 1; }
    return n;
}

pub fn pop(mut x: u32) -> u32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x << 8);
    x = x + (x << 16);
    x >> 24
}


pub fn ntz1(mut x: u32) -> u32 {
    return 32 - nlz(!x & (x - 1));
}

pub fn ntz2(mut x: u32) -> u32 {
    return pop(!x & (x - 1));
}

pub fn ntz3(mut x: u32) -> u32 {
    let mut n;

    if (x == 0) { return (32); }
    n = 1;
    if ((x & 0x0000FFFF) == 0) {
        n = n + 16;
        x = x >> 16;
    }
    if ((x & 0x000000FF) == 0) {
        n = n + 8;
        x = x >> 8;
    }
    if ((x & 0x0000000F) == 0) {
        n = n + 4;
        x = x >> 4;
    }
    if ((x & 0x00000003) == 0) {
        n = n + 2;
        x = x >> 2;
    }
    return n - (x & 1);
}

pub fn ntz4(mut x: u32) -> u32 {
    let mut y = 0;
    let mut n = 0;

    if (x == 0) { return 32; }
    n = 31;
    y = x << 16;
    if (y != 0) {
        n = n - 16;
        x = y;
    }
    y = x << 8;
    if (y != 0) {
        n = n - 8;
        x = y;
    }
    y = x << 4;
    if (y != 0) {
        n = n - 4;
        x = y;
    }
    y = x << 2;
    if (y != 0) {
        n = n - 2;
        x = y;
    }
    y = x << 1;
    if (y != 0) { n = n - 1; }
    return n;
}

pub fn ntz4a(mut x: u32) -> u32 {
    let mut y;
    let mut n;

    if (x == 0) { return 32; }
    n = 31;
    y = x << 16;
    if (y != 0) {
        n = n - 16;
        x = y;
    }
    y = x << 8;
    if (y != 0) {
        n = n - 8;
        x = y;
    }
    y = x << 4;
    if (y != 0) {
        n = n - 4;
        x = y;
    }
    y = x << 2;
    if (y != 0) {
        n = n - 2;
        x = y;
    }
    n = n - ((x << 1) >> 31);
    return n;
}

pub fn ntz5(mut x: u32) -> u32 {
    if (x & 15 > 0) {
        if (x & 3 > 0) {
            if (x & 1 > 0) { return 0; } else { return 1; }
        } else if (x & 4 > 0) { return 2; } else { return 3; }
    } else if (x & 0x30 > 0) {
        if (x & 0x10 > 0) { return 4; } else { return 5; }
    } else if (x & 0x40 > 0) { return 6; } else if (x > 0) { return 7; } else { return 8; }
}

pub fn ntz6(mut x: u32) -> u32 {
    let mut n;

    x = !x & (x - 1);
    n = 0;                       // n = 32;
    while (x != 0) {              // while (x != 0) {
        n = n + 1;                //    n = n - 1;
        x = x >> 1;               //    x = x + x;
    }                            // }
    return n;                    // return n;
}

pub fn ntz6a(mut x: u32) -> u32 {
    let mut n;

    n = 32;
    while (x != 0) {
        n = n - 1;
        x = x + x;
    }
    return n;
}

/* Dean Gaudet's algorithm. To be most useful there must be a good way
to evaluate the C "conditional expression" (a?b:c construction) without
branching. The result of a?b:c is b if a is true (nonzero), and c if a
is false (0).
   For example, a compare to zero op that sets a target GPR to 1 if the
operand is 0, and to 0 if the operand is nonzero, will do it. With this
instruction, the algorithm is entirely branch-free. But the most
interesting thing about it is the high degree of parallelism. All six
lines with conditional expressions can be executed in parallel (on a
machine with sufficient computational units).
   Although the instruction count is 30 measured statically, it could
execute in only 10 cycles on a machine with sufficient parallelism.
   The first two uses of y can instead be x, which would increase the
useful parallelism on most machines (the assignments to y, bz, and b4
could then all run in parallel). */
pub fn ntz7(mut x: i32) -> i32 {
    let y;
    let bz;
    let b4;
    let b3;
    let b2;
    let b1;
    let b0;

    y = x & -x;               // Isolate rightmost 1-bit.
    bz = if y > 0 { 0 } else { 1 };           // 1 if y = 0.
    b4 = if y & 0x0000FFFF > 0 { 0 } else { 16 };
    b3 = if y & 0x00FF00FF > 0 { 0 } else { 8 };
    b2 = if y & 0x0F0F0F0F > 0 { 0 } else { 4 };
    b1 = if y & 0x33333333 > 0 { 0 } else { 2 };
    b0 = if y & 0x55555555 > 0 { 0 } else { 1 };
    return bz + b4 + b3 + b2 + b1 + b0;
}

/* Below is David Seal's algorithm, found at
http://www.ciphersbyritter.com/NEWS4/BITCT.HTM Table
entries marked "u" are unused. 6 ops including a
multiply, plus an indexed load. */
pub fn ntz8(mut x: i32) -> i32 {
    let table =
        vec![32, 0, 1, 12, 2, 6, 99, 13, 3, 99, 7, 99, 99, 99, 99, 14,
             10, 4, 99, 99, 8, 99, 99, 25, 99, 99, 99, 99, 99, 21, 27, 15,
             31, 11, 5, 99, 99, 99, 99, 99, 9, 99, 99, 24, 99, 99, 20, 26,
             30, 99, 99, 99, 99, 23, 99, 19, 29, 99, 22, 18, 28, 17, 16, 99];

    x = (x & -x) * 0x0450FBAF;
    return table[(x >> 26) as usize];
}

/* Seal's algorithm with multiply expanded.
9 elementary ops plus an indexed load. */
pub fn ntz8a(mut x: i32) -> i32 {
    let table =
        vec![32, 0, 1, 12, 2, 6, 99, 13, 3, 99, 7, 99, 99, 99, 99, 14,
             10, 4, 99, 99, 8, 99, 99, 25, 99, 99, 99, 99, 99, 21, 27, 15,
             31, 11, 5, 99, 99, 99, 99, 99, 9, 99, 99, 24, 99, 99, 20, 26,
             30, 99, 99, 99, 99, 23, 99, 19, 29, 99, 22, 18, 28, 17, 16, 99];

    x = (x & -x);
    x = (x << 4) + x;    // x = x*17.
    x = (x << 6) + x;    // x = x*65.
    x = (x << 16) - x;   // x = x*65535.
    return table[(x >> 26) as usize];
}

/* Reiser's algorithm. Three ops including a "remainder,"
plus an indexed load. */
pub fn ntz9(mut x: i32) -> i32 {
    let table = vec![32, 0, 1, 26, 2, 23, 27,
                     99, 3, 16, 24, 30, 28, 11, 99, 13, 4,
                     7, 17, 99, 25, 22, 31, 15, 29, 10, 12,
                     6, 99, 21, 14, 9, 5, 20, 8, 19, 18];

    x = (x & -x) % 37;
    return table[x as usize];
}

/* Using a de Bruijn sequence. This is a table lookup with a 32-entry
table. The de Bruijn sequence used here is
                0000 0100 1101 0111 0110 0101 0001 1111,
obtained from Danny Dube's October 3, 1997, posting in
comp.compression.research. Thanks to Norbert Juffa for this reference. */
pub fn ntz10(mut x: i32) -> i32 {
    let table =
        vec![0, 1, 2, 24, 3, 19, 6, 25, 22, 4, 20, 10, 16, 7, 12, 26,
             31, 23, 18, 5, 21, 9, 15, 11, 30, 17, 8, 14, 29, 13, 28, 27];

    if (x == 0) { return 32; }
    x = (x & -x) * 0x04D7651F;
    return table[(x >> 27) as usize];
}

/* Norbert Juffa's code, answer to exercise 1 of Chapter 5 (2nd ed). */
pub fn ntz11(mut n: i32) -> i32 {
    let tab =
        vec![0, 1, 2, 24, 3, 19, 6, 25,
             22, 4, 20, 10, 16, 7, 12, 26,
             31, 23, 18, 5, 21, 9, 15, 11,
             30, 17, 8, 14, 29, 13, 28, 27
        ];
    let mut k = 0;
    n = n & (-n);        /* isolate lsb */
    if SLOW_MUL {
        k = (n << 11) - n;
        k = (k << 2) + k;
        k = (k << 8) + n;
        k = (k << 5) - k;
    } else {
        k = n * 0x4d7651f;
    }
    return if n > 0 { tab[(k >> 27) as usize] } else { 32 };
}

#[test]
fn test_basics2() {
    assert_eq!(ntz(12), 2);
    assert_eq!(ntz1(12), 2);
    assert_eq!(ntz2(12), 2);
    assert_eq!(ntz3(12), 2);
    assert_eq!(ntz4(12), 2);
    assert_eq!(ntz5(12), 2);
    assert_eq!(ntz6(12), 2);
    assert_eq!(ntz7(12), 2);
    assert_eq!(ntz8(12), 2);
    assert_eq!(ntz9(12), 2);
    assert_eq!(ntz10(12), 2);
    assert_eq!(ntz11(12), 2);
    // assert_eq!(ntz(12), 2);

    assert_eq!(ntz(123), 0);
    assert_eq!(ntz(21), 0);
    assert_eq!(nlz(21), 27);
    assert_eq!(nlz(123), 25);
    assert_eq!(nlz(13), 28);
    assert_eq!(pop(13), 3);
    assert_eq!(pop(3), 2);
    assert_eq!(pop(34), 2);
}

/// Given a word x that represents a subset, the idea is to find the rightmost
/// contiguous group of 1’s in x and the following 0’s, and “increment” that quantity to the
/// next value that has the same number of 1’s.
pub fn snoob(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut ripple = 0;
    let mut ones = 0;
    // x = xxx0 1111 0000
    smallest = x & -x;           //     0000 0001 0000
    ripple = x + smallest;       //     xxx1 0000 0000
    ones = x ^ ripple;           //     0001 1111 0000
    ones = (ones >> 2) / smallest; //     0000 0000 0111
    return ripple | ones;        //     xxx1 0000 0111
}

pub fn snoob1(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut ripple = 0;
    let mut ones = 0;
// x = xxx0 1111 0000
    smallest = x & -x;           //     0000 0001 0000
    ripple = x + smallest;       //     xxx1 0000 0000
    ones = x ^ ripple;           //     0001 1111 0000
    ones = ones >> (2 + ntz(x as u32) as i32); //     0000 0000 0111
    return ripple | ones;        //     xxx1 0000 0111
}

/* Variation 2: nlz to avoid division.  Nine ops. */
pub fn snoob2(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut ripple = 0;
    let mut ones = 0;
// x = xxx0 1111 0000
    smallest = x & -x;           //     0000 0001 0000
    ripple = x + smallest;       //     xxx1 0000 0000
    ones = x ^ ripple;           //     0001 1111 0000
    ones = ones >> (33 - nlz(smallest as u32)); // 0000 0000 0111
    return ripple | ones;        //     xxx1 0000 0111
}

/* Variation 3: pop to avoid division.  Nine ops. */
pub fn snoob3(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut ripple = 0;
    let mut ones = 0;
// x = xxx0 1111 0000
    smallest = x & -x;           //     0000 0001 0000
    ripple = x + smallest;       //     xxx1 0000 0000
    ones = x ^ ripple;           //     0001 1111 0000
    ones = (1 <<                 //     0000 0000 0111
        (pop(ones as u32) - 2)) - 1;
    return ripple | ones;        //     xxx1 0000 0111
}

/* The version below is from Harbison & Steele Fourth Ed. section 7.6.7
(p. 215).  Nine ops, not counting the "if" statement. */
pub fn next_set_of_n_elements(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut new_smallest = 0;
    let mut ripple = 0;
    let mut ones = 0;

    if x == 0 { return 0; }
    smallest = (x & -x);
    ripple = x + smallest;
    new_smallest = (ripple & -ripple);
    ones = ((new_smallest / smallest) >> 1) - 1;
    return ripple | ones;
}

/* I ran into this next version in October 2007 on the TopCoder web site:
http://forums.topcoder.com/?module=Message&messageID=574258
   The author is David de Kloet. The variable x must be signed,
or after generating a correct sequence, it will generate some incorrect
values and then loop forever. Must have x != 0. After generating the
sequence, it generates 0xFFFFFFFF and sticks at that value.
   The number of shifts done in the while-loop is equal to the number of
trailing zeros in the input x. So what is the average? I don't know,
will work on it. The values of x are NOT random, they tend to have more
trailing 0's than purely random numbers would have. (For uniformly
distributed random numbers, the average is 1.)
   For a word size of 32, if n (the number of 1-bits) = 1, the average
number of trailing 0's is 15.5 (average of the numbers from 0 t 31). For
n = 2, the average is 10 (I think). It gets lower for higher values of
n. */
pub fn snoob4(mut x: i32) -> i32 {
    let mut y = x + (x & -x);
    x = x & !y;
    while ((x & 1) == 0) { x = x >> 1; }
    x = x >> 1;
    return y | x;
}

#[test]
fn test_basics3() {
    assert_eq!(snoob(0b111011110000), 0b111100000111);
    assert_eq!(snoob1(0b111011110000), 0b111100000111);
    assert_eq!(snoob2(0b111011110000), 0b111100000111);
    assert_eq!(snoob3(0b111011110000), 0b111100000111);
    assert_eq!(snoob4(0b111011110000), 0b111100000111);
    assert_eq!(next_set_of_n_elements(0b111011110000), 0b111100000111);
}

// fn multover(mut x: i32, mut y: i32, mut z: i32, mut m: i32, mut n: i32, mut t: i32) -> i32 {
//     m = nlz(x);
//     n = nlz(y);
//     if (m + n <= 30)
//     goto
//     overflow;
//     t = x * (y >> 1);
//     if ((int)
//     t < 0) goto
//     overflow;
//     z = t * 2;
//     if (y & 1) {
//         z = z + x;
//         if (z < x)
//         goto
//         overflow;
//     }
//
//
//     println!("Overflows\n");
// }
