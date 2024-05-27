pub fn round(x: u32, y: u32) -> u32 {
    (x + y - 1) & !(y - 1)
}