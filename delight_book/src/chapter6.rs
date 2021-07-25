use chapter2::{basics_nlz, basics_pop};

// Find leftmost 0-byte, simple sequence of tests.
pub fn search_zbytel1(x: i64) -> i64 {
    return if (x >> 24) == 0 { 0 } else if (x & 0x00FF0000) == 0 { 1 } else if (x & 0x0000FF00) == 0 { 2 } else if (x & 0x000000FF) == 0 { 3 } else { 4 };
}

// Find leftmost 0-byte, branch-free code.
pub fn search_zbytel2(x: i64) -> i64 {
// Original byte: 00 80 other
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F;   // 7F 7F 1xxxxxxx
    y = !(y | x | 0x7F7F7F7F);           // 80 00 00000000
    let n = basics_nlz(y as u32) >> 3;             // n = 0 ... 4, 4 if x
    return n as i64;                    // has no 0-byte.
}

// Find leftmost 0-byte, not using nlz.
pub fn search_zbytel3(x: i64) -> i64 {
// Original byte: 00 80 other
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F; // 7F 7F 1xxxxxxx
    y = !(y | x | 0x7F7F7F7F);         // 80 00 00000000
// These steps map:
    return if y == 0 { 4 }             // 00000000 ==> 4,
    else if y > 0x0000FFFF           // 80xxxxxx ==> 0,
    { (y >> 31) ^ 1 }          // 0080xxxx ==> 1,
    else                               // 000080xx ==> 2,
    { (y >> 15) ^ 3 };          // 00000080 ==> 3.
}

pub fn search_zbytel3a(x: u32) -> u32 {
    let table = [4, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];  // Original byte: 00 80 other
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F; // 7F 7F 1xxxxxxx
    y = !(y | x | 0x7F7F7F7F);         // 80 00 00000000
    return table[(y.wrapping_mul(0x00204081).wrapping_shr(28)) as usize];
}

// Find leftmost 0-byte by evaluating a polynomial.
pub fn search_zbytel4(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F;
    y = y | x;           // Leading 1 on nonzero bytes.

    let t1 = y >> 31;               // t1 = a.
    let t2 = (y >> 23) & t1;         // t2 = ab.
    let t3 = (y >> 15) & t2;         // t3 = abc.
    let t4 = (y >> 7) & t3;         // t4 = abcd.
    return t1 + t2 + t3 + t4;
}

// Find leftmost byte having value le 9.
pub fn search_valle9(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x76767676;
    y = y | x;
    y = y | 0x7F7F7F7F;          // Bytes > 9 are 0xFF.
    y = !y;                      // Bytes > 9 are 0x00,
// bytes <= 9 are 0x80.
    let n = basics_nlz(y as u32) >> 3;
    return n as i64;
}

// Find leftmost byte having an upper case letter.
pub fn search_valupcase(x: i64) -> i64 {
    let mut d = ((x as i64 | 0x80808080) - 0x41414141) as i64;
    d = !((x | 0x7F7F7F7F) ^ d);
    let mut y = (d & 0x7F7F7F7F) + 0x66666666;
    y = y | d;
    y = y | 0x7F7F7F7F;    // Bytes not from 41-5A are FF.
    y = !y;                // Bytes not from 41-5A are 00,
// bytes from 41-5A are 80.
    let n = basics_nlz(y as u32) >> 3;
    return n as i64;
}

/* This function finds the length and position of the shortest
contiguous string of 1's in a word. The position is the distance of the
leftmost bit of the string, from the left end of the string, or 32 if x
= 0. If two or more contiguous strings are the same length, this
function finds the leftmost one.
   Example: For x = 0x00FF0FF0 it returns length 8, position 8.
   Executes in 8 + 4n instructions on the basic RISC (w/o andc), plus
the time for the nlz function, for n >= 2, where n is the length of the
shortest contiguous string of 1's in x. */
pub fn  search_fminstr1(mut x: i32, mut apos: & mut i32) -> i32 {
    if x == 0 {*apos = 32; return 0; }
    let b = !(x >> 1) & x;   // 0-1 transitions.
    let mut e = x & !(x << 1);   // 1-0 transitions.
    let mut k = 1;
    loop {
        if b & e != 0 {
            break;
        }
        e = e << 1;
        k += 1;
    }
    *apos = basics_nlz((b & e) as u32) as i32;
    return k;
}

