use std::io::Write;

// Arithmetic: 0x0***
pub const INSN_ADD_I64: u16 = 0x0000;
pub const INSN_SUB_I64: u16 = 0x0001;
pub const INSN_MUL_I64: u16 = 0x0002;
pub const INSN_DIV_I64: u16 = 0x0003;

pub const INSN_ADD_F64: u16 = 0x0100;
pub const INSN_SUB_F64: u16 = 0x0101;
pub const INSN_MUL_F64: u16 = 0x0102;
pub const INSN_DIV_F64: u16 = 0x0103;

pub const INSN_EQ_I64: u16 = 0x0200;
pub const INSN_NE_I64: u16 = 0x0201;
pub const INSN_LT_I64: u16 = 0x0202;
pub const INSN_GT_I64: u16 = 0x0203;
pub const INSN_LE_I64: u16 = 0x0204;
pub const INSN_GE_I64: u16 = 0x0205;

pub const INSN_EQ_F64: u16 = 0x0300;
pub const INSN_NE_F64: u16 = 0x0301;
pub const INSN_LT_F64: u16 = 0x0302;
pub const INSN_GT_F64: u16 = 0x0303;
pub const INSN_LE_F64: u16 = 0x0304;
pub const INSN_GE_F64: u16 = 0x0305;

pub const INSN_NUMCONV_I64_F64: u16 = 0x0400;
pub const INSN_NUMCONV_F64_I64: u16 = 0x0401;

// Memory: 0x4***
pub const INSN_DROP1: u16 = 0x4001;
pub const INSN_DROP2: u16 = 0x4002;

pub const INSN_SWAP0: u16 = 0x4100;
pub const INSN_SWAP1: u16 = 0x4101;
pub const INSN_SWAP2: u16 = 0x4102;
pub const INSN_SWAP3: u16 = 0x4103;

pub const INSN_COPY0: u16 = 0x4200;
pub const INSN_COPY1: u16 = 0x4201;
pub const INSN_COPY2: u16 = 0x4202;
pub const INSN_COPY3: u16 = 0x4203;
pub const INSN_COPY4: u16 = 0x4204;
pub const INSN_COPY5: u16 = 0x4205;
pub const INSN_COPY6: u16 = 0x4206;
pub const INSN_COPY7: u16 = 0x4207;

pub const INSN_COPY_BYTE: u16 = 0x4300;

pub const INSN_ROTATE: u16 = 0x4400;

pub const INSN_PUSH_I64_U8: u16 = 0x4500;

pub const INSN_PUSH_I64_I8: u16 = 0x4600;

pub const INSN_PUSH_I64_ZERO: u16 = 0x04700;
pub const INSN_PUSH_I64_ONE: u16 = 0x04701;
pub const INSN_PUSH_I64_TWO: u16 = 0x04702;
pub const INSN_PUSH_I64_U16: u16 = 0x4703;
pub const INSN_PUSH_I64_I16: u16 = 0x4704;
pub const INSN_PUSH_I64_U32: u16 = 0x4705;
pub const INSN_PUSH_I64_I32: u16 = 0x4706;
pub const INSN_PUSH_I64_I64: u16 = 0x4707;

pub const INSN_PUSH_F64_ZERO: u16 = 0x4800;
pub const INSN_PUSH_F64_ONE: u16 = 0x4801;
pub const INSN_PUSH_F64_TWO: u16 = 0x4802;
pub const INSN_PUSH_F64_F64: u16 = 0x48FF;

pub const INSN_LOAD_CONSTANT_U8: u16 = 0x4900;

pub const INSN_LOAD_CONSTANT_U16: u16 = 0x4A00;
pub const INSN_LOAD_CONSTANT_U32: u16 = 0x4A01;

// Branching: 0x8***
pub const INSN_BRANCH_U8: u16 = 0x8000;

pub const INSN_BRANCH_I8: u16 = 0x8100;

pub const INSN_BRANCH_U16: u16 = 0x8200;
pub const INSN_BRANCH_I16: u16 = 0x8201;
pub const INSN_BRANCH_I32: u16 = 0x8202;

pub const INSN_BRANCH_IF_U8: u16 = 0x8300;

pub const INSN_BRANCH_IF_I8: u16 = 0x8400;

pub const INSN_BRANCH_IF_U16: u16 = 0x8500;
pub const INSN_BRANCH_IF_I16: u16 = 0x8501;
pub const INSN_BRANCH_IF_I32: u16 = 0x8502;

pub const INSN_BRANCH_IF_ZERO_U8: u16 = 0x8600;

