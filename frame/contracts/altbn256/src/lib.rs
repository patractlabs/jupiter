//! ALTBN_128 mocking

/// padding
fn padding_64(input: &[u8]) -> [u8; 64] {
    let mut res = [0; 64];
    res.copy_from_slice(&mut &input[..input.len()]);
    res
}

/// Mock add
pub fn bn256_add(input: &[u8]) -> [u8; 64] {
    padding_64(input)
}

/// Mock mul
pub fn bn256_mul(input: &[u8]) -> [u8; 64] {
    padding_64(input)
}

/// Mock pairing
pub fn bn256_pairing(_input: &[u8]) -> bool {
    true
}
