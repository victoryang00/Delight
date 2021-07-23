/* A the functions herein are for transposing an 8x8 bit matrix, that
is presumed to be a block of a larger matrix of size m x n bytes. Brief
description of the functions:

transpose8vn:    Very naive method, directly places one bit at a time.
transpose8b64:   Basic shifting method that directly places a few bits at a time.
transpose8b64c:  Compact version of 8b64; uses a for-loop.
transpose8bS64:  Like 8b64, but uses GLS's bit swapping device.
transpose8r64:   Basic recursive method (the main point of this comparison study).
transpose8rS64:  Like 8r64, but uses GLS's bit swapping device.
transpose8b32, ..., 8rS32: Above four coded for a 32-bit machine.
transpose8rSr32: Like 8rS32 but done in reverse (coarse to fine granularity). */

/* This is the very naive method, that directly places one bit at a
time. This may be too naive to include in the book (i.e., maybe we
should take this out if there should be another edition).
   This is equay suitable (or unsuitable, if you wish) for a 32- or a
64-bit machine.
   Instruction counts for the calculation part:
   62 ANDs
   56 shifts
   56 ORs (or ADDs or XORs)
   --
  174 total (very naive method, placing one bit at a time) */
pub fn transpose8vn(A: Vec<char>, m: usize, n: usize, mut B: Vec<char>) {
// Load the array into eight one-byte variables.
    let a0 = A[0] as u8;
    let a1 = A[m] as u8;
    let a2 = A[2 * m] as u8;
    let a3 = A[3 * m] as u8;
    let a4 = A[4 * m] as u8;
    let a5 = A[5 * m] as u8;
    let a6 = A[6 * m] as u8;
    let a7 = A[7 * m] as u8;

    let b0 = (a0 & 128) | (a1 & 128) / 2 | (a2 & 128) / 4 | (a3 & 128) / 8 | (a4 & 128) / 16 | (a5 & 128) / 32 | (a6 & 128) / 64 | (a7) / 128;
    let b1 = (a0 & 64) * 2 | (a1 & 64) | (a2 & 64) / 2 | (a3 & 64) / 4 | (a4 & 64) / 8 | (a5 & 64) / 16 | (a6 & 64) / 32 | (a7 & 64) / 64;
    let b2 = (a0 & 32) * 4 | (a1 & 32) * 2 | (a2 & 32) | (a3 & 32) / 2 | (a4 & 32) / 4 | (a5 & 32) / 8 | (a6 & 32) / 16 | (a7 & 32) / 32;
    let b3 = (a0 & 16) * 8 | (a1 & 16) * 4 | (a2 & 16) * 2 | (a3 & 16) | (a4 & 16) / 2 | (a5 & 16) / 4 | (a6 & 16) / 8 | (a7 & 16) / 16;
    let b4 = (a0 & 8) * 16 | (a1 & 8) * 8 | (a2 & 8) * 4 | (a3 & 8) * 2 | (a4 & 8) | (a5 & 8) / 2 | (a6 & 8) / 4 | (a7 & 8) / 8;
    let b5 = (a0 & 4) * 32 | (a1 & 4) * 16 | (a2 & 4) * 8 | (a3 & 4) * 4 | (a4 & 4) * 2 | (a5 & 4) | (a6 & 4) / 2 | (a7 & 4) / 4;
    let b6 = (a0 & 2) * 64 | (a1 & 2) * 32 | (a2 & 2) * 16 | (a3 & 2) * 8 | (a4 & 2) * 4 | (a5 & 2) * 2 | (a6 & 2) | (a7 & 2) / 2;
    let b7 = (a0) * 128 | (a1 & 1) * 64 | (a2 & 1) * 32 | (a3 & 1) * 16 | (a4 & 1) * 8 | (a5 & 1) * 4 | (a6 & 1) * 2 | (a7 & 1);

    B[0] = b0 as char;
    B[n] = b1 as char;
    B[2 * n] = b2 as char;
    B[3 * n] = b3 as char;
    B[4 * n] = b4 as char;
    B[5 * n] = b5 as char;
    B[6 * n] = b6 as char;
    B[7 * n] = b7 as char;
}

