pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // 1. Decode the hexadecimal string into bytes
    let raw_tx_bytes = match hex::decode(raw_tx_hex) {
        Ok(bytes) => bytes,
        Err(_e) => return Err("Hex decode error".to_string()),
    };

    // 2. Check for sufficient length (version is 4 bytes)
    if raw_tx_bytes.len() < 4 {
        // Changed error message to match unit_test.rs assertion
        return Err("Transaction data too short".to_string());
    }

    // 3. Extract the first 4 bytes (the version field)
    let version_bytes: [u8; 4] = [
        raw_tx_bytes[0],
        raw_tx_bytes[1],
        raw_tx_bytes[2],
        raw_tx_bytes[3],
    ];

    // 4. Convert from little-endian bytes to u32
    // Bitcoin uses little-endian for multi-byte integers like the version.
    let version = u32::from_le_bytes(version_bytes);

    // 5. Return the extracted version
    Ok(version)
}
