#[cfg(target_arch = "x86_64")]
use std::borrow::BorrowMut;
#[cfg(target_arch = "riscv64")]
use core::borrow::BorrowMut;

/// w[0], u[0], and v[0] contain the LEAST significant halfwords.
/// (The words are in little-endian order).
/// This is Knuth's Algorithm M from [Knuth Vol. 2 Third edition (1998)]
/// section 4.3.1, altered for signed numbers.  Picture is:
///                   u[m-1] ... u[1] u[0]
///                 x v[n-1] ... v[1] v[0]
///                   --------------------
///        w[m+n-1] ............ w[1] w[0]
pub fn multiply_mulmns<'a>(w: &'a mut [u8], mut u: &'a mut [u8], mut v: &'a mut [u8], m: i32, n: i32) {
    for i in 0..m {
        w[i as usize] = 0;
    }
    for j in 0..n {
        let mut k = 0;
        for i in 0..m {
            let t = u[i as usize] * v[j as usize] + w[(i + j) as usize] + k;
            w[(i + j) as usize] = t;          // (I.e., t & 0xFFFF).
            k = t.wrapping_shr(16);
        }
        w[(j + m) as usize] = k;
    }

/// Now w[] has the unsigned product.  Correct by
/// subtracting v*2**16m if u < 0, and
/// subtracting u*2**16n if v < 0.
    if u[(m - 1) as usize] < 0 {
        let mut b = 0;                    // Initialize borrow.
        for j in 0..n {
            let t = w[(j + m) as usize] - v[j as usize] - b;
            w[(j + m) as usize] = t;
            b = t.wrapping_shr(31);
        }
    }
    if v[(n - 1) as usize] < 0 {
        let mut b = 0;
        for i in 0..m {
            let t = w[(i + n) as usize] - u[i as usize] - b;
            w[(i + n) as usize] = t;
            b = t.wrapping_shr(31);
        }
    }
}

#[allow(overflowing_literals)]
pub fn multiply_mulinv(d: i32) -> i32 {           // d must be odd.
    let mut x1 = 0xFFFFFFFF;
    let mut v1 = -d;
    let mut x2 = 1;
    let mut v2 = d;
    while v2 > 1 {
        let q = v1 / v2;
        let x3 = x1 - q * x2;
        let v3 = v1 - q * v2;
        x1 = x2;
        v1 = v2;
        x2 = x3;
        v2 = v3;
    }
    return x2;
}

pub fn multiply_mulinv2(d: i32) -> i32 {          // d must be odd.
    let mut xn = d;
    let t = d * xn;
    loop {
        if t == 1 { return xn; }
        xn = xn * (2 - t);
    }
}
/// w[0], u[0], and v[0] contain the LEAST significant halfwords.
/// (The halfwords are in little-endian order).
/// This is Knuth's Algorithm M from [Knuth Vol. 2 Third edition (1998)]
/// section 4.3.1.  Picture is:
///                   u[m-1] ... u[1] u[0]
///                 x v[n-1] ... v[1] v[0]
///                   --------------------
///        w[m+n-1] ............ w[1] w[0]
#[allow(arithmetic_overflow)]
pub fn multiply_mulmnu<'a>(w: &'a mut [u8], mut u: &'a mut [u8], mut v: &'a mut [u8], m: i32, n: i32) {
    for i in 0..m {
        w[i as usize] = 0;
    }

    for j in 0..n {
        let mut k = 0;
        for i in 0..m {
            let t = u[i as usize] * v[j as usize] + w[(i + j) as usize] + k;
            w[(i + j) as usize] = t;          // (I.e., t & 0xFFFF).
            let k = t >> 16;
        }
        w[(j + m) as usize] = k;
    }
}