/* The above executes in 174 instructions (just the calculation part).
62 ANDs, 56 shifts, and 56 ORs. */

/* transpose8b64 directly places the bits in the target array. It uses
64-bit quantities, which makes it easy to understand. It can be easily
translated for execution on a 32-bit machine, either by hand or by
letting the compiler do it, if your compiler supports 64-bit integers on
a 32-bit machine.
   It is based on the observation that the bits in the 64-bit doubleword
a move either 0, 7, 14, 21, 28, 35, 42, or 49 positions to the left or
right. This is iustrated by the diagram below. Each digit, letter, $,
or period represents a single bit in a 64-bit word. A dash represents a
0, resulting from the shift instructions.
   Looking at the Input and Output lines, the bit denoted by the
character '0' does not move, '1' moves 7 positions to the right, '2'
moves 14 positions to the right, etc.
   Note: Rotate shifts do not help. That is, they do not move any
additional bits to their output positions.

Input x: 01234567 89abcdef ghijklmn opqrstuv wxyzABCD EFGHIJKL MNOPQRST UVWXYZ$.
Output:  08gowEMU 19hpxFNV 2aiqyGOW 3bjrzHPX 4cksAIQY 5dltBJRZ 6emuCKS$ 7fnvDLT.

x:       01234567 89abcdef ghijklmn opqrstuv wxyzABCD EFGHIJKL MNOPQRST UVWXYZ$.
x <<  7: 789abcde fghijklm nopqrstu vwxyzABC DEFGHIJK LMNOPQRS TUVWXYZ$ .-------
x << 14: efghijkl mnopqrst uvwxyzAB CDEFGHIJ KLMNOPQR STUVWXYZ $.------ --------
x << 21: lmnopqrs tuvwxyzA BCDEFGHI JKLMNOPQ RSTUVWXY Z$.----- -------- --------
x << 28: stuvwxyz ABCDEFGH IJKLMNOP QRSTUVWX YZ$.---- -------- -------- --------
x << 35: zABCDEFG HIJKLMNO PQRSTUVW XYZ$.--- -------- -------- -------- --------
x << 42: GHIJKLMN OPQRSTUV WXYZ$.-- -------- -------- -------- -------- --------
x << 49: NOPQRSTU VWXYZ$.- --------- ------- -------- -------- -------- --------
x >>  7: -------0 12345678 9abcdefg hijklmno pqrstuvw xyzABCDE FGHIJKLM NOPQRSTU
x >> 14: -------- ------01 23456789 abcdefgh ijklmnop qrstuvwx yzABCDEF GHIJKLMN
x >> 21: -------- -------- -----012 3456789a bcdefghi jklmnopq rstuvwxy zABCDEFG
x >> 28: -------- -------- -------- ----0123 456789ab cdefghij klmnopqr stuvwxyz
x >> 35: -------- -------- -------- -------- ---01234 56789abc defghijk lmnopqrs
x >> 42: -------- -------- -------- ----------------- -0123456 789abcde fghijklm
x >> 49: -------- -------- -------- ----------------- -------- 01234567 89abcdef

The function below positions some of the bits with an expression of the
form (x & mask) << s, and some with an expression of the form (x >> s) &
mask. This is reduces the number of distinct masks that are required.
   Instruction counts for the calculation part, for a 64-bit machine:
   14 shifts
   15 ANDs
   14 ORs (or ADDs or XORs)
    9 Mask generation (4 for the first and 1 for each subsequent
      one, except the two smaest masks can be immediate fields).
   --
   52 total (64-bit machine, direct placement) */