pub const INSN_BRANCH_IF_ZERO_I8: u16 = 0x8700;

pub const INSN_BRANCH_IF_ZERO_U16: u16 = 0x8800;
pub const INSN_BRANCH_IF_ZERO_I16: u16 = 0x8801;
pub const INSN_BRANCH_IF_ZERO_I32: u16 = 0x8802;

pub const INSN_CALL_U8: u16 = 0x8900;

pub const INSN_CALL_U16: u16 = 0x8A00;
pub const INSN_CALL_U32: u16 = 0x8A01;

pub struct Assembler<'a, T: Write> {
    pub(crate) write: &'a mut T,
}

pub mod arithmetic {
    use std::io::{Result, Write};

    use byteorder::{LittleEndian, WriteBytesExt};

    use super::*;

    /// Integer arithmetic
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn add_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_ADD_I64)
        }

        pub fn sub_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SUB_I64)
        }

        pub fn mul_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_MUL_I64)
        }

        pub fn div_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_DIV_I64)
        }
    }

    /// Float arithmetic
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn add_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_ADD_F64)
        }

        pub fn sub_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SUB_F64)
        }

        pub fn mul_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_MUL_F64)
        }

        pub fn div_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_DIV_F64)
        }
    }

    /// Integer comparision
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn eq_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_EQ_I64)
        }

        pub fn ne_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_NE_I64)
        }

        pub fn lt_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_LT_I64)
        }

        pub fn gt_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_GT_I64)
        }

        pub fn le_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_LE_I64)
        }

        pub fn ge_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_GE_I64)
        }
    }

    /// Float comparision
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn eq_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_EQ_F64)
        }

        pub fn ne_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_NE_F64)
        }

        pub fn lt_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_LT_F64)
        }

        pub fn gt_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_GT_F64)
        }

        pub fn le_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_LE_F64)
        }

        pub fn ge_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_GE_F64)
        }
    }

    /// Number conversion
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn numconv_i64_f64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_NUMCONV_I64_F64)
        }

        pub fn numconv_f64_i64(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_NUMCONV_F64_I64)
        }
    }
}

pub mod memory {
    use std::io::{Result, Write};

    use byteorder::{LittleEndian, WriteBytesExt};

    use super::*;

    /// Dropping
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn drop1(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_DROP1)
        }

        pub fn drop2(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_DROP2)
        }
    }

    /// Swapping
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn swap0(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SWAP0)
        }

        pub fn swap1(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SWAP1)
        }

        pub fn swap2(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SWAP2)
        }

        pub fn swap3(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_SWAP3)
        }
    }

    /// Copying
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn copy(&mut self, offset: u8) -> Result<()> {
            match offset {
                0 => self.write.write_u16::<LittleEndian>(INSN_COPY0),
                1 => self.write.write_u16::<LittleEndian>(INSN_COPY1),
                2 => self.write.write_u16::<LittleEndian>(INSN_COPY2),
                3 => self.write.write_u16::<LittleEndian>(INSN_COPY3),
                4 => self.write.write_u16::<LittleEndian>(INSN_COPY4),
                5 => self.write.write_u16::<LittleEndian>(INSN_COPY5),
                6 => self.write.write_u16::<LittleEndian>(INSN_COPY6),
                7 => self.write.write_u16::<LittleEndian>(INSN_COPY7),
                _ => self
                    .write
                    .write_u16::<LittleEndian>(INSN_COPY_BYTE | offset as u16),
            }
        }
    }

    /// Rotating
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn rotate(&mut self) -> Result<()> {
            self.write.write_u16::<LittleEndian>(INSN_ROTATE)
        }
    }

    /// Integer pushing
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn push_i64(&mut self, value: i64) -> Result<()> {
            match value {
                0 => self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_ZERO),
                1 => self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_ONE),
                2 => self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_TWO),
                3..=255 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_PUSH_I64_U8 | (value as u16 & 0xFF)),
                -128..=-1 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_PUSH_I64_I8 | (value as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_U16)?;
                    self.write.write_u16::<LittleEndian>(value as u16)
                }
                -0x8000..=-0x0081 => {
                    self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_I16)?;
                    self.write.write_i16::<LittleEndian>(value as i16)
                }
                0x0001_0000..=0xFFFF_FFFF => {
                    self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_U32)?;
                    self.write.write_u32::<LittleEndian>(value as u32)
                }
                -0x8000_0000..=-0x8001 => {
                    self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_I32)?;
                    self.write.write_i32::<LittleEndian>(value as i32)
                }
                _ => {
                    self.write.write_u16::<LittleEndian>(INSN_PUSH_I64_I64)?;
                    self.write.write_i64::<LittleEndian>(value)
                }
            }
        }
    }

    /// Float pushing
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn push_f64(&mut self, value: f64) -> Result<()> {
            if value == 0. {
                self.write.write_u16::<LittleEndian>(INSN_PUSH_F64_ZERO)
            } else if value == 1. {
                self.write.write_u16::<LittleEndian>(INSN_PUSH_F64_ONE)
            } else if value == 2. {
                self.write.write_u16::<LittleEndian>(INSN_PUSH_F64_TWO)
            } else {
                self.write.write_u16::<LittleEndian>(INSN_PUSH_F64_F64)?;
                self.write.write_f64::<LittleEndian>(value)
            }
        }
    }

    /// Constant loading
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn load_constant(&mut self, index: u32) -> Result<()> {
            match index {
                0..=255 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_LOAD_CONSTANT_U8 | (index as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write
                        .write_u16::<LittleEndian>(INSN_LOAD_CONSTANT_U16)?;
                    self.write.write_u16::<LittleEndian>(index as u16)
                }
                _ => {
                    self.write
                        .write_u16::<LittleEndian>(INSN_LOAD_CONSTANT_U32)?;
                    self.write.write_u32::<LittleEndian>(index)
                }
            }
        }
    }
}

