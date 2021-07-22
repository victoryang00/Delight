/// Hilbert's Curve
///   `'````````````````::``````````````````:`    `:````````::```````':'```````::````````:`    `:````````::``'!:``':'```!'``::````````:`
///   `'````````````````::'`````````````````:`    `:````````::```````':'```````::````````:`    `:```::```::```::``':'``::'``::```::```:`
///   `'````````````````::'`````````````````:`    `:```!%%%%$$%%%|:``':'``:|%%%$$%%%%!```:`    `::!:;;;!'::;!:;:!;':';!:;:!;::'!;;;:!::`
///   `'```````;::::::::::::::::::::````````:`    `:```;;```::``';:``':'``:;'``::```;;```:`    `!!|!|||%!||||!||||!|!||!|!%|||!||||!%!!`
///   `'``````';:```````':'```````;;````````:`    `:```;;```::``';:``':'``;;'``::```;;```:`    `::;::::%$$$$!'::;;':';;::'!$$&$%:::';::`
///   `'``````';:```````::'```````;;````````:`    `:```;;```::``';:``':'``:;'``::```;;```:`    `:;!;;;''';;''';;!!:;:!!;;:'';;''';;:!;:`
///   `'``````';:```````::'```````;;````````:`    `:```;;```::```;%%%$$%%%%:```::```;;```:`    `::!:;;'``::``';:!;':';!:;''`::``';;:!::`
///   `'``````';:```````::'```````;;````````:`    `:```;;```::```````':'```````::```;;```:`    `:```:::;'::;;'::``':'``::';;::';:::```:`
///   `;'''!!'';;''''''':;:''''''';;'''!!''':`    `;'''!!''';;''''''':;:''''''';;'''!!''':`    `;''';;;!:;;!!:;;'':;:'';;:!!;;:!!;;''';`
///   `'``````';:```````::'```````;;````````:`    `:```;;```::```````':'```````::```;;```:`    `:'|%$%%!':::|%$%||%$%||%$%|:::';%$%%|::`
///   `'``````';:```````::'```````;;````````:`    `:```!%%%%$$%%%|```':'```!%%%%$$%%%|```:`    `!!|!||;;;||;;;|!;;!|!;;!|;;;||;;;||!|!!`
///   `'``````';:```````::'```````;;````````:`    `:```;;```::``::```':'```::``::```;;```:`    `::;:::`::!!::;!!:'':'':!!;::!!::`::';::`
///   `'```````:'```````::'```````':````````:`  -  `:```'''':;;''::```':'```::'';;'''''```:`    `::;::::;'::```::;;':';;::'``::';:::';::`
///   `'````````````````::'`````````````````:`    `:````````::``::```':'```::``::````````:`    `::;::::;'::```::;;'::;;::'``::';;:::;::`
///   `'````````````````::'`````````````````:`    '!;;;!%%%%$$%%%||``':'```!%%%%$$%%%|;;;!`    `:`:;||;:`::;|!|!;'':'';!|!|;::`:;||;:':`
///   `'````````````````::'`````````````````:`    `:````````::``';:``':'```;'``::````````:`    `!!!!||!!!|||%!||!!!|!!!||!%|||!!!||!!!!'
///   `'````````````````::'`````````````````:`    `:````````::``'!:``':'```!'``::````````:`    `:````````::``'!:``':'```!'``::````````:`
/// First three curves in the sequence defining Hilbert’s curve.

/// With regard to my great math teacher, Yifeng Yang, at my high school, he taught me a lot about fractal.

extern crate libc;

use libc::c_char;
use std::borrow::{Borrow, BorrowMut};
use chapter2::basics_isolate_1;

const CTYPE_u8_1: *const c_char = -1_isize as *const c_char;
const CTYPE_u8_2: *const c_char = std::usize::MAX as *const c_char;

#[test]
fn test_ctype() {
    assert_eq!(CTYPE_u8_1, 0xffffffffffffffff as *const i8);
    assert_eq!(CTYPE_u8_2, 0xffffffffffffffff as *const i8);
}

