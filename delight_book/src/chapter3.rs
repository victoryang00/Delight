fn clp2(mut x: u32) -> u32 {
    x = x - 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    return x + 1;
}

// fn  crc32g( ) {
// int i, j, crc;
// unsigned int byte, c;
// const unsigned int g0 = 0xEDB88320, g1 = g0 >> 1,
// g2 = g0 >> 2,    g3 = g0 >> 3;
//
// i = 0;
// crc = 0xFFFFFFFF;
// while (message[i] != 0) {
// byte = message[i];                // Get next byte.
// crc = crc ^ byte;
// for (j = 1; j >= 0; j--) {        // Do two times.
// c = ((crc<<31>>31) & g3) ^ ((crc<<30>>31) & g2) ^
// ((crc<<29>>31) & g1) ^ ((crc<<28>>31) & g0);
// crc = ((unsigned)crc >> 4) ^ c;
// }
// i = i + 1;
// }
// return ~crc;
// }