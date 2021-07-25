/// Bound checking
pub fn boundsAdds(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a + c;
    let mut t = b + d;
    let u = a & c & !s & !(b & d & !t);
    let v = ((a ^ c) | !(a ^ s)) & (!b & !d & t);
    if (u | v) < 0 {
        s = 0x80000000;
        t = 0x7FFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_boundsAdds() {
    let test = [[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
        [3, 6, 2, 7, 5, 13], [-6, -3, -7, -2, -13, -5]
    ];
    for i in 0..3 {
        let (s, t) = boundsAdds(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(s, test[i][4]);
        assert_eq!(t, test[i][5]);
    }
}

pub fn boundsAddu(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a + c;
    let mut t = b + d;
    if s >= a && t < b {
        s = 0;
        t = 0xFFFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_boundsAddu() {
    let test = [[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
        [3, 6, 2, 7, 5, 13], [-6, -3, -7, -2, -13, -5]
    ];
    for i in 0..3 {
        let (s, t) = boundsAddu(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(s, test[i][4]);
        assert_eq!(t, test[i][5]);
    }
}

pub fn boundsSubu(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a - d;
    let mut t = b - c;
    if s > a && t <= b {
        s = 0;
        t = 0xFFFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_boundsSubu() {
    let test = [[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]
    ];
    for i in 0..1 {
        let (s, t) = boundsSubu(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(t, test[i][5]);
    }
}

pub fn bounds_minOR(mut a: i64, b: i64, mut c: i64, d: i64) -> i64 {
    let mut m = 0x80000000;
    let mut temp = 0;
    while m != 0 {
        if (!a & c & m) != 0 {
            temp = (a | m) & -m;
            if temp <= b {
                a = temp;
                break;
            }
        } else if (a & !c & m) != 0 {
            temp = (c | m) & -m;
            if temp <= d {
                c = temp;
                break;
            }
        }
        m = m >> 1;
    }
    return a | c;
}

pub fn bounds_maxAND(a: i64, mut b: i64, c: i64, mut d: i64) -> i64 {
    let mut m = 0x80000000;
    let mut temp = 0;
    while m != 0 {
        if (b & !d & m) != 0 {
            temp = (b & !m) | (m - 1);
            if temp >= a {
                b = temp;
                break;
            }
        } else if (!b & d & m) != 0 {
            temp = (d & !m) | (m - 1);
            if temp >= c {
                d = temp;
                break;
            }
        }
        m = m >> 1;
    }
    return b & d;
}

pub fn bounds_maxXOR(a: i64, mut b: i64, c: i64, mut d: i64) -> i64 {
    let mut m = 0x80000000;
    while m != 0 {
        if b & d & m!=0 {
            let mut temp = (b - m) | (m - 1);
            if temp >= a { b = temp; } else {
                temp = (d - m) | (m - 1);
                if temp >= c { d = temp; }
            }
        }
        m = m >> 1;
    }
    return b ^ d;
}

/* Speedups: "b & !d" and "!b & d" move out of the loop.
A better starting value of m is
   m = 0x80000000 >> nlz(b ^ d);
(best to have mod 32 shifts for case b ^ d = 0).
Or, use one of the methods for computing flp2(x) in sect. 3-2.
*/
pub fn bounds_brute(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let mut rmax = 0;                    // Init to 0.
    for i in a..b {
        for j in c..d {
            if (i & j) > rmax { rmax = i & j; }
        }
    }
    return rmax;
}