pub fn hilbert_rtls_s_from_xy1(mut x: u32, mut y: u32, n: i32) -> u32 {
    let i = 0;
    let mut xi = 0;
    let mut yi = 0;
    let mut s = 0;

    for i in (n - 1)..0 {
        xi = (x >> i) & 1;          // Get bit i of x.
        yi = (y >> i) & 1;          // Get bit i of y.
        s = 4 * s + 2 * xi + (xi ^ yi);   // Append two bits to s.

        x = x ^ y;                    // These 3 lines swap
        y = y ^ (x & (yi - 1));       // x and y if yi = 0.
        x = x ^ y;
        x = x ^ ((-(xi as i64)) as u32 & (yi - 1));     // Complement x and y if
        y = y ^ ((-(xi as i64)) as u32 & (yi - 1));     // xi = 1 and yi = 0.
    }
    return s;
}

pub fn hilbert_rtls_s_from_xy(x: i32, y: i32, n: i32) -> i32 {
    let mut s = 0;                         // Initialize.
    for i in 0..n {
        let xi = (x >> i) & 1;          // Get bit i of x.
        let yi = (y >> i) & 1;          // Get bit i of y.

        if yi == 0 {
            if xi == 0 {
                s = s ^ ((s & 0x55555555) << 1);
            } else {
                s = s ^ ((!s & 0x55555555) << 1);
            }
        }
// Prepend two bits to s.
        s = (s >> 2) | (xi << 31) | ((xi ^ yi) << 30);
    }
    return s >> (32 - 2 * n);
}

#[test]
fn test_hilbert1() {
    let n = 1;
    let N = 1 << n;                          // N = 2**n.
    println!("    x     y     s, order {} Hilbert curve.\n", n);
    for x in 0..N {
        for y in 0..N {
            println!("{} {} {}\n", x, y, hilbert_rtls_s_from_xy1(x, y, n));
            println!("{} {} {}\n", x, y, hilbert_rtls_s_from_xy(x as i32, y as i32, n));
        }
    }
}

/*
Given the "order" n of a Hilbert curve and a distance s along the curve,
this program computes the corresponding (x, y) coordinates.  The square
that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is to employ the following state transition table:

If the current   And the next two     then       and enter
  state is        bits of s are      output        state
----------------------------------------------------------
     A                  00             00            B
     A                  01             01            A
     A                  10             11            A
     A                  11             10            D
     B                  00             00            A
     B                  01             10            B
     B                  10             11            B
     B                  11             01            C
     C                  00             11            D
     C                  01             10            C
     C                  10             00            C
     C                  11             01            B
     D                  00             11            C
     D                  01             01            D
     D                  10             00            D
     D                  11             10            A

The states correspond to mappings, with state A denoting the map from
binary 00 to 00, 01 to 01, 10 to 11, and 11 to 10, and similarly for
states B, C, and D.
   To use the table, start in state A.  Scan the bits of s in pairs from
left to right.  The first row means that if the current state is A and
the currently scanned bits of s are 00, then output 00 and enter state
B.  Then, advance to the next two bits of s.  The third row means that
if the current state is A and the scanned bits are 10, then output 11
and stay in state A.
   If the outputs are accumulated in left-to-right order, then when the
end of s is reached, the output quantity will contain x in the odd
numbered bit positions, and y in the even numbered bit positions.  For
example, suppose

                              s = 110100.

Then since the process starts in state A and the initial bits scanned
are 11, the process outputs 10 and enters state D (fourth row).  Then,
being in state D and scanning 01, the process outputs 01 and stays in
state D.  Lastly, the process outputs 11 and enters state C, although
the state is now immaterial.
   Thus the output is 100111.  From the odd and even bits respectively,
this gives x = 101 and y = 011.  Thus the (x, y) coordinates for s = 52
(110100) are (5, 3). */
pub fn hilbert_xy_from_s(s: i32, n: i32, xp: &mut i32, yp: &mut i32) {
    let mut state = 0;                            // Initialize.
    let mut x = 0;
    let mut y = 0;

    for i in (2 * n - 2..0).step_by(2) {   // Do n times.
        let row = 4 * state | (s >> i) & 3;      // Row in table.
        x = (x << 1) | (0x936C >> row) & 1;
        y = (y << 1) | (0x39C6 >> row) & 1;
        state = (0x3E6B94C1 >> 2 * row) & 3; // New state.
    }
    *xp = x;                              // Pass back
    *yp = y;                              // results.
}