pub fn transpose8b64(A: Vec<char>, m: usize, n: usize, mut B: Vec<i32>) {
    let mut x = 0;
    for i in 0..7 {      // Load 8 bytes from the
        x = x << 8 | A[m * i] as u64;// input array and pack
    }
// them into x.
    let mut y = x & 0x8040201008040201 |
        (x & 0x0080402010080402) << 7 |
        (x & 0x0000804020100804) << 14 |
        (x & 0x0000008040201008) << 21 |
        (x & 0x0000000080402010) << 28 |
        (x & 0x0000000000804020) << 35 |
        (x & 0x0000000000008040) << 42 |
        (x & 0x0000000000000080) << 49 |
        (x >> 7) & 0x0080402010080402 |
        (x >> 14) & 0x0000804020100804 |
        (x >> 21) & 0x0000008040201008 |
        (x >> 28) & 0x0000000080402010 |
        (x >> 35) & 0x0000000000804020 |
        (x >> 42) & 0x0000000000008040 |
        (x >> 49) & 0x0000000000000080;

    for i in 7..0 {   // Store result into
        B[n * i] = y as i32;
        y = y >> 8;
    }  // output array B.
}

/* This is a compact version of transpose8b64, ca. 75 instructions for
the calculation part. */
pub fn transpose8b64c(A: Vec<char>, m: usize, n: usize, mut B: Vec<char>) {
    let mut x: u64 = 0;
    for i in 0..7 {    // Load 8 bytes from the
        x = x << 8 | A[m * i] as u64;
    }    // input array and pack
    let mut mask: u64 = 0x8040201008040201;
    let mut y: u64 = x & mask;

    for s in (7..49).step_by(7) {
        mask = mask >> 8;
        y = y | ((x & mask) << s) | ((x >> s) & mask);
    }

    for i in 7..0 {   // Store result into
        B[n * i] = (y as u8) as char;
        y = y >> 8;
    }  // output array B.
}

/* This is transpose8b64 but using the GLS method of bit field swapping.
   Instruction counts for the calculation part:
    7 ANDs
   21 XORs
   14 shifts
    8 Mask generation (many can be generated from earlier masks)
   --
   50 total (direct placement method for a 64-bit machine, using GLS's bit swapping) */
pub fn transpose8bS64(A: Vec<char>, m: usize, n: usize, mut B: Vec<char>) {
    let mut x: u64 = 0;
    for i in 7..0 {    // Load 8 bytes from the
        x = x << 8 | A[m * i] as u64;
    }      // input array and pack
// them into x.

    let mut t = (x ^ (x >> 7)) & 0x0080402010080402;
    x = x ^ t ^ (t << 7);
    t = (x ^ (x >> 14)) & 0x0000804020100804;
    x = x ^ t ^ (t << 14);
    t = (x ^ (x >> 21)) & 0x0000008040201008;
    x = x ^ t ^ (t << 21);
    t = (x ^ (x >> 28)) & 0x0000000080402010;
    x = x ^ t ^ (t << 28);
    t = (x ^ (x >> 35)) & 0x0000000000804020;
    x = x ^ t ^ (t << 35);
    t = (x ^ (x >> 42)) & 0x0000000000008040;
    x = x ^ t ^ (t << 42);
    t = (x ^ (x >> 49)) & 0x0000000000000080;
    x = x ^ t ^ (t << 49);

    for i in 7..0 {   // Store result into
        B[n * i] = (x as u8) as char;
        x = x >> 8;
    }  // output array B.
}

/* transpose8r64 is the basic recursive method for a 64-bit machine.
This function positions some of the bits with an expression of the
form (x & mask) << s, and some with an expression of the form (x >> s) &
mask. This is reduces the number of distinct masks that are required.
   Instruction counts for the calculation part, for a 64-bit machine:
    6 shifts
    9 ANDs
    6 ORs (or ADDs or XORs)
   17 Mask generation
   --
   38 total (64-bit machine, basic recursive method) */
pub fn transpose8r64(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
    let mut x: u64 = 0;
    for i in 0..7 { // Load 8 bytes from the
        x = x << 8 | A[m * i] as u64;
    }     // input array and pack
// them into x.

    x = x & 0xAA55AA55AA55AA55 | (x & 0x00AA00AA00AA00AA) << 7 | (x >> 7) & 0x00AA00AA00AA00AA;
    x = x & 0xCCCC3333CCCC3333 | (x & 0x0000CCCC0000CCCC) << 14 | (x >> 14) & 0x0000CCCC0000CCCC;
    x = x & 0xF0F0F0F00F0F0F0F | (x & 0x00000000F0F0F0F0) << 28 | (x >> 28) & 0x00000000F0F0F0F0;

    for i in 7..0 {   // Store result into
        B[n * i] = x as i32;
        x = x >> 8;
    }  // output array B.
}

