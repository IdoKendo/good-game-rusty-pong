/// Computes the fletcher16 checksum  
/// Implemented according to wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
pub fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for item in data {
        sum1 = (sum1 + *item as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}