/*
Given the "order" n of a Hilbert curve and (x, y) coordinates, this
program computes the distance s along the curve to the point (x, y).
The square that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is to employ the following state transition table:

If the current   And the next bits   then append  and enter
  state is        of x and y are        to s        state
----------------------------------------------------------
     A                 (0, 0)            00           B
     A                 (0, 1)            01           A
     A                 (1, 0)            11           D
     A                 (1, 1)            10           A
     B                 (0, 0)            00           A
     B                 (0, 1)            11           C
     B                 (1, 0)            01           B
     B                 (1, 1)            10           B
     C                 (0, 0)            10           D
     C                 (0, 1)            11           B
     C                 (1, 0)            01           C
     C                 (1, 1)            00           D
     D                 (0, 0)            10           C
     D                 (0, 1)            01           D
     D                 (1, 0)            11           A
     D                 (1, 1)            00           C

The states correspond to mappings, with state A denoting the map from
binary 00 to 00, 01 to 01, 10 to 11, and 11 to 10, and similarly for
states B, C, and D.
   To use the table, start in state A.  Scan the bits of s in pairs from
left to right.  The first row means that if the current state is A and
the currently scanned bits of (x, y) are (0, 0), then output 00 and
enter state B. Then, advance to the next bits of (x, y).  The third row
means that if the current state is A and the scanned bits are (1, 0),
then output 11 and stay in state D.
   If the output is accumulated in left-to-right order, then when the
end of x and y is reached, the output quantity will contain the length s
of the curve from its beginning to (x, y).
For example, suppose the order is 3 and

                          (x, y) = (4, 3).

Then since the process starts in state A and the initial bits scanned
are (1, 0), the process outputs 11 and enters state D (third row).  Then,
being in state D and scanning (0, 1), the process outputs 01 and stays in
state D.  Lastly, the process outputs 01 and enters state D, although
the state is now immaterial.
   Thus the output is 110101, i.e., decimal 53. */
fn hilbert_s_from_xy(x: u32, y: u32, n: u32) -> u32 {
    let mut state = 0;                            // Initialize.
    let mut s = 0;
    for i in n - 1..0 {
        let row = 4 * state | 2 * ((x >> i) & 1) | (y >> i) & 1;
        s = (s << 2) | (0x361E9CB4 >> 2 * row) & 3;
        state = (0x8FE65831 >> 2 * row) & 3;
    }
    return s;
}

#[test]
fn test_hilbert2() {
    let n = 1;
    let N = 1 << 2 * n;                          // N = 2**n.
    let mut x = 0;
    let mut y = 0;
    println!("(x, y) coordinates along the Hilbert curve of order {}.\n", n);
    println!("    s     x     y\n");
    for s in 0..N {
        hilbert_xy_from_s(s, n, &mut x, &mut y);
        hilbert_s_from_xy(x as u32, y as u32, n as u32);
        println!("{} {} {}\n", s, x, y);
    }
}

/* Converts the unsigned integer k to binary character form.
Result is in string s of length len. */
pub fn hilbert_binary(mut k: i32, len: usize, mut s: Vec<char>) -> Vec<char> {
    s[len] = '\x00';
    for i in (len - 1)..0 {
        if k & 1 != 0 { s[i] = '1'; } else { s[i] = '0'; }
        k = k >> 1;
    }
    s
}

/* Converts the unsigned integer k to binary character form.
Result is in string s of length len. */
pub fn hilbert_binary1(mut k: i32, len: usize, mut s: Vec<char>) -> Vec<char> {
    s[len] = '\x00';
    for i in (len - 1)..0 {
        if k & 1 != 0 { s[i] = '1'; } else { s[i] = '0'; }
        k = k >> 1;
    }
    s
}

#[test]
fn test_binary() {
    let test: Vec<char> = vec!['1', '1'];
    let result: Vec<char> = vec!['1', '\x00'];
    assert_eq!(hilbert_binary(1, 1, test.clone()), result.clone());
    assert_eq!(hilbert_binary1(1, 1, test.clone()), result.clone());
    // let a=1;
}

pub struct addressOfXY<'a> {
    pub s: i64,
    pub xp: &'a mut i64,
    pub yp: &'a mut i64,
}

