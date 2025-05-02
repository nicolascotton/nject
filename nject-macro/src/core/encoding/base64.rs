// Encodes bytes in base64
pub fn encode(bytes: &[u8]) -> Vec<u8> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const PADDING: u8 = b'=';
    let len = bytes.len();
    let out_len = (len + 2).div_ceil(3) * 4;
    let mut out = Vec::with_capacity(out_len);
    for chunk in bytes.chunks(3) {
        // The first 6 bits of the first byte
        let a = chunk[0] >> 2;
        // The last 2 bits of the first byte and the first 4 bits of the second byte
        let b = ((chunk[0] & 0b11) << 4)
            | if let Some(&x) = chunk.get(1) {
                x >> 4
            } else {
                0
            };
        // The last 4 bits of the second byte and the first 2 bits of the third byte
        let c = if let Some(&x) = chunk.get(1) {
            ((x & 0b1111) << 2)
                | if let Some(&y) = chunk.get(2) {
                    y >> 6
                } else {
                    0
                }
        } else {
            0
        };
        // The last 6 bits of the third byte
        let d = if let Some(&x) = chunk.get(2) {
            x & 0b111111
        } else {
            0
        };
        out.push(ALPHABET[a as usize]);
        out.push(ALPHABET[b as usize]);
        out.push(if chunk.len() > 1 {
            ALPHABET[c as usize]
        } else {
            PADDING
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[d as usize]
        } else {
            PADDING
        });
    }
    out
}

// Decodes base64 bytes.
#[cfg(test)]
pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len() * 3 / 4);
    for chunk in data.chunks(4) {
        // Convert the chunk into a 24-bit integer
        let mut n = 0u32;
        for i in 0..4 {
            let c = chunk[i] as u32;
            let index = match c {
                43 => 62,
                47 => 63,
                48..=57 => c + 4,
                61 => 0,
                65..=90 => c - 65,
                97..=122 => c - 71,
                _ => panic!("Invalid char: {c}"),
            };
            // Shift and add the index to the 24-bit integer
            n = (n << 6) | index;
        }

        // Decode the 24-bit integer into 3 bytes
        for i in (0..3).rev() {
            // Get the 8-bit value by shifting and masking
            let value = ((n >> (i * 8)) & 0xFF) as u8;
            output.push(value);
        }

        // Remove padding bytes if the chunk has padding characters
        for &c in chunk {
            if c == b'=' {
                output.pop();
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_with_data_should_return_encoded_data() {
        // Given
        let data = "lib1 :: Module";
        // When
        let encoded = encode(data.as_bytes());
        // Then
        let encoded = String::from_utf8(encoded).unwrap();
        assert_eq!(&encoded, "bGliMSA6OiBNb2R1bGU=")
    }

    #[test]
    fn decode_with_encoded_data_should_return_decoded_data() {
        // Given
        let encoded = "bGliMSA6OiBNb2R1bGU=";
        // When
        let decoded = decode(encoded.as_bytes());
        // Then
        let decoded = String::from_utf8(decoded).unwrap();
        assert_eq!(&decoded, "lib1 :: Module")
    }

    #[test]
    fn encode_with_data_should_return_decodable_string() {
        let cases = vec![
            "A",
            "0",
            "1",
            "One",
            "Hello, World !!!",
            "lib1::Module1",
            "my::very::special_crate_123::Module456<Complex>",
        ];
        for case in cases {
            // Given
            let data = case;
            // When
            let encoded = encode(data.as_bytes());
            // Then
            let expected_encoded_len = (data.len() + 2) / 3 * 4;
            assert_eq!(encoded.len(), expected_encoded_len);
            let expected_decoded_len = encoded.len() * 3 / 4;
            let decoded = decode(&encoded);
            assert!(
                decoded.len() <= expected_decoded_len,
                "{:?} <= {:?}",
                decoded.len(),
                expected_decoded_len
            );
            let decoded = String::from_utf8(decoded).unwrap();
            assert_eq!(&decoded, data)
        }
    }
}
