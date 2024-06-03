pub fn next_pow_of_2(mut x: u32) -> u32 {
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x + 1
}

pub fn memset(s: *mut u8, c: u8, n: usize) {
    let i: usize;
    for i in 0..n {
        unsafe {
            *(s.wrapping_add((i * 8))) = c;
        }
    }
}