/* This is transpose8r64 but using the GLS method of bit field swapping.
   Instruction counts for the calculation part, for a 64-bit machine:
    6 shifts
    3 ANDs
    9 XORs
    8 Mask generation
   --
   26 total (64-bit machine, recursive method with GLS bit swapping) */
pub fn transpose8rS64(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
    let mut x: u64 = 0;
    for i in 0..7 {     // Load 8 bytes from the
        x = x << 8 | A[m * i] as u64;
    }      // input array and pack
// them into x.

    let mut t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AA;
    x = x ^ t ^ (t << 7);
    t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCC;
    x = x ^ t ^ (t << 14);
    t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0;
    x = x ^ t ^ (t << 28);

    for i in 7..0 {   // Store result into
        B[n * i] = x as i32;
        x = x >> 8;
    }  // output array B.
}

/* This is transpose8b64 adapted to a 32-bit machine more-or-less
mechanicay. Because of the double-length shifts, some of the terms of
the from (x & mask) << n were changed to (x << n) & mask'. Then, for
consistency, a were changed to that form.
   Instruction counts for the calculation part, for a 32-bit machine:
   26 shifts
   22 ANDs
   26 ORs (or ADDs or XORs)
   10 Mask generation (many can be generated from earlier masks)
   --
   84 total (32-bit machine, direct placement) */
pub fn transpose8b32(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
// Load the array and pack it into xh and xl.

    let xh: u64 = ((A[0] << 24) | (A[m] << 16) | (A[2 * m] << 8) | A[3 * m]) as u64;
    let xl: u64 = ((A[4 * m] << 24) | (A[5 * m] << 16) | (A[6 * m] << 8) | A[7 * m]) as u64;

    let yh = xh & 0x80402010 |
        (xh << 7 | xl >> 25) & 0x40201008 |
        (xh << 14 | xl >> 18) & 0x20100804 |
        (xh << 21 | xl >> 11) & 0x10080402 |
        (xh << 28 | xl >> 4) & 0x08040201 |
        (xl << 3) & 0x04020100 |
        (xl << 10) & 0x02010000 |
        (xl << 17) & 0x01000000 |
        (xh >> 7) & 0x00804020 |
        (xh >> 14) & 0x00008040 |
        (xh >> 21) & 0x00000080;

    let yl = xl & 0x08040201 |
        (xl << 7) & 0x04020100 |
        (xl << 14) & 0x02010000 |
        (xl << 21) & 0x01000000 |
        (xh << 25 | xl >> 7) & 0x10080402 |
        (xh << 18 | xl >> 14) & 0x20100804 |
        (xh << 11 | xl >> 21) & 0x40201008 |
        (xh << 4) & 0x80402010 |
        (xh >> 3) & 0x00804020 |
        (xh >> 10) & 0x00008040 |
        (xh >> 17) & 0x00000080;

    B[0] = (yh >> 24) as i32;
    B[n] = (yh >> 16) as i32;
    B[2 * n] = (yh >> 8) as i32;
    B[3 * n] = yh as i32;
    B[4 * n] = (yl >> 24) as i32;
    B[5 * n] = (yl >> 16) as i32;
    B[6 * n] = (yl >> 8) as i32;
    B[7 * n] = yl as i32;
}

/* This is transpose8b32 but using the GLS method of bit field swapping.
   Instruction counts for the calculation part, for a 32-bit machine:
   27 shifts
   10 ANDs
    6 ORs (or ADDs or XORs)
   31 XORs
    7 Mask generation (many can be generated from earlier masks)
   --
   81 total (32-bit machine, direct placement with GLS bit swapping) */