pub fn hilbert_glsxy<'a>(mut s: i64, n: i64, xp: &'a mut i64, yp: &'a mut i64) -> addressOfXY<'a> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for i in (0..2 * n).step_by(2) {
        let sa = (s >> (i + 1)) & 1;       // Get bit i+1 of s.
        let sb = (s >> i) & 1;             // Get bit i of s.

        if (sa ^ sb) == 0 {            // If sa,sb = 00 or 11,
            let temp = x;                  // swap x and y,
            x = y ^ (-sa);             // and if sa = 1,
            y = temp ^ (-sa);          // complement them.
        }
        x = (x >> 1) | (sa << 31);     // Prepend sa to x and
        y = (y >> 1) | ((sa ^ sb) << 31); // (sa^sb) to y.
    }
    *xp = x >> (32 - n);           // Right-adjust x and y
    *yp = y >> (32 - n);           // and return them to
    addressOfXY { s, xp, yp }
}                                      // the caller.

/// Parallel prefix xor op to propagate both complement and swap info together from left to right (there is
/// no step "cs ^= cs >> 1", so in effect it computes two independent parallel prefix operations on two
/// interleaved sets of sixteen bits).
pub fn hilbert_glsxy3<'a>(mut s: i64, n: i64, xp: &'a mut i64, yp: &'a mut i64) -> addressOfXY<'a> {
    s = s | (0x55555555 << 2 * n); // Pad s on left with 01
    let sr = (s >> 1) & 0x55555555;    // (no change) groups.
    let mut cs = ((s & 0x55555555) + sr)   // Compute complement &
        ^ 0x55555555;              // swap info in two-bit groups.

    cs = cs ^ (cs >> 2);
    cs = cs ^ (cs >> 4);
    cs = cs ^ (cs >> 8);
    cs = cs ^ (cs >> 16);
    let swap = cs & 0x55555555;         // Separate the swap and
    let comp = (cs >> 1) & 0x55555555;  // complement bits.

    let mut t = (s & swap) ^ comp;          // Calculate x and y in
    s = s ^ sr ^ t ^ (t << 1);      // the odd & even bit positions, resp.
    s = s & ((1 << 2 * n) - 1);    // Clear out any junk on the left (unpad).

// Now "unshuffle" to separate the x and y bits.

    t = (s ^ (s >> 1)) & 0x22222222;
    s = s ^ t ^ (t << 1);
    t = (s ^ (s >> 2)) & 0x0C0C0C0C;
    s = s ^ t ^ (t << 2);
    t = (s ^ (s >> 4)) & 0x00F000F0;
    s = s ^ t ^ (t << 4);
    t = (s ^ (s >> 8)) & 0x0000FF00;
    s = s ^ t ^ (t << 8);

    *xp = s >> 16;               // Assign the two halves
    *yp = s & 0xFFFF;            // of t to x and y.

    addressOfXY { s, xp, yp }
}

#[test]
fn test_glsxy() {
    let n = 10;
    let N = 1 << (2 * n);
    let mut xy1 = addressOfXY { s: 0, xp: &mut 0, yp: &mut 0 };
    let mut xy3 = addressOfXY { s: 0, xp: &mut 0, yp: &mut 0 };
    assert_eq!(xy1.xp, xy3.xp);
    assert_eq!(xy1.yp, xy3.yp);
    for s in 0..N {
        xy1 = hilbert_glsxy(s, n, xy1.xp.borrow_mut(), xy1.yp.borrow_mut());
        xy3 = hilbert_glsxy3(s, n, xy3.xp.borrow_mut(), xy3.yp.borrow_mut());
    }
}

/*
Given the "order" n of a Hilbert curve and a distance s along the curve,
this program computes the corresponding (x, y) coordinates.  The square
that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is that given in [Lam&Shap], described by the following
table.  Here i = n-1 for the most significant bit of x and y, and i = 0
for the least significant bits.

              s[2i+1:2i]   x[i]  y[i]  x[i-1:0]  y[i-1:0]
              -----------|-------------------------------
                   00    |  0     0    y[i-1:0]  x[i-1:0]
                   01    |  0     1    x[i-1:0]  y[i-1:0]
                   10    |  1     1    x[i-1:0]  y[i-1:0]
                   11    |  1     0   ~y[i-1:0] ~x[i-1:0]

To use this table, start at the least significant two bits of s (i = 0).
If they are both 0 (first row), set the least significant bits of x and
y to 0 and 0 respectively, and interchange x and y.  The last two
columns designate an interchange of the bits of x and y to the right of
bit i, but on this first iteration they are null, so there is no
interchange to do.  If the least significant two bits of s are 10 (third
row), set the least significant bits of x and y to 1, and similarly for
the other rows.
   Then, consider the next least significant two bits of s, and select
the appropriate row of the table to determine the next bits of x and y,
and how to change the bits of x and y to the right of i.  Continue until
the most significant bits of x and y have been processed. */
pub fn hilbert_lamxy(s: u32, n: u32, xp: &mut u32, yp: &mut u32) {
    let mut x = 0;
    let mut y = 0;
    for i in (0..2 * n).step_by(2) {
        let sa = (s >> (i + 1)) & 1;      // Get bit i+1 of s.
        let sb = (s >> i) & 1;          // Get bit i of s.

        if (sa ^ sb) == 0 {       // If sa,sb = 00 or 11,
            let temp = x;                // swap x and y,
            x = y ^ (0 - sa);             // and if sa = 1,
            y = temp ^ (0 - sa);          // complement them.
        }
        x = (x >> 1) | (sa << 31);  // Prepend sa to x and
        y = (y >> 1) | ((sa ^ sb) << 31); // (sa^sb) to y.
    }
    *xp = x >> (32 - n);           // Right-adjust x and y
    *yp = y >> (32 - n);           // and return them to
}                                 // the caller.

