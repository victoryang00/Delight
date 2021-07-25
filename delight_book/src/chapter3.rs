pub fn crc_clp2(mut x: i32) -> i32 {
    x = x - 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    return x + 1;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_crc_clp2() {
    assert_eq!(crc_clp2(0), 0);
}

pub fn crc_reverse(mut x: i32) -> i32 {
    x = ((x & 0x55555555) << 1) | ((x >> 1) & 0x55555555);
    x = ((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333);
    x = ((x & 0x0F0F0F0F) << 4) | ((x >> 4) & 0x0F0F0F0F);
    x = (x << 24) | ((x & 0xFF00) << 8) |
        ((x >> 8) & 0xFF00) | (x >> 24);
    return x;
}

#[allow(overflowing_literals)]
pub fn crc32a(message: &[char]) -> i32 {
    let mut i = 0;
    let mut crc = 0xFFFFFFFF;
    while message[i] != '\x00' {
        let mut byte = message[i];            // Get next byte.
        byte = crc_reverse((byte as u8) as i32) as u8 as char;         // 32-bit reversal.
        for j in 0..7 {    // Do eight times.
            if (crc ^ (byte as i32)) < 0 {
                crc = (crc << 1) ^ 0x04C11DB7;
            } else { crc = crc << 1; }
            byte = ((byte as u8) << 1) as char;          // Ready next msg bit.
        }
        i = i + 1;
    }
    return crc_reverse(!crc);
}

/* This is the basic CRC-32 calculation with some optimization but no
table lookup. The the byte reversal is avoided by shifting the crc reg
right instead of left and by using a reversed 32-bit word to represent
the polynomial.
   When compiled to Cyclops with GCC, this function executes in 8 + 72n
instructions, where n is the number of bytes in the input message. It
should be doable in 4 + 61n instructions.
   If the inner loop is strung out (approx. 5*8 = 40 instructions),
it would take about 6 + 46n instructions. */
#[allow(overflowing_literals)]
pub fn crc32b(message: &[char]) -> i32 {
    let mut i = 0;
    let mut crc = 0xFFFFFFFF;
    while message[i] != '\x00' {
        let mut byte = message[i];            // Get next byte.
        crc = crc ^ (byte as u8 as i32);
        for _ in 7..0 {    // Do eight times.
            let mask = -(crc & 1);
            crc = (crc >> 1) ^ (0xEDB88320 & mask);
        }
        i = i + 1;
    }
    !crc
}

/* This is derived from crc32b but does table lookup. First the table
itself is calculated, if it has not yet been set up.
Not counting the table setup (which would probably be a separate
function), when compiled to Cyclops with GCC, this function executes in
7 + 13n instructions, where n is the number of bytes in the input
message. It should be doable in 4 + 9n instructions. In any , two
of the 13 or 9 instrucions are load byte.
 => This is Figure 14-7 in the text. */
#[allow(overflowing_literals)]
pub fn crc32c(message: &[char]) -> i32 {
    let mut table: [char; 256] = ['\x00'; 256];
    if table[1] == '\x00' {
        for byte in 0..255 {
            let mut crc = byte;
            for _ in 7..0 {    // Do eight times.
                let mask = -(crc as i32 & 1);
                crc = (crc >> 1) ^ (0xEDB88320 & mask);
            }
            table[byte as usize] = crc as u8 as char;
        }
    }

    /* Through with table setup, now calculate the CRC. */

    let mut crc = 0xFFFFFFFF;
    for byte in message {
        crc = (crc >> 8) ^ table[((crc ^ *byte as i64) & 0xFF) as usize] as i64;
    }
    !crc as i32
}

/* This is crc32b modified to load the message a fullword at a time.
It assumes the message is word aligned and consists of an integral
number of words before the 0-byte that marks the end of the message.
   This works only on a little-endian machine.
   Not counting the table setup (which would probably be a separate
function), this function should be doable in 3 + 22w instructions, where
w is the number of fullwords in the input message. This is equivalent to
3 + 5.5n instructions, where n is the number of bytes. 1.25 of those 5.5
instructions are loads.
   This is Exercise 1 in the text. C.f. Christopher Dannemiller,
who got it from Linux Source base, */
#[allow(overflowing_literals)]
pub fn crc32cx(message: &mut [char]) -> i32 {
    let mut table: [char; 256] = ['\x00'; 256];

    /* Set up the table, if necessary. */
    if table[1] == '\x00' {
        for byte in 0..255 {
            let mut crc = byte;
            for _ in 7..0 {    // Do eight times.
                let mask = -(crc as i32 & 1);
                crc = (crc >> 1) ^ (0xEDB88320 & mask);
            }
            table[byte as usize] = crc as u8 as char;
        }
    }

    /* Through with table setup, now calculate the CRC. */

    let mut crc = 0xFFFFFFFF;
    for word in message.into_iter().step_by(4) {
        crc = crc ^ *word as i64;
        crc = (crc >> 8) ^ table[(crc & 0xFF) as usize] as i64;
        crc = (crc >> 8) ^ table[(crc & 0xFF) as usize] as i64;
        crc = (crc >> 8) ^ table[(crc & 0xFF) as usize] as i64;
        crc = (crc >> 8) ^ table[(crc & 0xFF) as usize] as i64;
    }
    !crc as i32
}

/* This is like crc32c (does table lookup) but it processes two bytes at
a time, except for a possible odd byte at the beginning or end of the
message. The table size is 65536 words.
   Not counting the table setup (which would probably be a separate
function), when compiled to Cyclops with GCC, this function executes in
14 + 14n instructions, where n is the number of halfwords in the input
message. This assumes there are no odd bytes at either end.
   Note: When accessing the table for a single byte b, the entry to use
is b << 8. I.e., if the byte is the letter 'a', the entry to use is that
with index 0x6100, not 0x0061. */
#[allow(overflowing_literals)]
pub fn crc32d(message: &[char]) -> i32 {
    let mut table: [char; 65534] = ['\x00'; 65534];

    if table[1] == '\x00' {         // If table has not yet
// been set up:
        for half in 0..65534 {
            let mut crc = half;
            for j in 15..0 {    // Do 15 times.
                let mut mask = -(crc as i32 & 1);
                crc = (crc >> 1) ^ (0xEDB88320 & mask);
            }
            table[half as usize] = crc as u8 as char;
        }
    }
    let mut crc = 0xFFFFFFFF;

// First, if message is aligned on an odd address,
// take care of the first byte.

    let mut i = (message[0] as u8 & 1) as i32;    // Start of halfwords.
    if i != 0 {                     // If i == 1:
        let byte = message[0];
        if byte == '\x00' {
            return 0;
        }  // If null message.
        crc = (crc >> 8) ^ table[((byte as i64 ^ 0xFF) << 8) as usize] as i64;
    }

// Next process the message two bytes at a time as long
// as both bytes are nonzero.
    let mut half = 0;
    while true {
        half = message[i as usize] as i32;
        if half <= 0xFF || (half & 0xFF) == 0 {
            break;
        }
        crc = (crc >> 16) ^ (table[((crc ^ half as i64) & 0xFFFF) as usize]) as i64;
        i = i + 2;
    }

// Lastly, process the odd byte at the end, if any.
// "half" is of the form 00xx, xx00, or 0000.

    if half & 0xFF != 0 {
        crc = (crc >> 8) ^ table[(((crc ^ half as i64) & 0xFF) << 8) as usize] as i64;
    }

    !crc as i32
}

/* This is sort of like the table lookup version (crc32c), but using
a 16-way switch statement instead.
   When compiled to Cyclops with GCC, this function executes in 6 + 38n
instructions, where n is the number of bytes in the input message. The
38 instructions per byte include 3 loads and 5 branches (not good). It
is actually 6 branches if you count the unnecessary one that GCC
generates because it isn't smart enough to know that the switch argument
cannot exceed 15. */
pub fn crc32e(message: &[char]) -> i32 {
    let g0 = 0xEDB88320;
    let g1 = g0 >> 1;
    let g2 = g0 >> 2;
    let g3 = g0 >> 3;

    let mut crc = 0xFFFFFFFF;
    let mut c = 0;
    for byte in message {   // Get next byte.
        crc = crc ^ *byte as i64;
        for j in 0..1 {        // Do two times.
            match crc & 0xF {
                0 => c = 0,
                1 => c = g3,
                2 => c = g2,
                3 => c = g2 ^ g3,
                4 => c = g1,
                5 => c = g1 ^ g3,
                6 => c = g1 ^ g2,
                7 => c = g1 ^ g2 ^ g3,
                8 => c = g0,
                9 => c = g0 ^ g3,
                10 => c = g0 ^ g2,
                11 => c = g0 ^ g2 ^ g3,
                12 => c = g0 ^ g1,
                13 => c = g0 ^ g1 ^ g3,
                14 => c = g0 ^ g1 ^ g2,
                15 => c = g0 ^ g1 ^ g2 ^ g3,
                _ => {}
            }
            crc = (crc >> 4) ^ c;
        }
    }
    !crc as i32
}

/* This is sort of like the table lookup version (crc32c), but using
a 256-way switch statement instead.
   The expressions for g1, g2, ..., g7 are determined by examining what
the CRC-32 algorithm does to a byte of value 1, 2, 4, 8, 16, 32, 64, and
128, respectively. g6 and g7 are complicated because the rightmost 1-bit
in g0 enters the picture.
   We rely on the compiler to evaluate, at compile time, all the
expressions involving the g's. They are the table values used in
function crc32c above (i.e., g7 = table[1], g6 = table[2], g5 =
table[4], etc.).
   This idea of using a switch statement is a dumb idea if a compiler is
used, because the compiler (GCC anyway) implements the switch statement
with a 256-word label table. Thus the program still has the load from a
table, and it is larger than crc32c by the three words of instructions
used at each  statement (two instructions to load the constant, plus
a branch). Howe=>er, since each  statement has the same amount o,
code (three w, the label table cou=>d be avoided if the program wer,
coded in assembly language. But it would still have poor
performance.
   At any rate, when compiled to Cyclops with GCC, this function
executes 6 + 19n instructions, where n is the number of bytes in the
input message. The 19 includes 2 loads and 3 branches (per byte), not
counting the one GCC generates to check that the switch argument doesn't
exceed 255 (it can't exceed 255). */
pub fn crc32f(message: &[char]) -> i32 {
    let g0 = 0xEDB88320;
    let g1 = g0 >> 1;
    let g2 = g0 >> 2;
    let g3 = g0 >> 3;
    let g4 = g0 >> 4;
    let g5 = g0 >> 5;
    let g6 = (g0 >> 6) ^ g0;
    let g7 = ((g0 >> 6) ^ g0) >> 1;

    let mut i = 0;
    let mut c = 0;
    let mut crc = 0xFFFFFFFF;
    for byte in message {   // Get next byte.
        crc = crc ^ *byte as i64;
        match crc & 0xFF {
            0 => c = 0,
            1 => c = g7,
            2 => c = g6,
            3 => c = g6 ^ g7,
            4 => c = g5,
            5 => c = g5 ^ g7,
            6 => c = g5 ^ g6,
            7 => c = g5 ^ g6 ^ g7,
            8 => c = g4,
            9 => c = g4 ^ g7,
            10 => c = g4 ^ g6,
            11 => c = g4 ^ g6 ^ g7,
            12 => c = g4 ^ g5,
            13 => c = g4 ^ g5 ^ g7,
            14 => c = g4 ^ g5 ^ g6,
            15 => c = g4 ^ g5 ^ g6 ^ g7,
            16 => c = g3,
            17 => c = g3 ^ g7,
            18 => c = g3 ^ g6,
            19 => c = g3 ^ g6 ^ g7,
            20 => c = g3 ^ g5,
            21 => c = g3 ^ g5 ^ g7,
            22 => c = g3 ^ g5 ^ g6,
            23 => c = g3 ^ g5 ^ g6 ^ g7,
            24 => c = g3 ^ g4,
            25 => c = g3 ^ g4 ^ g7,
            26 => c = g3 ^ g4 ^ g6,
            27 => c = g3 ^ g4 ^ g6 ^ g7,
            28 => c = g3 ^ g4 ^ g5,
            29 => c = g3 ^ g4 ^ g5 ^ g7,
            30 => c = g3 ^ g4 ^ g5 ^ g6,
            31 => c = g3 ^ g4 ^ g5 ^ g6 ^ g7,
            32 => c = g2,
            33 => c = g2 ^ g7,
            34 => c = g2 ^ g6,
            35 => c = g2 ^ g6 ^ g7,
            36 => c = g2 ^ g5,
            37 => c = g2 ^ g5 ^ g7,
            38 => c = g2 ^ g5 ^ g6,
            39 => c = g2 ^ g5 ^ g6 ^ g7,
            40 => c = g2 ^ g4,
            41 => c = g2 ^ g4 ^ g7,
            42 => c = g2 ^ g4 ^ g6,
            43 => c = g2 ^ g4 ^ g6 ^ g7,
            44 => c = g2 ^ g4 ^ g5,
            45 => c = g2 ^ g4 ^ g5 ^ g7,
            46 => c = g2 ^ g4 ^ g5 ^ g6,
            47 => c = g2 ^ g4 ^ g5 ^ g6 ^ g7,
            48 => c = g2 ^ g3,
            49 => c = g2 ^ g3 ^ g7,
            50 => c = g2 ^ g3 ^ g6,
            51 => c = g2 ^ g3 ^ g6 ^ g7,
            52 => c = g2 ^ g3 ^ g5,
            53 => c = g2 ^ g3 ^ g5 ^ g7,
            54 => c = g2 ^ g3 ^ g5 ^ g6,
            55 => c = g2 ^ g3 ^ g5 ^ g6 ^ g7,
            56 => c = g2 ^ g3 ^ g4,
            57 => c = g2 ^ g3 ^ g4 ^ g7,
            58 => c = g2 ^ g3 ^ g4 ^ g6,
            59 => c = g2 ^ g3 ^ g4 ^ g6 ^ g7,
            60 => c = g2 ^ g3 ^ g4 ^ g5,
            61 => c = g2 ^ g3 ^ g4 ^ g5 ^ g7,
            62 => c = g2 ^ g3 ^ g4 ^ g5 ^ g6,
            63 => c = g2 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            64 => c = g1,
            65 => c = g1 ^ g7,
            66 => c = g1 ^ g6,
            67 => c = g1 ^ g6 ^ g7,
            68 => c = g1 ^ g5,
            69 => c = g1 ^ g5 ^ g7,
            70 => c = g1 ^ g5 ^ g6,
            71 => c = g1 ^ g5 ^ g6 ^ g7,
            72 => c = g1 ^ g4,
            73 => c = g1 ^ g4 ^ g7,
            74 => c = g1 ^ g4 ^ g6,
            75 => c = g1 ^ g4 ^ g6 ^ g7,
            76 => c = g1 ^ g4 ^ g5,
            77 => c = g1 ^ g4 ^ g5 ^ g7,
            78 => c = g1 ^ g4 ^ g5 ^ g6,
            79 => c = g1 ^ g4 ^ g5 ^ g6 ^ g7,
            80 => c = g1 ^ g3,
            81 => c = g1 ^ g3 ^ g7,
            82 => c = g1 ^ g3 ^ g6,
            83 => c = g1 ^ g3 ^ g6 ^ g7,
            84 => c = g1 ^ g3 ^ g5,
            85 => c = g1 ^ g3 ^ g5 ^ g7,
            86 => c = g1 ^ g3 ^ g5 ^ g6,
            87 => c = g1 ^ g3 ^ g5 ^ g6 ^ g7,
            88 => c = g1 ^ g3 ^ g4,
            89 => c = g1 ^ g3 ^ g4 ^ g7,
            90 => c = g1 ^ g3 ^ g4 ^ g6,
            91 => c = g1 ^ g3 ^ g4 ^ g6 ^ g7,
            92 => c = g1 ^ g3 ^ g4 ^ g5,
            93 => c = g1 ^ g3 ^ g4 ^ g5 ^ g7,
            94 => c = g1 ^ g3 ^ g4 ^ g5 ^ g6,
            95 => c = g1 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            96 => c = g1 ^ g2,
            97 => c = g1 ^ g2 ^ g7,
            98 => c = g1 ^ g2 ^ g6,
            99 => c = g1 ^ g2 ^ g6 ^ g7,
            100 => c = g1 ^ g2 ^ g5,
            101 => c = g1 ^ g2 ^ g5 ^ g7,
            102 => c = g1 ^ g2 ^ g5 ^ g6,
            103 => c = g1 ^ g2 ^ g5 ^ g6 ^ g7,
            104 => c = g1 ^ g2 ^ g4,
            105 => c = g1 ^ g2 ^ g4 ^ g7,
            106 => c = g1 ^ g2 ^ g4 ^ g6,
            107 => c = g1 ^ g2 ^ g4 ^ g6 ^ g7,
            108 => c = g1 ^ g2 ^ g4 ^ g5,
            109 => c = g1 ^ g2 ^ g4 ^ g5 ^ g7,
            110 => c = g1 ^ g2 ^ g4 ^ g5 ^ g6,
            111 => c = g1 ^ g2 ^ g4 ^ g5 ^ g6 ^ g7,
            112 => c = g1 ^ g2 ^ g3,
            113 => c = g1 ^ g2 ^ g3 ^ g7,
            114 => c = g1 ^ g2 ^ g3 ^ g6,
            115 => c = g1 ^ g2 ^ g3 ^ g6 ^ g7,
            116 => c = g1 ^ g2 ^ g3 ^ g5,
            117 => c = g1 ^ g2 ^ g3 ^ g5 ^ g7,
            118 => c = g1 ^ g2 ^ g3 ^ g5 ^ g6,
            119 => c = g1 ^ g2 ^ g3 ^ g5 ^ g6 ^ g7,
            120 => c = g1 ^ g2 ^ g3 ^ g4,
            121 => c = g1 ^ g2 ^ g3 ^ g4 ^ g7,
            122 => c = g1 ^ g2 ^ g3 ^ g4 ^ g6,
            123 => c = g1 ^ g2 ^ g3 ^ g4 ^ g6 ^ g7,
            124 => c = g1 ^ g2 ^ g3 ^ g4 ^ g5,
            125 => c = g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g7,
            126 => c = g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6,
            127 => c = g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            128 => c = g0,
            129 => c = g0 ^ g7,
            130 => c = g0 ^ g6,
            131 => c = g0 ^ g6 ^ g7,
            132 => c = g0 ^ g5,
            133 => c = g0 ^ g5 ^ g7,
            134 => c = g0 ^ g5 ^ g6,
            135 => c = g0 ^ g5 ^ g6 ^ g7,
            136 => c = g0 ^ g4,
            137 => c = g0 ^ g4 ^ g7,
            138 => c = g0 ^ g4 ^ g6,
            139 => c = g0 ^ g4 ^ g6 ^ g7,
            140 => c = g0 ^ g4 ^ g5,
            141 => c = g0 ^ g4 ^ g5 ^ g7,
            142 => c = g0 ^ g4 ^ g5 ^ g6,
            143 => c = g0 ^ g4 ^ g5 ^ g6 ^ g7,
            144 => c = g0 ^ g3,
            145 => c = g0 ^ g3 ^ g7,
            146 => c = g0 ^ g3 ^ g6,
            147 => c = g0 ^ g3 ^ g6 ^ g7,
            148 => c = g0 ^ g3 ^ g5,
            149 => c = g0 ^ g3 ^ g5 ^ g7,
            150 => c = g0 ^ g3 ^ g5 ^ g6,
            151 => c = g0 ^ g3 ^ g5 ^ g6 ^ g7,
            152 => c = g0 ^ g3 ^ g4,
            153 => c = g0 ^ g3 ^ g4 ^ g7,
            154 => c = g0 ^ g3 ^ g4 ^ g6,
            155 => c = g0 ^ g3 ^ g4 ^ g6 ^ g7,
            156 => c = g0 ^ g3 ^ g4 ^ g5,
            157 => c = g0 ^ g3 ^ g4 ^ g5 ^ g7,
            158 => c = g0 ^ g3 ^ g4 ^ g5 ^ g6,
            159 => c = g0 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            160 => c = g0 ^ g2,
            161 => c = g0 ^ g2 ^ g7,
            162 => c = g0 ^ g2 ^ g6,
            163 => c = g0 ^ g2 ^ g6 ^ g7,
            164 => c = g0 ^ g2 ^ g5,
            165 => c = g0 ^ g2 ^ g5 ^ g7,
            166 => c = g0 ^ g2 ^ g5 ^ g6,
            167 => c = g0 ^ g2 ^ g5 ^ g6 ^ g7,
            168 => c = g0 ^ g2 ^ g4,
            169 => c = g0 ^ g2 ^ g4 ^ g7,
            170 => c = g0 ^ g2 ^ g4 ^ g6,
            171 => c = g0 ^ g2 ^ g4 ^ g6 ^ g7,
            172 => c = g0 ^ g2 ^ g4 ^ g5,
            173 => c = g0 ^ g2 ^ g4 ^ g5 ^ g7,
            174 => c = g0 ^ g2 ^ g4 ^ g5 ^ g6,
            175 => c = g0 ^ g2 ^ g4 ^ g5 ^ g6 ^ g7,
            176 => c = g0 ^ g2 ^ g3,
            177 => c = g0 ^ g2 ^ g3 ^ g7,
            178 => c = g0 ^ g2 ^ g3 ^ g6,
            179 => c = g0 ^ g2 ^ g3 ^ g6 ^ g7,
            180 => c = g0 ^ g2 ^ g3 ^ g5,
            181 => c = g0 ^ g2 ^ g3 ^ g5 ^ g7,
            182 => c = g0 ^ g2 ^ g3 ^ g5 ^ g6,
            183 => c = g0 ^ g2 ^ g3 ^ g5 ^ g6 ^ g7,
            184 => c = g0 ^ g2 ^ g3 ^ g4,
            185 => c = g0 ^ g2 ^ g3 ^ g4 ^ g7,
            186 => c = g0 ^ g2 ^ g3 ^ g4 ^ g6,
            187 => c = g0 ^ g2 ^ g3 ^ g4 ^ g6 ^ g7,
            188 => c = g0 ^ g2 ^ g3 ^ g4 ^ g5,
            189 => c = g0 ^ g2 ^ g3 ^ g4 ^ g5 ^ g7,
            190 => c = g0 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6,
            191 => c = g0 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            192 => c = g0 ^ g1,
            193 => c = g0 ^ g1 ^ g7,
            194 => c = g0 ^ g1 ^ g6,
            195 => c = g0 ^ g1 ^ g6 ^ g7,
            196 => c = g0 ^ g1 ^ g5,
            197 => c = g0 ^ g1 ^ g5 ^ g7,
            198 => c = g0 ^ g1 ^ g5 ^ g6,
            199 => c = g0 ^ g1 ^ g5 ^ g6 ^ g7,
            200 => c = g0 ^ g1 ^ g4,
            201 => c = g0 ^ g1 ^ g4 ^ g7,
            202 => c = g0 ^ g1 ^ g4 ^ g6,
            203 => c = g0 ^ g1 ^ g4 ^ g6 ^ g7,
            204 => c = g0 ^ g1 ^ g4 ^ g5,
            205 => c = g0 ^ g1 ^ g4 ^ g5 ^ g7,
            206 => c = g0 ^ g1 ^ g4 ^ g5 ^ g6,
            207 => c = g0 ^ g1 ^ g4 ^ g5 ^ g6 ^ g7,
            208 => c = g0 ^ g1 ^ g3,
            209 => c = g0 ^ g1 ^ g3 ^ g7,
            210 => c = g0 ^ g1 ^ g3 ^ g6,
            211 => c = g0 ^ g1 ^ g3 ^ g6 ^ g7,
            212 => c = g0 ^ g1 ^ g3 ^ g5,
            213 => c = g0 ^ g1 ^ g3 ^ g5 ^ g7,
            214 => c = g0 ^ g1 ^ g3 ^ g5 ^ g6,
            215 => c = g0 ^ g1 ^ g3 ^ g5 ^ g6 ^ g7,
            216 => c = g0 ^ g1 ^ g3 ^ g4,
            217 => c = g0 ^ g1 ^ g3 ^ g4 ^ g7,
            218 => c = g0 ^ g1 ^ g3 ^ g4 ^ g6,
            219 => c = g0 ^ g1 ^ g3 ^ g4 ^ g6 ^ g7,
            220 => c = g0 ^ g1 ^ g3 ^ g4 ^ g5,
            221 => c = g0 ^ g1 ^ g3 ^ g4 ^ g5 ^ g7,
            222 => c = g0 ^ g1 ^ g3 ^ g4 ^ g5 ^ g6,
            223 => c = g0 ^ g1 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            224 => c = g0 ^ g1 ^ g2,
            225 => c = g0 ^ g1 ^ g2 ^ g7,
            226 => c = g0 ^ g1 ^ g2 ^ g6,
            227 => c = g0 ^ g1 ^ g2 ^ g6 ^ g7,
            228 => c = g0 ^ g1 ^ g2 ^ g5,
            229 => c = g0 ^ g1 ^ g2 ^ g5 ^ g7,
            230 => c = g0 ^ g1 ^ g2 ^ g5 ^ g6,
            231 => c = g0 ^ g1 ^ g2 ^ g5 ^ g6 ^ g7,
            232 => c = g0 ^ g1 ^ g2 ^ g4,
            233 => c = g0 ^ g1 ^ g2 ^ g4 ^ g7,
            234 => c = g0 ^ g1 ^ g2 ^ g4 ^ g6,
            235 => c = g0 ^ g1 ^ g2 ^ g4 ^ g6 ^ g7,
            236 => c = g0 ^ g1 ^ g2 ^ g4 ^ g5,
            237 => c = g0 ^ g1 ^ g2 ^ g4 ^ g5 ^ g7,
            238 => c = g0 ^ g1 ^ g2 ^ g4 ^ g5 ^ g6,
            239 => c = g0 ^ g1 ^ g2 ^ g4 ^ g5 ^ g6 ^ g7,
            240 => c = g0 ^ g1 ^ g2 ^ g3,
            241 => c = g0 ^ g1 ^ g2 ^ g3 ^ g7,
            242 => c = g0 ^ g1 ^ g2 ^ g3 ^ g6,
            243 => c = g0 ^ g1 ^ g2 ^ g3 ^ g6 ^ g7,
            244 => c = g0 ^ g1 ^ g2 ^ g3 ^ g5,
            245 => c = g0 ^ g1 ^ g2 ^ g3 ^ g5 ^ g7,
            246 => c = g0 ^ g1 ^ g2 ^ g3 ^ g5 ^ g6,
            247 => c = g0 ^ g1 ^ g2 ^ g3 ^ g5 ^ g6 ^ g7,
            248 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4,
            249 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g7,
            250 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g6,
            251 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g6 ^ g7,
            252 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g5,
            253 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g7,
            254 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6,
            255 => c = g0 ^ g1 ^ g2 ^ g3 ^ g4 ^ g5 ^ g6 ^ g7,
            _ => {}
        } // end switch
        crc = (crc >> 8) ^ c;
        i = i + 1;
    }
    !crc as i32
}

pub fn crc32g(message: &[char]) -> i32 {
    let g0 = 0xEDB88320;
    let g1 = g0 >> 1;
    let g2 = g0 >> 2;
    let g3 = g0 >> 3;

    let mut i: usize = 0;
    let mut crc = 0xFFFFFFFF;
    while message[i] != '\x00' {
        let byte = message[i];                // Get next byte.
        crc = crc ^ byte as i64;
        for j in 1..0 {        // Do two times.
            let c = ((crc << 31 >> 31) & g3) ^ ((crc << 30 >> 31) & g2) ^
                ((crc << 29 >> 31) & g1) ^ ((crc << 28 >> 31) & g0);
            crc = (crc >> 4) ^ c;
        }
        i = i + 1;
    }
    !crc as i32
}

/* This is derived from crc32f by constructing the constant c using
algebraic expressions involving the rightmost eight bits of the crc
register, rather than using a 256-way switch statement.
   We rely on the compiler to compute the constants g>>1, g>>2, etc.,
and load them into registers ahead of the loops. Note that crc is now a
SIGNED integer, so the right shifts of 31 are sign-propagating shifts.
   When compiled to Cyclops with GCC, this function executes in 22 + 38n
instructions, where n is the number of bytes in the input message. There
is only one load and one branch executed per byte. */
pub fn crc32h(message: &[char]) -> i64 {
    let g0 = 0xEDB88320;
    let g1 = g0 >> 1;
    let g2 = g0 >> 2;
    let g3 = g0 >> 3;
    let g4 = g0 >> 4;
    let g5 = g0 >> 5;
    let g6 = (g0 >> 6) ^ g0;
    let g7 = ((g0 >> 6) ^ g0) >> 1;

    let mut i = 0;
    let mut crc = 0xFFFFFFFF;
    let mut byte;
    while (byte = message[i]) != () {    // Get next byte.
        crc = crc ^ byte as i64;
        let c = ((crc << 31 >> 31) & g7) ^ ((crc << 30 >> 31) & g6) ^
            ((crc << 29 >> 31) & g5) ^ ((crc << 28 >> 31) & g4) ^
            ((crc << 27 >> 31) & g3) ^ ((crc << 26 >> 31) & g2) ^
            ((crc << 25 >> 31) & g1) ^ ((crc << 24 >> 31) & g0);
        crc = (crc >> 8) ^ c;
        i = i + 1;
    }
    return !crc;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_crc() {
    assert_eq!(crc32a(&['\x00']), 0);
    assert_eq!(crc32b(&['\x00']), 0);
    assert_eq!(crc32c(&['\x00']), -16777216);
    assert_eq!(crc32cx(&mut ['\x00']), -1);
    assert_eq!(crc32d(&['\x00']), 0);
    assert_eq!(crc32e(&['\x00']), 1304293916);
    assert_eq!(crc32f(&['\x00']), -771559539);
    assert_eq!(crc32g(&['\x00']), 0);
    assert_eq!(crc32h(&['\x00']), -4294967296);
}

