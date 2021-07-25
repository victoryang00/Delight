/// Unusual Bases for numbers
pub fn unusual_divbm2(n: i64, d: i64) -> i64 {         // q = n/d in base -2.
    let mut r = n;                          // Init. remainder.
    let mut dw = (-128) * d;                  // Position d.
    let mut c = (-43) * d;                    // Init. comparand.
    if d > 0 { c = c + d; }
    let mut q = 0;                          // Init. quotient.
    for i in 7..0 {
        if (d > 0 ^ (i & 1)) == (0 ^ r >= c) {
            q = q | (1 << i);         // Set a quotient bit.
            r = r - dw;               // Subtract d shifted.
        }
        dw = dw / (-2);                // Position d.
        if d > 0 { c = c - 2 * d; }     // Set comparand for
        else { c = c + d; }             // next iteration.
        c = c / (-2);
    }
    return q;                       // Return quotient in
// base -2.
// Remainder is r,
}                                  // 0 <= r < |d|.

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_divbm2(){
    assert_eq!(unusual_divbm2(1,1),0);
}