pub fn g_pow_x_mod_p(g: u32, mut x: u32, p: i64) -> u32 {
    let mut current: i64 = 1;
    let mut mult: i64 = g as i64;

    if x == 0 {
        return 1;
    }

    while x != 0 {
        if (x & 1) > 0 {
            current = (mult * current) % p;
        }
        x >>= 1;
        mult = (mult * mult) % p;
    }
    current as u32
}

pub fn transform_value(value: &mut [u8], key: u32, key_byte: u8) {
    value[0] ^= value[0] + (key >> 00 & 0xFF) as u8 + key_byte;
    value[1] ^= value[1] + (key >> 08 & 0xFF) as u8 + key_byte;
    value[2] ^= value[2] + (key >> 16 & 0xFF) as u8 + key_byte;
    value[3] ^= value[3] + (key >> 24 & 0xFF) as u8 + key_byte;

    value[4] ^= value[4] + (key >> 00 & 0xFF) as u8 + key_byte;
    value[5] ^= value[5] + (key >> 08 & 0xFF) as u8 + key_byte;
    value[6] ^= value[6] + (key >> 16 & 0xFF) as u8 + key_byte;
    value[7] ^= value[7] + (key >> 24 & 0xFF) as u8 + key_byte;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_with_known_result() {
        assert_eq!(g_pow_x_mod_p(10, 20, 30), 10);
    }

    #[test]
    fn compare_transform_with_known_result() {
        let expected: [u8; 8] = [58, 49, 1, 1, 58, 49, 1, 1];
        let mut buf: [u8; 8] = [0; 8];
        let key = 12345;
        transform_value(&mut buf, key, (key & 7) as u8);

        assert_eq!(buf, expected);
    }
}