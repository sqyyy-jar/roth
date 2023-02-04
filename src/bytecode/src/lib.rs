pub mod leb128;

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::leb128::{decode, encode};

    #[test]
    fn test_encode() {
        let a = 0xCAFEBABEu64;
        let mut bytes = Vec::with_capacity(9);
        encode(&mut bytes, a).expect("Encode to vec");
        assert_eq!(bytes, [0xBE, 0xF5, 0xFA, 0xD7, 0x0C]);
    }

    #[test]
    fn test_decode() {
        const BYTES: [u8; 5] = [0xBE, 0xF5, 0xFA, 0xD7, 0x0C];
        let a = decode(&mut Cursor::new(&BYTES)).expect("Decode to u64");
        assert_eq!(a, 0xCAFEBABEu64);
    }

    #[test]
    fn test_encode_max() {
        let a = 0xFFFF_FFFF_FFFF_FFFFu64;
        let mut bytes = Vec::with_capacity(9);
        encode(&mut bytes, a).expect("Encode to vec");
        assert_eq!(
            bytes,
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn test_decode_max() {
        const BYTES: [u8; 10] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        let a = decode(&mut Cursor::new(&BYTES)).expect("Decode to u64");
        assert_eq!(a, 0xFFFF_FFFF_FFFF_FFFFu64);
    }
}