pub fn transpose8bS32(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
// Load the array and pack it into xh and xl.

    let mut xh: u64 = ((A[0] << 24) | (A[m] << 16) | (A[2 * m] << 8) | A[3 * m]) as u64;
    let mut xl: u64 = ((A[4 * m] << 24) | (A[5 * m] << 16) | (A[6 * m] << 8) | A[7 * m]) as u64;

    let mut th = (xh ^ (xh >> 7)) & 0x00804020;
    let mut tl = (xl ^ (xl >> 7 | xh << 25)) & 0x10080402;
    xh = xh ^ th ^ (th << 7 | tl >> 25);
    xl = xl ^ tl ^ (tl << 7);

    th = (xh ^ (xh >> 14)) & 0x00008040;
    tl = (xl ^ (xl >> 14 | xh << 18)) & 0x20100804;
    xh = xh ^ th ^ (th << 14 | tl >> 18);
    xl = xl ^ tl ^ (tl << 14);

    th = (xh ^ (xh >> 21)) & 0x00000080;
    tl = (xl ^ (xl >> 21 | xh << 11)) & 0x40201008;
    xh = xh ^ th ^ (th << 21 | tl >> 11);
    xl = xl ^ tl ^ (tl << 21);

    tl = (xl ^ (xh << 4)) & 0x80402010;
    xh = xh ^ (tl >> 4);
    xl = xl ^ tl ^ (tl << 28);

    tl = (xl ^ (xh >> 3)) & 0x00804020;
    xh = xh ^ (tl << 3);
    xl = xl ^ tl;

    tl = (xl ^ (xh >> 10)) & 0x00008040;
    xh = xh ^ (tl << 10);
    xl = xl ^ tl;

    tl = (xl ^ (xh >> 17)) & 0x00000080;
    xh = xh ^ (tl << 17);
    xl = xl ^ tl;

    B[0] = (xh >> 24) as i32;
    B[n] = (xh >> 16) as i32;
    B[2 * n] = (xh >> 8) as i32;
    B[3 * n] = xh as i32;
    B[4 * n] = (xl >> 24) as i32;
    B[5 * n] = (xl >> 16) as i32;
    B[6 * n] = (xl >> 8) as i32;
    B[7 * n] = xl as i32;
}

/* Next is the basic "recursive" method. Decided not to include this in
HD. It's too similar to transpose8rS32, which is a little better (probably).
   Instruction counts for the calculation part:
   16 ANDs
   10 shifts
   10 ORs
    9 mask generation
   --
   45 total (recursive method, direct placement at each step) */
pub fn transpose8r32(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
// Load the array and pack it into x and y.
    let mut x: u64 = ((A[0] << 24) | (A[m] << 16) | (A[2 * m] << 8) | A[3 * m]) as u64;
    let mut y: u64 = ((A[4 * m] << 24) | (A[5 * m] << 16) | (A[6 * m] << 8) | A[7 * m]) as u64;

    x = (x & 0xAA55AA55) | ((x & 0x00AA00AA) << 7) |
        ((x >> 7) & 0x00AA00AA);
    y = (y & 0xAA55AA55) | ((y & 0x00AA00AA) << 7) |
        ((y >> 7) & 0x00AA00AA);

    x = (x & 0xCCCC3333) | ((x & 0x0000CCCC) << 14) |
        ((x >> 14) & 0x0000CCCC);
    y = (y & 0xCCCC3333) | ((y & 0x0000CCCC) << 14) |
        ((y >> 14) & 0x0000CCCC);

    let t = (x & 0xF0F0F0F0) | ((y >> 4) & 0x0F0F0F0F);
    y = ((x << 4) & 0xF0F0F0F0) | (y & 0x0F0F0F0F);
    x = t;

    B[0] = (x >> 24) as i32;
    B[n] = (x >> 16) as i32;
    B[2 * n] = (x >> 8) as i32;
    B[3 * n] = x as i32;
    B[4 * n] = (y >> 24) as i32;
    B[5 * n] = (y >> 16) as i32;
    B[6 * n] = (y >> 8) as i32;
    B[7 * n] = y as i32;
}

/* This is transpose8r32 but using the GLS method of bit field swapping.
   Instruction counts for the calculation part:
    8 ANDs
   12 XORs
   10 shifts
    2 ORs
    5 mask generation
   --
   37 total (recursive method using GLS's bit swapping) */
