const LATIN1_TO_UTF8_BYTES: [[u8; 2]; 128] = [
    [0xC2, 0x80],
    [0xC2, 0x81],
    [0xC2, 0x82],
    [0xC2, 0x83],
    [0xC2, 0x84],
    [0xC2, 0x85],
    [0xC2, 0x86],
    [0xC2, 0x87],
    [0xC2, 0x88],
    [0xC2, 0x89],
    [0xC2, 0x8A],
    [0xC2, 0x8B],
    [0xC2, 0x8C],
    [0xC2, 0x8D],
    [0xC2, 0x8E],
    [0xC2, 0x8F],
    [0xC2, 0x90],
    [0xC2, 0x91],
    [0xC2, 0x92],
    [0xC2, 0x93],
    [0xC2, 0x94],
    [0xC2, 0x95],
    [0xC2, 0x96],
    [0xC2, 0x97],
    [0xC2, 0x98],
    [0xC2, 0x99],
    [0xC2, 0x9A],
    [0xC2, 0x9B],
    [0xC2, 0x9C],
    [0xC2, 0x9D],
    [0xC2, 0x9E],
    [0xC2, 0x9F],
    [0xC2, 0xA0],
    [0xC2, 0xA1],
    [0xC2, 0xA2],
    [0xC2, 0xA3],
    [0xC2, 0xA4],
    [0xC2, 0xA5],
    [0xC2, 0xA6],
    [0xC2, 0xA7],
    [0xC2, 0xA8],
    [0xC2, 0xA9],
    [0xC2, 0xAA],
    [0xC2, 0xAB],
    [0xC2, 0xAC],
    [0xC2, 0xAD],
    [0xC2, 0xAE],
    [0xC2, 0xAF],
    [0xC2, 0xB0],
    [0xC2, 0xB1],
    [0xC2, 0xB2],
    [0xC2, 0xB3],
    [0xC2, 0xB4],
    [0xC2, 0xB5],
    [0xC2, 0xB6],
    [0xC2, 0xB7],
    [0xC2, 0xB8],
    [0xC2, 0xB9],
    [0xC2, 0xBA],
    [0xC2, 0xBB],
    [0xC2, 0xBC],
    [0xC2, 0xBD],
    [0xC2, 0xBE],
    [0xC2, 0xBF],
    [0xC3, 0x80],
    [0xC3, 0x81],
    [0xC3, 0x82],
    [0xC3, 0x83],
    [0xC3, 0x84],
    [0xC3, 0x85],
    [0xC3, 0x86],
    [0xC3, 0x87],
    [0xC3, 0x88],
    [0xC3, 0x89],
    [0xC3, 0x8A],
    [0xC3, 0x8B],
    [0xC3, 0x8C],
    [0xC3, 0x8D],
    [0xC3, 0x8E],
    [0xC3, 0x8F],
    [0xC3, 0x90],
    [0xC3, 0x91],
    [0xC3, 0x92],
    [0xC3, 0x93],
    [0xC3, 0x94],
    [0xC3, 0x95],
    [0xC3, 0x96],
    [0xC3, 0x97],
    [0xC3, 0x98],
    [0xC3, 0x99],
    [0xC3, 0x9A],
    [0xC3, 0x9B],
    [0xC3, 0x9C],
    [0xC3, 0x9D],
    [0xC3, 0x9E],
    [0xC3, 0x9F],
    [0xC3, 0xA0],
    [0xC3, 0xA1],
    [0xC3, 0xA2],
    [0xC3, 0xA3],
    [0xC3, 0xA4],
    [0xC3, 0xA5],
    [0xC3, 0xA6],
    [0xC3, 0xA7],
    [0xC3, 0xA8],
    [0xC3, 0xA9],
    [0xC3, 0xAA],
    [0xC3, 0xAB],
    [0xC3, 0xAC],
    [0xC3, 0xAD],
    [0xC3, 0xAE],
    [0xC3, 0xAF],
    [0xC3, 0xB0],
    [0xC3, 0xB1],
    [0xC3, 0xB2],
    [0xC3, 0xB3],
    [0xC3, 0xB4],
    [0xC3, 0xB5],
    [0xC3, 0xB6],
    [0xC3, 0xB7],
    [0xC3, 0xB8],
    [0xC3, 0xB9],
    [0xC3, 0xBA],
    [0xC3, 0xBB],
    [0xC3, 0xBC],
    [0xC3, 0xBD],
    [0xC3, 0xBE],
    [0xC3, 0xBF],
];
pub struct UnicodeUtil {}

impl UnicodeUtil {
    pub fn latin1_to_utf8(latin1_bytes: &[u8]) -> Vec<u8> {
        let mut utf8_bytes = Vec::new();
        for i in latin1_bytes {
            if *i < 128 {
                utf8_bytes.push(*i);
            } else {
                let bytes = &LATIN1_TO_UTF8_BYTES[*i as usize - 128];
                utf8_bytes.push(bytes[0]);
                utf8_bytes.push(bytes[1]);
            }
        }
        utf8_bytes
    }
}