pub fn multiply_mulqdu1(west: &mut [i32], us: &mut [i32], vmw: &mut [i32]) {
    west[2] = 0;
    west[3] = 0;

    for j in 3..0 {
        let mut k = 0;
        for i in 3..0 {
            let t = us[i] * vmw[j] + west[i + j + 1] + k;
            west[i + j + 1] = t & 0xFFFF;
            k = t >> 16;
        }
        west[j] = k;
    }
    return;
}
/// The program below takes 16 ops, 4 of which are multiplies,
/// which are of the type unsigned 16 x 16 ==> 32.
/// The statement "low = (w1 << 16) + (w0 & 0xFFFF);" placed just before
/// the return statement, computes the low-order part in 3 more ops.
pub fn multiply_mulhu(u: i32, v: i32) -> i32 {
    let u0 = u & 0xFFFF;
    let u1 = u >> 16;
    let v0 = v & 0xFFFF;
    let v1 = v >> 16;
    let w0 = u0 * v0;
    let t = u1 * v0 + (w0 >> 16);
    let mut w1 = t & 0xFFFF;
    let w2 = t >> 16;
    w1 = u0 * v1 + w1;
    return u1 * v1 + w2 + (w1 >> 16);
}
/* The next version does it using only three multiplications.
It is based on:
Let u = a*2**16 + b,
    v = c*2**16 + d.
Then calculate
   p = ac,
   q = bd,
   r = (-a + b)(c - d)
Then uv = p*2**32 + (r + p + q)*2**16 + q.
   There is a difficulty in computing r, because it doesn't
quite fit in a 32-bit word. But because 0 <= a, b, c, d < 2**16,
it is easy to see that
   -2**32 < r < 2**32.
   Thus it can be represented as a 64-bit quantity with the high-order
32 bits being either 0 or all 1's. The low-order 32 bits, rlow, can be
calculated directly from r = (-a + b)*(c - d), using 32-bit
instructions. The high-order 32 bits will be all 1's if the product is
negative, and 0 if it is nonnegative. The product is negative if (-a + b)
and (c - d) have opposite signs. Thus, basically,
   rhigh = ((-a + b) xor (c - d)) >>s 31.
However, if either a = b or c = d, we must ensure that rhigh = 0. It
suffices to test rlow, i.e., follow the above assignment to rhigh with:
   if (rlow == 0) rhigh = 0.
This is because if rlow = 0, it must be the case that either a = b
or c = d, because the product cannot be >= 2**32.
   This leads to the function below.
*/
pub fn multiply_mulhu1(u: i32, v: i32) -> i32 {
    let a = u >> 16;
    let b = u & 0xFFFF;
    let c = v >> 16;
    let d = v & 0xFFFF;

    let mut p = a * c;
    let mut q = b * d;
    let mut rlow = (-a + b) * (c - d);
    let mut rhigh = ((-a + b) ^ (c - d)) >> 31;
    if rlow == 0 {
        rhigh = 0;    // Correction.
    }
    q = q + (q >> 16);   // Overflow cannot occur here.
    rlow = rlow + p;
    if rlow < p {
        rhigh = rhigh + 1;
    }
    rlow = rlow + q;
    if rlow < q {
        rhigh = rhigh + 1;
    }
    return p + (rlow >> 16) + (rhigh << 16);
}
/* Branch-free version: */
pub fn multiply_mulhu2(u: i32, v: i32) -> i32 {
    let a = u >> 16;
    let b = u & 0xFFFF;
    let c = v >> 16;
    let d = v & 0xFFFF;

    let mut p = a * c;
    let mut q = b * d;
    let x = -a + b;
    let y = c - d;
    let mut rlow = x * y;
    let mut rhigh = (x ^ y) & (rlow | -rlow);
    let mut rhigh = rhigh >> 31;

    q = q + (q >> 16);   // Overflow cannot occur here.
    let t = (rlow & 0xFFFF) + (p & 0xFFFF) + (q & 0xFFFF);
    p += (t >> 16) + (rlow >> 16) + (p >> 16) + (q >> 16);
    p += (rhigh << 16);
    return p;
}

pub fn multiply_mulhs(u: i32, v: i32) -> i32 {
    let u0 = u & 0xFFFF;
    let u1 = u >> 16;
    let v0 = v & 0xFFFF;
    let v1 = v >> 16;
    let w0 = u0 * v0;
    let t = u1 * v0 + (w0 >> 16);
    let mut w1 = t & 0xFFFF;
    let mut w2 = t >> 16;
    w1 = u0 * v1 + w1;
    return u1 * v1 + w2 + (w1 >> 16);
}
/// w (two words) gets the product of u and v (one word each).
/// w[0] is the most significant word of the result, w[1] the least.
/// (The words are in big-endian order).
/// It is Knuth's Algorithm M from [Knu2] section 4.3.1.
pub fn multiply_muldwu1(w: &mut [i32], u: i32, v: i32) {
    let u0 = u >> 16;
    let u1 = u & 0xFFFF;
    let v0 = v >> 16;
    let v1 = v & 0xFFFF;

    let mut t = u1 * v1;
    let w3 = t & 0xFFFF;             // (*)
    let mut k = t >> 16;

    t = u0 * v1 + k;
    let w2 = t & 0xFFFF;
    let w1 = t >> 16;

    t = u1 * v0 + w2;
    k = t >> 16;

    w[0] = u0 * v0 + w1 + k;
    w[1] = (t << 16) + w3;       // (*)
    /* w[1] = u*v;                  // Alternative. */

    return;
}
/// w (two words) gets the product of u and v (one word each).
/// w[0] is the most significant word of the result, w[1] the least.
/// (The words are in big-endian order).
/// It is derived from Knuth's Algorithm M from [Knu2] section 4.3.1.
pub fn multiply_muldws1(w: &mut [i32], u:i32, v:i32) {
    let u0 = u >> 16;
    let u1 = u & 0xFFFF;
    let v0 = v >> 16;
    let v1 = v & 0xFFFF;

    let mut t = u1 * v1;
    let mut w3 = t & 0xFFFF;             // (*)
    let mut k = t >> 16;

    t = u0 * v1 + k;
    let w2 = t & 0xFFFF;
    let w1 = t >> 16;

    t = u1 * v0 + w2;
    k = t >> 16;

    w[0] = u0 * v0 + w1 + k;
    w[1] = (t << 16) + w3;       // (*)

    return;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_multiply_muldws(){
    let mut a = [0;50];
    assert_eq!(multiply_muldws1(a.borrow_mut(), 1, 1), ());
}