pub fn transpose8rS32(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
// Load the array and pack it into x and y.
    let mut x: u64 = ((A[0] << 24) | (A[m] << 16) | (A[2 * m] << 8) | A[3 * m]) as u64;
    let mut y: u64 = ((A[4 * m] << 24) | (A[5 * m] << 16) | (A[6 * m] << 8) | A[7 * m]) as u64;

    let mut t = (x ^ (x >> 7)) & 0x00AA00AA;
    x = x ^ t ^ (t << 7);
    t = (y ^ (y >> 7)) & 0x00AA00AA;
    y = y ^ t ^ (t << 7);

    t = (x ^ (x >> 14)) & 0x0000CCCC;
    x = x ^ t ^ (t << 14);
    t = (y ^ (y >> 14)) & 0x0000CCCC;
    y = y ^ t ^ (t << 14);

    t = (x & 0xF0F0F0F0) | ((y >> 4) & 0x0F0F0F0F);
    y = ((x << 4) & 0xF0F0F0F0) | (y & 0x0F0F0F0F);
    x = t;

    B[0] = (x >> 24) as i32;
    B[n] = (x >> 16) as i32;
    B[2 * n] = (x >> 8) as i32;
    B[3 * n] = x as i32;
    B[4 * n] = (y >> 24) as i32;
    B[5 * n] = (y >> 16) as i32;
    B[6 * n] = (y >> 8) as i32;
    B[7 * n] = y as i32;
}

/* This is transpose8rS32 done "backwards" (coarse to fine granularity).
Why? Just to show that this works.
   Instruction counts for the calculation part:
    8 ANDs
   12 XORs
   10 shifts
    2 ORs
    5 mask generation
   --
   37 total (recursive method in reverse, using GLS's bit swapping) */
pub fn transpose8rSr32(A: Vec<i32>, m: usize, n: usize, mut B: Vec<i32>) {
// Load the array and pack it into x and y.
    let mut x: u64 = ((A[0] << 24) | (A[m] << 16) | (A[2 * m] << 8) | A[3 * m]) as u64;
    let mut y: u64 = ((A[4 * m] << 24) | (A[5 * m] << 16) | (A[6 * m] << 8) | A[7 * m]) as u64;

    let mut t = (x & 0xF0F0F0F0) | ((y >> 4) & 0x0F0F0F0F);
    y = ((x << 4) & 0xF0F0F0F0) | (y & 0x0F0F0F0F);
    x = t;

    t = (x ^ (x >> 14)) & 0x0000CCCC;
    x = x ^ t ^ (t << 14);
    t = (y ^ (y >> 14)) & 0x0000CCCC;
    y = y ^ t ^ (t << 14);

    t = (x ^ (x >> 7)) & 0x00AA00AA;
    x = x ^ t ^ (t << 7);
    t = (y ^ (y >> 7)) & 0x00AA00AA;
    y = y ^ t ^ (t << 7);

    B[0] = (x >> 24) as i32;
    B[n] = (x >> 16) as i32;
    B[2 * n] = (x >> 8) as i32;
    B[3 * n] = x as i32;
    B[4 * n] = (y >> 24) as i32;
    B[5 * n] = (y >> 16) as i32;
    B[6 * n] = (y >> 8) as i32;
    B[7 * n] = y as i32;
}

/* Summary of instruction counts and operation counts (instruction
counts less those for mask generation) for all the transpose8 functions
above except for transpose8b64c:

        Instructions          Operations
           Machine              Machine
       64-bit  32-bit       64-bit  32-bit
8vn      174     174          174     174      Very naive method
8b        54      84           43      74      Basic direct placement
8bS       50      81           42      74      Above with GLS bit swapping
8r        38      45           21      36      Basic recursive
8rS       26      37           18      32      Above with GLS bit swapping
8rSr              37                   32      8rS in reverse order
*/

/* Below is the original version from Guy Steele. */
pub fn transpose32a(mut a: Vec<i64>) {
    let mut j = 16;
    let mut m = 0x0000FFFF;
    loop {
        if j == 0 {
            break;
        }
        let mut k = 0;
        loop {
            if k >= 32 {
                break;
            }
            let t = (a[k] ^ (a[k | j] >> j)) & m;
            a[k] ^= t;
            a[k | j] ^= (t << j);
            k = ((k | j) + 1) & !j
        }
        j >>= 1;
        m ^= m << j;
    }
}

