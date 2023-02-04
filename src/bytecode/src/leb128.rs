use std::io::{Read, Result, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

pub fn encode(write: &mut impl Write, value: u64) -> Result<()> {
    if value == 0 {
        write.write_u8(0x00)?;
        return Ok(());
    }
    let bin_digits = 64 - value.leading_zeros();
    let parts = parts(bin_digits);
    let mut tmp = value;
    for _ in 0..parts - 1 {
        write.write_u8(128u8 | (tmp as u8 & 127))?;
        tmp >>= 7;
    }
    write.write_u8(tmp as u8 & 127)?;
    Ok(())
}

pub fn decode(read: &mut impl Read) -> Result<u64> {
    let mut value = 0;
    let mut offset = 0;
    loop {
        let part = read.read_u8()? as u64;
        value |= (part & 127) << offset;
        offset += 7;
        if part & 128 == 0 {
            break;
        }
    }
    Ok(value)
}

fn parts(bin_digits: u32) -> u32 {
    if bin_digits % 7 == 0 {
        bin_digits / 7
    } else {
        bin_digits / 7 + 1
    }
}
