/// Some Elementary functions
pub fn elementary_isqrt1(x: i64) -> i64 {
    if x <= 1
    { return x; }
    let mut s = 1;
    let mut x1 = x - 1;
    if x1 > 65535 {
        s = s + 8;
        x1 = x1 >> 16;
    }
    if x1 > 255 {
        s = s + 4;
        x1 = x1 >> 8;
    }
    if x1 > 15 {
        s = s + 2;
        x1 = x1 >> 4;
    }
    if x1 > 3 { s = s + 1; }

    let mut g0 = 1 << s;                // g0 = 2**s.
    let mut g1 = (g0 + (x >> s)) >> 1;  // g1 = (g0 + x/g0)/2.

    while g1 < g0 {           // Do while approximations
        g0 = g1;                 // strictly decrease.
        g1 = (g0 + (x / g0)) >> 1;
    }
    return g0;
}

pub fn elementary_isqrt2(x: i64) -> i64 {
    let mut s = 0;
    if x <= 4224 {
        if x <= 24 {
            if x <= 3 { return (x + 3) >> 2; } else if x <= 8
            { return 2; } else { return (x >> 4) + 3; }
        } else if x <= 288
        {
            if x <= 80 { s = 3; } else { s = 4; }
        } else if x <= 1088
        { s = 5; } else { s = 6; }
    } else if x <= 1025 * 1025 - 1 {
        if x <= 257 * 257 - 1
        {
            if x <= 129 * 129 - 1
            { s = 7; } else { s = 8; }
        } else if x <= 513 * 513 - 1
        { s = 9; } else { s = 10; }
    } else if x <= 4097 * 4097 - 1 {
        if x <= 2049 * 2049 - 1
        { s = 11; } else { s = 12; }
    } else if x <= 16385 * 16385 - 1
    {
        if x <= 8193 * 8193 - 1
        { s = 13; } else { s = 14; }
    } else if x <= 32769 * 32769 - 1
    { s = 15; } else { s = 16; }
    let mut g0 = 1 << s;                // g0 = 2**s.

// Continue as in the previous program.
    let mut g1 = (g0 + (x >> s)) >> 1;  // g1 = (g0 + x/g0)/2.

    while g1 < g0 {           // Do while approximations
        g0 = g1;                 // strictly decrease.
        g1 = (g0 + (x / g0)) >> 1;
    }
    return g0;
}

pub fn elementary_isqrt3(x: i64) -> i64 {
    let mut a = 1;
    let mut b = (x >> 5) + 8;            // See text.
    if b > 65535
    { b = 65535; }
    while b >= a {
        let mut m = (a + b) >> 1;
        if m * m > x
        { b = m - 1; } else { a = m + 1; }
    }
    return a - 1;
}

// Hardware algorithm [GLS]
pub fn elementary_isqrt4(mut x: i64) -> i64 {
    let mut m = 0x40000000;
    let mut y = 0;
    while m != 0 {              // Do 16 times.
        let b = y | m;
        y = y >> 1;
        if x >= b {
            x = x - b;
            y = y | m;
        }
        m = m >> 2;
    }
    return y;
}

/* A 64-bit version of the above is easy to obtain. Change the
declaration of the function from int to unsigned int, change the
parameter x and the three automatic variables from unsigned to unsigned
long long, and change the first assignment to m to "m =
0x4000000000000000LL;". The while-loop will be iterated 32 times. */
/* Modification of isqrt4 that avoids the branch in the loop. */
pub fn elementary_isqrt5(mut x: i64) -> i64 {
    let mut m = 0x40000000;
    let mut y = 0;
    while m != 0 {              // Do 16 times.
        let b = y | m;
        y = y >> 1;
        let t = (x | !(x - b)) >> 31; // -1 if x >= b, else 0.
        x = x - (b & t);
        y = y | (m & t);
        m = m >> 2;
    }
    return y;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_elementary() {
    assert_eq!(elementary_isqrt1(1), 1);
    assert_eq!(elementary_isqrt2(1), 1);
    assert_eq!(elementary_isqrt3(1), 1);
    assert_eq!(elementary_isqrt4(1), 1);
    assert_eq!(elementary_isqrt5(1), 1);
}


// Execution time is 3 + (11 + mul)11 = 124 + 11*mul (avg) cycles.
// ------------------------------ cut ----------------------------------
pub fn elementary_icbrt1(mut x: i32) -> i32 {
    let mut y = 0;
    for s in (30..0).step_by(3) {
        y = 2 * y;
        let b = (3 * y * (y + 1) + 1) << s;
        if x >= b {
            x = x - b;
            y = y + 1;
        }
    }
    return y;
}
// ---------------------------- end cut --------------------------------

// Strength reduced to avoid a multiplication of variables.
// Execution time is 4 + 13.5*11 = 152 (avg) cycles.

pub fn elementary_icbrt2(mut x: i32) -> i32 {
    let mut y2 = 0;
    let mut y = 0;
    for s in (30..0).step_by(3) {
        y2 = 4 * y2;
        y = 2 * y;
        let b = (3 * (y2 + y) + 1) << s;
        if x >= b {
            x = x - b;
            y2 = y2 + 2 * y + 1;
            y = y + 1;
        }
    }
    return y;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_elementary1() {
    assert_eq!(elementary_icbrt1(1), 0);
    assert_eq!(elementary_icbrt2(1), 0);
}