/* Below is essentiay the same code, but modified to apub fn certain C
expressions out of sympathy for readers who are not very familiar with
C. Also modified to use k + j rather than k | j, because + seems more
natural for this program.  None of these changes affects the number of
instructions executed. */
pub fn transpose32b(mut A: Vec<i32>) {
    let mut m = 0x0000FFFF;
    let mut j = 16;
    loop {
        if j == 0 {
            break;
        }
        let mut k = 0;
        loop {
            if k >= 32 { break; }
            let t = (A[k] ^ (A[k + j] >> j)) & m;
            A[k] = A[k] ^ t;
            A[k + j] = A[k + j] ^ (t << j);
            k = (k + j + 1) & !j
        }
        j = j >> 1;
        m = m ^ (m << j);
    }
}
/* Straight-line version of transpose32a & b. */
macro_rules! rotateright {
    ($x:expr,$k:expr) => {
        ($x >> $k) | ($x << (32 - $k))
    }
}
macro_rules! rotateleft {
    ($x:expr,$k:expr) => {
        ($x << $k) | ($x >> (32 - $k))
    }
}
macro_rules! swap {
    ($a0:expr,$a1:expr,$j:expr,$m:expr) => {
        let t =  ($a0 ^ ($a1 >> $j)) & $m;
        $a0 = $a0 ^ t;
        $a1 = $a1 ^ (t << $j);
    }
}