/* Variation of lamxy that eliminates the branch in the loop. */
/*
Given the "order" n of a Hilbert curve and a distance s along the curve,
this program computes the corresponding (x, y) coordinates.  The square
that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is that given in [Lam&Shap], described by the following
table.  Here i = n-1 for the most significant bit of x and y, and i = 0
for the least significant bits.

              s[2i+1:2i]   x[i]  y[i]  x[i-1:0]  y[i-1:0]
              -----------|-------------------------------
                   00    |  0     0    y[i-1:0]  x[i-1:0]
                   01    |  0     1    x[i-1:0]  y[i-1:0]
                   10    |  1     1    x[i-1:0]  y[i-1:0]
                   11    |  1     0   ~y[i-1:0] ~x[i-1:0]

To use this table, start at the least significant two bits of s (i = 0).
If they are both 0 (first row), set the least significant bits of x and
y to 0 and 0 respectively, and interchange x and y.  The last two
columns designate an interchange of the bits of x and y to the right of
bit i, but on this first iteration they are null, so there is no
interchange to do.  If the least significant two bits of s are 10 (third
row), set the least significant bits of x and y to 1, and similarly for
the other rows.
   Then, consider the next least significant two bits of s, and select
the appropriate row of the table to determine the next bits of x and y,
and how to change the bits of x and y to the right of i.  Continue until
the most significant bits of x and y have been processed. */
pub fn hilbert_lamxy1(s: u32, n: u32, xp: &mut u32, yp: &mut u32) {
    let mut x = 0;
    let mut y = 0;
    for i in (0..2 * n).step_by(2) {
        let sa = (s >> (i + 1)) & 1;      // Get bit i+1 of s.
        let sb = (s >> i) & 1;          // Get bit i of s.

        let swap = (sa ^ sb) - 1;  // -1 if should swap, else 0.
        let cmpl = 0 - (sa & sb);     // -1 if should compl't, else 0.
        x = x ^ y;
        y = y ^ (x & swap) ^ cmpl;
        x = x ^ y;

        x = (x >> 1) | (sa << 31);  // Prepend sa to x and
        y = (y >> 1) | ((sa ^ sb) << 31); // (sa^sb) to y.
    }
    *xp = x >> (32 - n);           // Right-adjust x and y
    *yp = y >> (32 - n);           // and return them to
}                                 // the caller.

/*
Given the "order" n of a Hilbert curve and coordinates x and y, this
program computes the length s of the curve from the origin to (x, y).
The square that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is that given in [Lam&Shap], described by the following
table.  Here i = n-1 for the most significant bit of x and y, and i = 0
for the least significant bits.

                    x[i]  y[i] | s[2i+1:2i]   x   y
                    -----------|-------------------
                     0     0   |     00       y   x
                     0     1   |     01       x   y
                     1     0   |     11      ~y  ~x
                     1     1   |     10       x   y

To use this table, start at the most significant bits of x and y
(i = n - 1).  If they are both 0 (first row), set the most significant
two bits of s to 00 and interchange x and y.  (Actually, it is only
necessary to interchange the remaining bits of x and y.)  If the most
significant bits of x and y are 10 (third row), output 11, interchange x
and y, and complement x and y.
   Then, consider the next most significant bits of x and y (which may
have been changed by this process), and select the appropriate row of
the table to determine the next two bits of s, and how to change x and
y.  Continue until the least significant bits of x and y have been
processed. */
pub fn hilbert_lams1(mut x: u64, mut y: u64, n: u32) -> u64 {
    let mut s = 0;                         // Initialize.
    for i in n - 1..0 {
        let xi = (x >> i) & 1;          // Get bit i of x.
        let yi = (y >> i) & 1;          // Get bit i of y.
        s = 4 * s + 2 * xi + (xi ^ yi);   // Append two bits to s.

        x = x ^ y;                    // These 3 lines swap
        y = y ^ (x & (yi - 1));       // x and y if yi = 0.
        x = x ^ y;
        x = x ^ (0 - xi & (yi - 1));     // Complement x and y if
        y = y ^ (0 - xi & (yi - 1));     // xi = 1 and yi = 0.
    }
    return s;
}

