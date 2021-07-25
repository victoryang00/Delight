use chapter2::basics_nlz;
#[cfg(target_arch = "x86_64")]
use std::borrow::BorrowMut;
#[cfg(target_arch = "riscv64")]
use core::borrow::BorrowMut;

/// Integer Divisions

/* q[0], r[0], u[0], and v[0] contain the LEAST significant halfwords.
(The sequence is in little-endian order).

This first version is a fairly precise implementation of Knuth's
Algorithm D, for a binary computer with base b = 2**16.  The caller
supplies
   1. Space q for the quotient, m - n + 1 halfwords (at least one).
   2. Space r for the remainder (optional), n halfwords.
   3. The dividend u, m halfwords, m >= 1.
   4. The divisor v, n halfwords, n >= 2.
The most significant digit of the divisor, v[n-1], must be nonzero.  The
dividend u may have leading zeros; this just makes the algorithm take
longer and makes the quotient contain more leading zeros.  A value of
NULL may be given for the address of the remainder to signify that the
caller does not want the remainder.
   The program does not alter the input parameters u and v.
   The quotient and remainder returned may have leading zeros.  The
function itself returns a value of 0 for success and 1 for invalid
parameters (e.g., division by 0).
   For now, we must have m >= n.  Knuth's Algorithm D also requires
that the dividend be at least as long as the divisor.  (In his terms,
m >= 0 (unstated).  Therefore m+n >= n.) */
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn division_divmnu(q: &mut [u8], r: &mut [u8], u: &[u8], v: &[u8], m: i32, n: i32) -> i32 {
    let b:u16 = 65536; // Number base (16 bits).

    if m < n || n <= 0 || v[(n - 1)as usize] == 0 {
        return 1;              // Return if invalid param.
    }
    if n == 1 {                        // Take care of
        let mut k = 0;                            // the case of a
        for j in m - 1..0 {    // single-digit
            q[j as usize] = ((k * b + u[j as usize] as u16) / v[0] as u16) as u8;      // divisor here.
            k = (k * b + u[j as usize] as u16) - q[j as usize] as u16 * v[0] as u16;
        }
        if r != &[] {
            r[0] = k as u8;
        }
        return 0;
    }

// Normalize by shifting v left just enough so that
// its high-order bit is on, and shift u left the
// same amount.  We may have to append a high-order
// digit on the dividend; we do that unconditionally.
    let mut s = basics_nlz(v[(n - 1) as usize] as u32) - 16;        // 0 <= s <= 15.
    let mut vn = [0;100000];
    for i in n - 1..0 {
        vn[i as usize] = (v[i as usize] << s) | (v[i as usize - 1] >> 16 - s);
    }
    vn[0] = v[0] << s;
    let mut un = [0;100000];

    un[m as usize] = u[m as usize - 1] >> 16 - s;
    for i in m - 1..0 {
        un[i as usize] = (u[i as usize] << s) | (u[i as usize - 1] >> 16 - s);
    }
    un[0] = u[0] << s;

    for j in m-n..0 {       // Main loop.
// Compute estimate qhat of q[j].
        let mut qhat = (un[(j + n) as usize] * b as u8 + un[(j + n) as usize - 1]) / vn[n as usize - 1];
        let mut rhat = (un[(j + n) as usize] * b as u8 + un[(j + n) as usize - 1]) - qhat * vn[n as usize- 1];
        loop {
            if qhat >= b as u8 || qhat * vn[n as usize - 2] > (b * rhat as u16 + un[(j + n)as usize - 2] as u16)as u8 {
                qhat = qhat - 1;
                rhat = rhat + vn[n as usize - 1];
                if rhat >= b as u8{
                    break;
                }
            }
        }
// Multiply and subtract.
        let mut k = 0;
        for i in 0..n {
            let p = qhat * vn[i as usize];
            let t = un[(i + j) as usize] - k - (p & 0xFFFF);
            un[(i + j)as usize] = t;
            k = (p >> 16) - (t >> 16);
        }
        let mut t = un[(j + n)as usize] - k;
        un[(j + n)as usize] = t;

        q[j as usize] = qhat;              // Store quotient digit.
        if t < 0 {              // If we subtracted too
            q[j as usize] = q[j as usize] - 1;       // much, add back.
            k = 0;
            for i in 0..n{
                t = un[(i + j)as usize] + vn[i as usize] + k;
                un[(i + j) as usize] = t;
                k = t >> 16;
            }
            un[(j + n)as usize] = un[(j + n)as usize] + k;
        }
    } // End j.
// If the caller wants the remainder, unnormalize
// it and pass it back.
    if r != &[] {
        for i in 0..n {
            r[i as usize] = (un[i as usize] >> s) | (un[i as usize + 1] << 16 - s);
        }
    }
    return 0;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_division_divmnu(){
    let mut a: [u8;50] = [0;50];
    let mut b: [u8;50] = [0;50];
    let mut c: [u8;50] = [0;50];
    let mut d: [u8;50] = [0;50];
    assert_eq!(division_divmnu(a.borrow_mut(),  b.borrow_mut(), c.borrow_mut(), d.borrow_mut(),1,1), 1);
}