pub fn transpose32c(mut A: Vec<i32>, mut B: Vec<i32>) {
    let mut a0 = A[0];
    let mut a1 = A[1];
    let mut a2 = A[2];
    let mut a3 = A[3];
    let mut a4 = A[4];
    let mut a5 = A[5];
    let mut a6 = A[6];
    let mut a7 = A[7];
    let mut a8 = A[8];
    let mut a9 = A[9];
    let mut a10 = A[10];
    let mut a11 = A[11];
    let mut a12 = A[12];
    let mut a13 = A[13];
    let mut a14 = A[14];
    let mut a15 = A[15];
    let mut a16 = A[16];
    let mut a17 = A[17];
    let mut a18 = A[18];
    let mut a19 = A[19];
    let mut a20 = A[20];
    let mut a21 = A[21];
    let mut a22 = A[22];
    let mut a23 = A[23];
    let mut a24 = A[24];
    let mut a25 = A[25];
    let mut a26 = A[26];
    let mut a27 = A[27];
    let mut a28 = A[28];
    let mut a29 = A[29];
    let mut a30 = A[30];
    let mut a31 = A[31];

    let mut m = 0x0000FFFF;
    swap!(a0, a16, 16, m);
    swap!(a1, a17, 16, m);
    swap!(a2, a18, 16, m);
    swap!(a3, a19, 16, m);
    swap!(a4, a20, 16, m);
    swap!(a5, a21, 16, m);
    swap!(a6, a22, 16, m);
    swap!(a7, a23, 16, m);
    swap!(a8, a24, 16, m);
    swap!(a9, a25, 16, m);
    swap!(a10, a26, 16, m);
    swap!(a11, a27, 16, m);
    swap!(a12, a28, 16, m);
    swap!(a13, a29, 16, m);
    swap!(a14, a30, 16, m);
    swap!(a15, a31, 16, m);
    m = 0x00FF00FF;
    swap!(a0, a8, 8, m);
    swap!(a1, a9, 8, m);
    swap!(a2, a10, 8, m);
    swap!(a3, a11, 8, m);
    swap!(a4, a12, 8, m);
    swap!(a5, a13, 8, m);
    swap!(a6, a14, 8, m);
    swap!(a7, a15, 8, m);
    swap!(a16, a24, 8, m);
    swap!(a17, a25, 8, m);
    swap!(a18, a26, 8, m);
    swap!(a19, a27, 8, m);
    swap!(a20, a28, 8, m);
    swap!(a21, a29, 8, m);
    swap!(a22, a30, 8, m);
    swap!(a23, a31, 8, m);
    m = 0x0F0F0F0F;
    swap!(a0, a4, 4, m);
    swap!(a1, a5, 4, m);
    swap!(a2, a6, 4, m);
    swap!(a3, a7, 4, m);
    swap!(a8, a12, 4, m);
    swap!(a9, a13, 4, m);
    swap!(a10, a14, 4, m);
    swap!(a11, a15, 4, m);
    swap!(a16, a20, 4, m);
    swap!(a17, a21, 4, m);
    swap!(a18, a22, 4, m);
    swap!(a19, a23, 4, m);
    swap!(a24, a28, 4, m);
    swap!(a25, a29, 4, m);
    swap!(a26, a30, 4, m);
    swap!(a27, a31, 4, m);
    m = 0x33333333;
    swap!(a0, a2, 2, m);
    swap!(a1, a3, 2, m);
    swap!(a4, a6, 2, m);
    swap!(a5, a7, 2, m);
    swap!(a8, a10, 2, m);
    swap!(a9, a11, 2, m);
    swap!(a12, a14, 2, m);
    swap!(a13, a15, 2, m);
    swap!(a16, a18, 2, m);
    swap!(a17, a19, 2, m);
    swap!(a20, a22, 2, m);
    swap!(a21, a23, 2, m);
    swap!(a24, a26, 2, m);
    swap!(a25, a27, 2, m);
    swap!(a28, a30, 2, m);
    swap!(a29, a31, 2, m);
    m = 0x55555555;
    swap!(a0, a1, 1, m);
    swap!(a2, a3, 1, m);
    swap!(a4, a5, 1, m);
    swap!(a6, a7, 1, m);
    swap!(a8, a9, 1, m);
    swap!(a10, a11, 1, m);
    swap!(a12, a13, 1, m);
    swap!(a14, a15, 1, m);
    swap!(a16, a17, 1, m);
    swap!(a18, a19, 1, m);
    swap!(a20, a21, 1, m);
    swap!(a22, a23, 1, m);
    swap!(a24, a25, 1, m);
    swap!(a26, a27, 1, m);
    swap!(a28, a29, 1, m);
    swap!(a30, a31, 1, m);

    B[0] = a0;
    B[1] = a1;
    B[2] = a2;
    B[3] = a3;
    B[4] = a4;
    B[5] = a5;
    B[6] = a6;
    B[7] = a7;
    B[8] = a8;
    B[9] = a9;
    B[10] = a10;
    B[11] = a11;
    B[12] = a12;
    B[13] = a13;
    B[14] = a14;
    B[15] = a15;
    B[16] = a16;
    B[17] = a17;
    B[18] = a18;
    B[19] = a19;
    B[20] = a20;
    B[21] = a21;
    B[22] = a22;
    B[23] = a23;
    B[24] = a24;
    B[25] = a25;
    B[26] = a26;
    B[27] = a27;
    B[28] = a28;
    B[29] = a29;
    B[30] = a30;
    B[31] = a31;
}

/* Copied from GLS's note.  This is the "three shearing transformations"
method.  The code below takes 1280 ops to do the bit rearrangements
(i.e., not counting loop control, loads, stores, and indexing).  Not
competitive with the other methods.  */
pub fn transpose32d(mut a: Vec<i32>) {
    for k in 0..32 { a[k] = rotateright!(a[k], k); }
    let mut j = 16;
    let mut m: u64 = 0xFFFF0000;
    loop {
        if j == 0 {
            break;
        }
        for k in 0..j {
            let mut t = a[k] as u64 & m;
            a[k] = a[k] ^ t as i32;
            for q in (k + j..32).step_by(j) {
                let u = a[q] as u64 & m;
                a[q] = a[q] ^ u as i32 ^ t as i32;
                t = u;
            }
            a[k] = a[k] ^ t as i32;
        }
        j = j >> 1;
        m ^= m >> j;
    }
    for k in 0..32 { a[k] = rotateleft!(a[k], 31 - k); }
    for k in 0..16 {
        let t = a[k];
        a[k] = a[31 - k];
        a[31 - k] = t;
    }
}