/*
Given the "order" n of a Hilbert curve and coordinates x and y, this
program computes the length s of the curve from the origin to (x, y).
The square that the Hilbert curve traverses is of size 2**n by 2**n.
   The method is that given in [Lam&Shap], described by the following
table.  Here i = n-1 for the most significant bit of x and y, and i = 0
for the least significant bits.

                    x[i]  y[i] | s[2i+1:2i]   x   y
                    -----------|-------------------
                     0     0   |     00       y   x
                     0     1   |     01       x   y
                     1     0   |     11      ~y  ~x
                     1     1   |     10       x   y

To use this table, start at the most significant bits of x and y
(i = n - 1).  If they are both 0 (first row), set the most significant
two bits of s to 00 and interchange x and y.  (Actually, it is only
necessary to interchange the remaining bits of x and y.)  If the most
significant bits of x and y are 10 (third row), output 11, interchange x
and y, and complement x and y.
   Then, consider the next most significant bits of x and y (which may
have been changed by this process), and select the appropriate row of
the table to determine the next two bits of s, and how to change x and
y.  Continue until the least significant bits of x and y have been
processed. */
pub fn hilbert_lams(mut x: u64, mut y: u64, n: u32) -> u64 {
    let mut s = 0;                         // Initialize.
    for i in n - 1..0 {
        let xi = (x >> i) & 1;          // Get bit i of x.
        let yi = (y >> i) & 1;          // Get bit i of y.

        if yi == 0 {
            let temp = x;                // Swap x and y and,
            x = y ^ (0 - xi);             // if xi = 1,
            y = temp ^ (0 - xi);          // complement them.
        }
        s = 4 * s + 2 * xi + (xi ^ yi);   // Append two bits to s.
    }
    return s;
}

#[test]
fn test_lams() {
    let n = 10;
    let N = 1 << n;
    for x in 0..N {
        for y in 0..N {
            assert_eq!(hilbert_lams1(x, y, n), hilbert_lams(x, y, n));
        }
    }
}

/*
Simulates the left-to-right logic circuit for determining whether to
increment or decrement x or y.

This code is not in the book.  It is not written for efficiency, just to
ensure that the logic works.


(X, Y) = (xi, yi) swapped if S = 1 and complemented if C = 1.
I      = 1/0: increment/decrement.
W      = 1/0: change x/y.
S      = 1: Swap x and y.
C      = 1: Complement x and y.

Initial conditions: I(n) = W(n) = S(n) = C(n) = 0.
*/
pub fn hilbert_logic_inc_from_xy(x: i32, y: i32, n: i32, Ip: &mut i32, Wp: &mut i32) {
    let S = 0;
    let C = 0;
    let I = 0;
    let W = 0;
    for i in n - 1..0 {
        let xi = (x >> i) & 1;        // Get bit i of x.
        let yi = (y >> i) & 1;        // Get bit i of y.

        let X = (S & yi | !S & xi) ^ C;
        let Y = (S & xi | !S & yi) ^ C;
        let I = !C & !X | C & X & Y | I & X & !Y;
        let W = !S & !X & Y | S & !(X ^ Y) | W & X & !Y;
        let S = !(S ^ Y);
        let C = C ^ (X & !Y);
        println!("i = {}, xi = {}, yi = {}, I = {}, W = {}, S = {}, C = {}\n", i, xi, yi, I, W, S, C);
    }
    *Ip = I;                     // Return I and W
    *Wp = W;                     // to caller.
    return;
}

