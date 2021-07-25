/// Error Correction Code
pub fn hamming_parity(mut x: i64) -> i64 {
    x = x ^ (x >> 1);
    x = x ^ (x >> 2);
    x = x ^ (x >> 4);
    x = x ^ (x >> 8);
    x = x ^ (x >> 16);
    return x & 1;
}

/* Computes the six parity check bits for the
"information" bits given in the 32-bit word u. The
check bits are p[5:0]. On sending, an overall parity
bit will be prepended to p (by another process).

Bit   Checks these bits of u
p[0]  0, 1, 3, 5, ..., 31 (0 and the odd positions).
p[1]  0, 2-3, 6-7, ..., 30-31 (0 and positions xxx1x).
p[2]  0, 4-7, 12-15, 20-23, 28-31 (0 and posns xx1xx).
p[3]  0, 8-15, 24-31 (0 and positions x1xxx).
p[4]  0, 16-31 (0 and positions 1xxxx).
p[5]  1-31 */
pub fn hamming_checkbits(mut u: i64) -> i64 {

// First calculate p[5:0] ignoring u[0].
    let mut p0 = u ^ (u >> 2);
    p0 = p0 ^ (p0 >> 4);
    p0 = p0 ^ (p0 >> 8);
    p0 = p0 ^ (p0 >> 16);        // p0 is in posn 1.

    let t1 = u ^ (u >> 1);
    let mut p1 = t1 ^ (t1 >> 4);
    p1 = p1 ^ (p1 >> 8);
    p1 = p1 ^ (p1 >> 16);        // p1 is in posn 2.

    let t2 = t1 ^ (t1 >> 2);
    let mut p2 = t2 ^ (t2 >> 8);
    p2 = p2 ^ (p2 >> 16);        // p2 is in posn 4.

    let t3 = t2 ^ (t2 >> 4);
    let p3 = t3 ^ (t3 >> 16);        // p3 is in posn 8.

    let p4 = t3 ^ (t3 >> 8);         // p4 is in posn 16.

    let p5 = p4 ^ (p4 >> 16);        // p5 is in posn 0.

    let mut p = ((p0 >> 1) & 1) | ((p1 >> 1) & 2) | ((p2 >> 2) & 4) |
        ((p3 >> 5) & 8) | ((p4 >> 12) & 16) | ((p5 & 1) << 5);

    p = p ^ (-(u & 1) & 0x3F);   // Now account for u[0].
    return p;
}


/* This function looks at the received seven check
bits and 32 information bits (pr and ur), and
determines how many errors occurred (under the
presumption that it must be 0, 1, or 2). It returns
with 0, 1, or 2, meaning that no errors, one error, or
two errors occurred. It corrects the information word
received (ur) if there was one error in it. */
pub fn hamming_correct(pr: i64, ur: &mut i64) -> i64 {
    let po = hamming_parity(pr ^ *ur);       // Compute overall parity
// of the received data.
    let p = hamming_checkbits(*ur);          // Calculate check bits
// for the received info.
    let mut syn = p ^ (pr & 0x3F);       // Syndrome (exclusive of
// overall parity bit).
    if po == 0 {
        if syn == 0 { return 0; }   // If no errors, return 0.
        else { return 2; }            // Two errors, return 2.
    }
// One error occurred.
    if ((syn - 1) & syn) == 0  // If syn has zero or one
    { return 1; }                 // bits set, then the
// error is in the check
// bits or the overall
// parity bit (no
// correction required).

// One error, and syn bits 5:0 tell where it is in ur.

    let b = syn - 31 - (syn >> 5); // Map syn to range 0 to 31.
// if (syn == 0x1f) b = 0;    // (These two lines equiv.
// else b = syn & 0x1f;       // to the one line above.)
    *ur = *ur ^ (1 << b);      // Correct the bit.
    return 1;
}


pub fn hamming_perturb(p: &mut i64, u: &mut i64) -> i64 {
    /* This generates all the possible 39-bit quantities with 0, 1, or 2
    bits set, and alters the corresponding 0, 1, or 2 bits of p and u,
    treating them as a concatenation of p and u (39 bits long).
       The error bit words are generated in the order (illustrated for a
    5-bit quantitity):

    00011, 00101, 01001, 10001, 00001, 00110, 01010, 10010, 00010,
    01100, 10100, 00100, 11000, 01000, 10000, 00000. */

    let mask: i64 = (1 << 39) - 1;
    let mut x = 1;
    let mut y = 2;
    let errorBits;
    let num;

    errorBits = x | y;

    if errorBits == 0 { num = 0; }        // Set num = number
    else if x == 0 || y == 0
    { num = 1; }                          // of 1-bits in
    else { num = 2; }                     // errorBits.

    *u = *u ^ errorBits;                 // Apply the
    *p = *p ^ (errorBits >> 32);         // error bits.

    if y != 0 { y = (y << 1) & mask; } else {
        x = (x << 1) & mask;
        y = (x << 1) & mask;
    }
    return num;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_hamming() {
    assert_eq!(hamming_parity(1), 1);
}