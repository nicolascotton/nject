/// FNV hash function
pub fn fnv(data: &[u8]) -> [u8; 16] {
    // https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV_hash_parameters
    const FNV_OFFSET_BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x00000100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    // loop over each byte of data
    for &byte in data {
        hash ^= u128::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn fnv_with_data_should_return_corresponding_hash() {
        // Given
        let data = b"hello world";
        // When
        let hashed = fnv(data);
        // Then
        let hashed = to_hex(&hashed);
        assert_eq!(&hashed, "c95984f170c495ba01ee83118e66933f")
    }

    fn to_hex(bytes: &[u8]) -> String {
        let mut result = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            write!(&mut result, "{:02x}", byte).unwrap();
        }
        result
    }
}
