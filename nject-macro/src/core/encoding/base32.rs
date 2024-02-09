/// Encodes a string in base32. **Useful for filename**.
#[cfg(test)]
pub fn encode(input: &[u8]) -> Vec<u8> {
    // The base32 alphabet
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    // The number of bits per character
    const BITS_PER_CHAR: usize = 5;
    // The output string
    let mut output = Vec::with_capacity(input.len() * 2);
    // The buffer to store the bits
    let mut buffer: u64 = 0;
    // The number of bits in the buffer
    let mut bits_in_buffer: usize = 0;
    // Iterate over the input bytes
    for &byte in input {
        // Add the byte to the buffer
        buffer = (buffer << 8) | (byte as u64);
        // Increase the number of bits in the buffer
        bits_in_buffer += 8;
        // While there are enough bits in the buffer to encode a character
        while bits_in_buffer >= BITS_PER_CHAR {
            // Extract the top 5 bits from the buffer
            let index = (buffer >> (bits_in_buffer - BITS_PER_CHAR)) as usize;
            // Append the corresponding character to the output
            output.push(ALPHABET[index]);
            // Remove the top 5 bits from the buffer
            buffer &= (1 << (bits_in_buffer - BITS_PER_CHAR)) - 1;
            // Decrease the number of bits in the buffer
            bits_in_buffer -= BITS_PER_CHAR;
        }
    }
    // If there are remaining bits in the buffer
    if bits_in_buffer > 0 {
        // Pad the buffer with zeros to the right
        buffer <<= BITS_PER_CHAR - bits_in_buffer;
        // Extract the top 5 bits from the buffer
        let index = buffer as usize;
        // Append the corresponding character to the output
        output.push(ALPHABET[index]);
    }
    // Return the output string
    output
}

/// Decodes a string in base32. **Useful for filename**.
#[cfg(test)]
pub fn decode(input: &[u8]) -> Result<Vec<u8>, String> {
    // Create a vector to store the decoded bytes
    let mut output = Vec::with_capacity(input.len());
    // Iterate over the input in chunks of 8 characters
    for chunk in input.chunks(8) {
        // Create a buffer to store the 5-bit values
        let mut buffer = [0u8; 8];
        // Convert each character to a 5-bit value and store it in the buffer
        for (i, &c) in chunk.iter().enumerate() {
            let index = match c {
                50..=55 => c - 24,
                65..=90 => c - 65,
                _ => return Err(format!("Invalid char: {c}")),
            };
            buffer[i] = index;
        }
        // Decode the buffer into 5 bytes using bit shifting and masking
        let b0 = (buffer[0] << 3) | (buffer[1] >> 2);
        let b1 = ((buffer[1] & 0b11) << 6) | (buffer[2] << 1) | (buffer[3] >> 4);
        let b2 = ((buffer[3] & 0b1111) << 4) | (buffer[4] >> 1);
        let b3 = ((buffer[4] & 0b1) << 7) | (buffer[5] << 2) | (buffer[6] >> 3);
        let b4 = ((buffer[6] & 0b111) << 5) | buffer[7];
        // Push the decoded bytes to the output vector
        output.push(b0);
        output.push(b1);
        output.push(b2);
        output.push(b3);
        output.push(b4);
    }
    output.truncate(input.len() * 5 / 8);
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_with_data_should_return_encoded_data() {
        // Given
        let data = "lib1 :: Module";
        // When
        let encoded = encode(data.as_bytes());
        // Then
        let encoded = String::from_utf8(encoded).unwrap();
        assert_eq!(&encoded, "NRUWEMJAHI5CATLPMR2WYZI")
    }

    #[test]
    fn decode_with_encoded_data_should_return_decoded_data() {
        // Given
        let encoded = "NRUWEMJAHI5CATLPMR2WYZI";
        // When
        let decoded = decode(encoded.as_bytes()).unwrap();
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
            let decoded = decode(&encoded).unwrap();
            let decoded = String::from_utf8(decoded).unwrap();
            assert_eq!(&decoded, data)
        }
    }
}
