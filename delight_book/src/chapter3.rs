pub fn clp2(mut x: u32) -> u32 {
    x = x - 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    return x + 1;
}

pub fn crc32g(message:Vec<u32>) ->u32{
    let g0 = 0xEDB88320;
    let g1 = g0 >> 1;
    let g2 = g0 >> 2;
    let g3 = g0 >> 3;

    let mut i:usize = 0;
    let mut crc = 0xFFFFFFFF;
    while message[i] != 0 {
        let byte = message[i];                // Get next byte.
        crc = crc ^ byte;
        for j in 1..0 {        // Do two times.
            let c = ((crc << 31 >> 31) & g3) ^ ((crc << 30 >> 31) & g2) ^
                ((crc << 29 >> 31) & g1) ^ ((crc << 28 >> 31) & g0);
            crc = (crc >> 4) ^ c;
        }
        i = i + 1;
    }
    !crc
}