pub mod branching {
    use std::io::{Result, Write};

    use byteorder::{LittleEndian, WriteBytesExt};

    use super::*;

    /// Direct branching
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn branch(&mut self, offset: i32) -> Result<()> {
            match offset {
                0..=0xFF => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_U8 | (offset as u16 & 0xFF)),
                -0x80..=-1 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_I8 | (offset as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_U16)?;
                    self.write.write_u16::<LittleEndian>(offset as u16)
                }
                -0x8000..=-0x0081 => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_I16)?;
                    self.write.write_i16::<LittleEndian>(offset as i16)
                }
                _ => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_I32)?;
                    self.write.write_i32::<LittleEndian>(offset)
                }
            }
        }
    }

    /// If branching
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn branch_if(&mut self, offset: i32) -> Result<()> {
            match offset {
                0..=0xFF => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_IF_U8 | (offset as u16 & 0xFF)),
                -0x80..=-1 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_IF_I8 | (offset as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_IF_U16)?;
                    self.write.write_u16::<LittleEndian>(offset as u16)
                }
                -0x8000..=-0x0081 => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_IF_I16)?;
                    self.write.write_i16::<LittleEndian>(offset as i16)
                }
                _ => {
                    self.write.write_u16::<LittleEndian>(INSN_BRANCH_IF_I32)?;
                    self.write.write_i32::<LittleEndian>(offset)
                }
            }
        }
    }

    /// If not branching
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn branch_if_not(&mut self, offset: i32) -> Result<()> {
            match offset {
                0..=0xFF => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_IF_ZERO_U8 | (offset as u16 & 0xFF)),
                -0x80..=-1 => self
                    .write
                    .write_u16::<LittleEndian>(INSN_BRANCH_IF_ZERO_I8 | (offset as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write
                        .write_u16::<LittleEndian>(INSN_BRANCH_IF_ZERO_U16)?;
                    self.write.write_u16::<LittleEndian>(offset as u16)
                }
                -0x8000..=-0x0081 => {
                    self.write
                        .write_u16::<LittleEndian>(INSN_BRANCH_IF_ZERO_I16)?;
                    self.write.write_i16::<LittleEndian>(offset as i16)
                }
                _ => {
                    self.write
                        .write_u16::<LittleEndian>(INSN_BRANCH_IF_ZERO_I32)?;
                    self.write.write_i32::<LittleEndian>(offset)
                }
            }
        }
    }

    /// Calling
    impl<'a, T: Write> Assembler<'a, T> {
        pub fn call(&mut self, index: u32) -> Result<()> {
            match index {
                0..=0xFF => self
                    .write
                    .write_u16::<LittleEndian>(INSN_CALL_U8 | (index as u16 & 0xFF)),
                0x0100..=0xFFFF => {
                    self.write.write_u16::<LittleEndian>(INSN_CALL_U16)?;
                    self.write.write_u16::<LittleEndian>(index as u16)
                }
                _ => {
                    self.write.write_u16::<LittleEndian>(INSN_CALL_U32)?;
                    self.write.write_u32::<LittleEndian>(index)
                }
            }
        }
    }
}
