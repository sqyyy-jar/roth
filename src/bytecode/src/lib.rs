pub mod asm;
pub mod format;
pub mod leb128;

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{
        format::Binary,
        leb128::{decode, encode},
    };

    #[test]
    fn leb128_encode() {
        let a = 0xCAFEBABEu64;
        let mut bytes = Vec::with_capacity(9);
        encode(&mut bytes, a).expect("Encode to vec");
        assert_eq!(bytes, [0xBE, 0xF5, 0xFA, 0xD7, 0x0C]);
    }

    #[test]
    fn leb128_decode() {
        const BYTES: [u8; 5] = [0xBE, 0xF5, 0xFA, 0xD7, 0x0C];
        let a = decode(&mut Cursor::new(&BYTES)).expect("Decode to u64");
        assert_eq!(a, 0xCAFEBABEu64);
    }

    #[test]
    fn leb128_encode_max() {
        let a = u64::MAX;
        let mut bytes = Vec::with_capacity(9);
        encode(&mut bytes, a).expect("Encode to vec");
        assert_eq!(
            bytes,
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn leb128_decode_max() {
        const BYTES: [u8; 10] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        let a = decode(&mut Cursor::new(&BYTES)).expect("Decode to u64");
        assert_eq!(a, u64::MAX);
    }

    #[test]
    fn leb128_encode_min() {
        let a = 0u64;
        let mut bytes = Vec::with_capacity(9);
        encode(&mut bytes, a).expect("Encode to vec");
        assert_eq!(bytes, [0x00]);
    }

    #[test]
    fn leb128_decode_min() {
        const BYTES: [u8; 1] = [0x00];
        let a = decode(&mut Cursor::new(&BYTES)).expect("Decode to u64");
        assert_eq!(a, 0u64);
    }

    #[test]
    fn format_encode_empty() {
        let mut bytes = Vec::with_capacity(4);
        let binary = Binary {
            sections: Vec::with_capacity(0),
        };
        binary.write(&mut bytes).expect("Encode binary");
        assert_eq!(bytes, [0x00, b'r', b't', b'h', 0x00])
    }

    #[test]
    fn format_decode_empty() {
        const BYTES: [u8; 5] = [0x00, b'r', b't', b'h', 0x00];
        let binary = Binary::read(&mut Cursor::new(&BYTES)).expect("Decode binary");
        assert_eq!(binary.sections.len(), 0);
    }
}
