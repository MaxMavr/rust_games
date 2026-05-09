const CRC32_POLYNOMIAL: u32 = 0xEDB88320;
const CRC32_FINAL_XOR: u32 = 0xFFFFFFFF;
pub const CRC32_INITIAL: u32 = 0xFFFFFFFF;

pub fn crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &b in data {
        crc ^= b as u32;

        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

pub fn crc32_finalize(crc: u32) -> u32 {
    crc ^ CRC32_FINAL_XOR
}