/*
Simulates the left-to-right logic circuit for converting path length s
to (x, y) coordinates in the order n Hilbert curve.

                  If       Then set     And set swap and
              s[2i+1:2i]   x[i]  y[i]   complement controls
              -----------|------------|-------------------
                   00    |  0     0   | swap = ~swap
                   01    |  0     1   | No change
                   10    |  1     1   | No change
                   11    |  1     0   | swap = ~swap, cmpl = ~cmpl

To use this table, initially have swap = cmpl = 0, which means do not
swap and do not complement.  Start with the most significant two bits of
s (i = n).  If they are both 0 (first row), set the most significant
bits of x and y to 0, and invert the "swap" control.  Next, test the
next two bits of s. Suppose they are 01.  Then, since swap = 1, set
x[n-1] = 1 and x[y-1] = 0 and do not change the swap and complement
controls.
   Continue until the least significant bits of s have been processed. */
pub fn hilbert_logic_xy_from_s(s: i32, n: i32, xp: &mut i32, yp: &mut i32) {
    let mut x = 0;
    let mut y = 0;                     // Initialize result.
    let mut swap = false;
    let mut cmpl = false;               // Initialize controls.
    for i in n - 1..0 {
        let sa = (s >> (2 * i + 1)) & 1;    // Get bit 2i+1 of s.
        let sb = (s >> 2 * i) & 1;        // Get bit 2i of s.

        let mut xi = sa;                    // Set basic (xi, yi).
        let mut yi = sa ^ sb;
        if swap {                 // Swap and/or
            let temp = xi;               // complement
            xi = yi;                 // xi and yi.
            yi = temp;
        }
        if cmpl {
            xi = 1 - xi;
            yi = 1 - yi;
        }

        x = (x << 1) | xi;          // Append xi and yi
        y = (y << 1) | yi;          // to x and y.

        if (sa ^ sb) == 0 {       // Update controls.
            swap = (1 - (swap as i32)) != 0;
            if sa != 0 {
                cmpl = (1 - (cmpl as i32)) != 0;
            }
        }
    }
    *xp = x;                       // Return (x, y) to
    *yp = y;                       // caller.
    return;
}

#[test]
fn test_logic() {
    let n = 10;
    let mut I = 0;
    let mut W = 0;
    let N: i64 = 1 << 2 * n;
    let mut x = 0;
    let mut y = 0;
    for s in 0..N {
        assert_eq!(hilbert_logic_inc_from_xy(x, y, n, &mut I, &mut W), hilbert_logic_xy_from_s(s as i32, n, &mut x, &mut y));
        if W == 0 {
            x = x + I;
        } else {
            y = y + I;
        }
    }
}


static mut XX: i32 = -1;
static mut YY: i32 = 0;              // Global variables.
static mut SS: i32 = 0;                      // Dist. along curve.
static mut blen: i32 = 0;                      // Dist. along curve.

pub unsafe fn hilbert(mut dir: i32, rot: i32, order: i32) {
    if order == 0 {
        return;
    }
    dir = dir + rot;
    hilbert(dir, -rot, order - 1);
    step(dir);
    dir = dir - rot;
    hilbert(dir, rot, order - 1);
    step(dir);
    hilbert(dir, rot, order - 1);
    dir = dir - rot;
    step(dir);
    hilbert(dir, -rot, order - 1);
}

pub unsafe fn binary(mut k: i32, len: usize, mut s: Vec<char>) {
    /* Converts the unsigned integer k to binary character
    form.  Result is string s of length len. */
    s[len] = '\x00';
    for i in len - 1..0 {
        if k & 1!=0 { s[i] = "1".parse().unwrap(); } else { s[i] = "0".parse().unwrap(); }
        k = k >> 1;
    }
}

pub unsafe fn step(dir: i32) {
    let ii: Vec<char> = Vec::with_capacity(17);
    let xx: Vec<char> = Vec::with_capacity(17);
    let yy: Vec<char> = Vec::with_capacity(17);

    match dir & 3 {
        0 => XX = XX + 1,
        1 => YY = YY + 1,
        2 => XX = XX - 1,
        3 => YY = YY - 1,
        _ => {}
    }
    binary(SS, (2 * blen) as usize, ii.clone());
    binary(XX, blen as usize, xx.clone());
    binary(YY, blen as usize, yy.clone());
    println!("{}   {}   {} {}\n", dir, ii.into_iter().collect::<String>() , xx.into_iter().collect::<String>(), yy.into_iter().collect::<String>());
    SS = SS + 1;                   // Increment distance.
}
