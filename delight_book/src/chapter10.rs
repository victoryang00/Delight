//shrsi t,n,k-1 Form the integer
//shri t,t,32-k 2**k – 1 if n < 0, else 0.
//add t,n,t     Add it to n,
//shrsi q,t,k   and shift right (signed).
//bge n,label      Branch if n >= 0.
//addi n,n,2**k-1  Add 2**k - 1 to n,
//shrsi n,n,k      and shift right (signed).
//shrsi q,n,k
//addze q,q

use chapter2::basics_abs;

#[derive(PartialEq,Debug)]
pub struct ms {
    M: i64,
    // Magic number
    s: i64, // and shift amount.
}

pub fn divint_magic(d: i64) -> ms {   // Must have 2 <= d <= 2**31-1
    let two31 = 0x80000000;     // 2**31.
    let mut mag = ms { M: 0, s: 0 };
    let ad = basics_abs(d);
    let t = two31 + (d >> 31);
    let anc = t - 1 - t % ad;     // Absolute value of nc.
    let mut p = 31;                 // Init. p.
    let mut q1 = two31 / anc;         // Init. q1 = 2**p/|nc|.
    let mut r1 = two31 - q1 * anc;    // Init. r1 = rem(2**p, |nc|).
    let mut q2 = two31 / ad;          // Init. q2 = 2**p/|d|.
    let mut r2 = two31 - q2 * ad;     // Init. r2 = rem(2**p, |d|).
    let mut delta=0;
    while q1 < delta || (q1 == delta && r1 == 0) {
        p = p + 1;
        q1 = 2 * q1;           // Update q1 = 2**p/|nc|.
        r1 = 2 * r1;           // Update r1 = rem(2**p, |nc|).
        if r1 >= anc {     // (Must be an unsigned
            q1 = q1 + 1;      // comparison here).
            r1 = r1 - anc;
        }
        q2 = 2 * q2;           // Update q2 = 2**p/|d|.
        r2 = 2 * r2;           // Update r2 = rem(2**p, |d|).
        if r2 >= ad {      // (Must be an unsigned
            q2 = q2 + 1;      // comparison here).
            r2 = r2 - ad;
        }
        delta = ad - r2;
    };

    mag.M = q2 + 1;
    if d < 0
    { mag.M = -mag.M; } // Magic number and
    mag.s = p - 32;            // shift amount to return.
    return mag;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_hamming(){
    let mag = ms { M: 2147483649, s: -1 };
    assert_eq!(divint_magic(1),mag);
}