/* This function performs the same functions as fminstr1. It might be
useful is your machine has population count as an instruction. The loop
is executed a number of times equal to the number of contiguous strings
of 1-bits in x.
   Executes in 5 + 11n instructions on the full RISC, where n is the
number of strings of 1's in x, for n >= 1 (that is, for x != 0). This
assumes the if-test goes either way half the time, and that pop and nlz
count as one instruction each.
   If "(k <= kmin)" is changed to "(k < kmin)", it finds the RIGHTmost
shortest contiguous string of 1's, when two or more such strings are the
same length.
   If that expression is changed to "(k >= kmin)" and the initialization
of kmin is changed to "kmin = 0;", it finds the leftmost LONGEST
contiguous string of 1's. And if the comparison is changed to ">" and
the initialization of kmin is changed to "kmin = -1;", it finds the
rightmost longest contiguous string of 1's.
   The code is also easily modified to compute the "bestfit" function. */
pub fn search_fminstr11(mut x: i32, mut apos: & mut i32) -> i32 {
    let mut kmin = 32;
    let mut xmin = 0;
    let mut y0 = basics_pop(x as u32);
    let x0 = x;
    while x != 0 {
        x = ((x & -x) + x) & x;   // Turn off rightmost
        let y = basics_pop(x as u32);               // string.
        let k = y0 - y;               // k = length of string
        if k <= kmin {          // turned off.
            kmin = k;              // Save shortest length
            xmin = x;              // found, and the string.
        }
        y0 = y;
    };
    *apos = basics_nlz((x0 ^ xmin) as u32) as i32;
    return kmin as i32;
}

/* This function finds the length and position of the shortest string of
1's in x that is of length n or more, for n >= 1. If such a string does
not exist, it returns with apos = 32 and the length undefined (actually,
the returned value is n - 1). */
pub fn search_bestfit(mut x: i32, n: i32, apos: & mut i32) -> i32 {
    let mut m = n;
    while m > 1 {
        let s = m >> 1;
        x = x & (x << s);
        m = m - s;
    }
    return search_fminstr1(x, apos) + n - 1;
}

/* This function finds the length and position of the shortest string of
1's in x that is of length n or more, for n >= 0. If such a string does
not exist, it returns with apos = 32 and the length undefined (actually,
the returned value is 32).
   This is a simple modification of fminstr11. */
pub fn search_bestfit1(mut x: i32, n: i32, apos: & mut i32) -> i32 {
    let mut kmin = 32;
    let mut xmin = x;
    let mut y0 = basics_pop(x as u32);
    let mut x0 = x;
    while x != 0 {
        x = ((x | (x - 1)) + 1) & x;      // Turn off
        let y = basics_pop(x as u32);               // rightmost string.
        let k = y0 - y;               // k = length of string
        if k <= kmin && k >= n as u32 {// turned off.
            kmin = k;              // Save shortest length
            xmin = x;              // found, and the string.
        }
        y0 = y;
    };
    *apos = basics_nlz((x0 ^ xmin) as u32) as i32;
    return kmin as i32;
}

// Find first string of 1's of a given length, simple routine.
pub fn search_ffstr11(mut x: i32, n: i32) -> i32 {
    let mut p = 0;               // Initialize position to return.
    while x != 0 {
        let mut k = basics_nlz(x as u32);       // Skip over initial 0's
        x = x << k;       // (if any).
        p = p + k;
        k = basics_nlz(!x as u32);      // Count first/next group of 1's.
        if k >= n as u32 {      // If enough,
            return p as i32;
        }     // return.
        x = x << k;       // Not enough 1's, skip over
        p = p + k;        // them.
    }
    return 32;
}

// Find first string of n 1's, shift-and-and sequence.
pub fn search_ffstr12(mut x: i32, mut n: i32) -> i32 {
    while n > 1 {
        let s = n >> 1;
        let x = x & (x << s);
        n = n - s;
    }
    return basics_nlz(x as u32) as i32;
}

// Find first string of n 1's, shift-and-and sequence unrolled.
pub fn search_ffstr13(mut x: i32, mut n: i32) -> u32 {
    let mut s = n >> 1;
    let mut x = x & (x << s);
    n = n - s;

    s = n >> 1;
    x = x & (x << s);
    n = n - s;

    s = n >> 1;
    x = x & (x << s);
    n = n - s;

    s = n >> 1;
    x = x & (x << s);
    n = n - s;

    s = n >> 1;
    x = x & (x << s);

    return basics_nlz(x as u32);
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_search(){
    assert_eq!(search_bestfit(1,1,&mut 1),1);
    assert_eq!(search_bestfit1(1,1,&mut 1),1);
    assert_eq!(search_ffstr11(1,1),31);
    assert_eq!(search_ffstr12(1,1),31);
    assert_eq!(search_ffstr13(1,1),31);
}