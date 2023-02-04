use std::io::{Error, ErrorKind, Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::leb128::{decode, encode};

pub const SECTION_STRINGS: u8 = 0x01;
pub const SECTION_FUNCTIONS: u8 = 0x02;
pub const SECTION_MODULES: u8 = 0x03;
pub const SECTION_IMPORTS: u8 = 0x04;

pub struct Binary {
    pub sections: Vec<BinarySection>,
}

impl Binary {
    pub fn read(read: &mut impl Read) -> Result<Self> {
        if read.read_u32::<BigEndian>()? != 0x00_72_74_68 {
            return Err(Error::new(ErrorKind::Other, "Invalid byte magic"));
        }
        let sections_len = decode(read)?;
        let mut sections = Vec::with_capacity(sections_len as _);
        for _ in 0..sections_len {
            sections.push(BinarySection::read(read)?);
        }
        Ok(Self { sections })
    }

    pub fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_all(b"\0rth")?;
        encode(write, self.sections.len() as _)?;
        for section in &self.sections {
            section.write(write)?;
        }
        Ok(())
    }
}

pub enum BinarySection {
    Strings { strings: Vec<String> },
    Functions { functions: Vec<BinaryFunction> },
    Modules { modules: Vec<String> },
    Imports { imports: Vec<BinaryImport> },
}

impl BinarySection {
    pub fn read(read: &mut impl Read) -> Result<Self> {
        let tag = read.read_u8()?;
        match tag {
            SECTION_STRINGS => {
                let len = decode(read)?;
                let mut strings = Vec::with_capacity(len as _);
                for _ in 0..len {
                    strings.push(read_str(read)?);
                }
                Ok(Self::Strings { strings })
            }
            SECTION_FUNCTIONS => {
                let len = decode(read)?;
                let mut functions = Vec::with_capacity(len as _);
                for _ in 0..len {
                    functions.push(BinaryFunction::read(read)?);
                }
                Ok(Self::Functions { functions })
            }
            SECTION_MODULES => {
                let len = decode(read)?;
                let mut modules = Vec::with_capacity(len as _);
                for _ in 0..len {
                    modules.push(read_str(read)?);
                }
                Ok(Self::Modules { modules })
            }
            SECTION_IMPORTS => {
                let len = decode(read)?;
                let mut imports = Vec::with_capacity(len as _);
                for _ in 0..len {
                    imports.push(BinaryImport::read(read)?);
                }
                Ok(Self::Imports { imports })
            }
            _ => Err(Error::new(ErrorKind::Other, "Invalid section tag")),
        }
    }

    pub fn write(&self, write: &mut impl Write) -> Result<()> {
        match self {
            BinarySection::Strings { strings } => {
                write.write_u8(SECTION_STRINGS)?;
                encode(write, strings.len() as _)?;
                for string in strings {
                    write_str(write, string)?;
                }
            }
            BinarySection::Functions { functions } => {
                write.write_u8(SECTION_FUNCTIONS)?;
                encode(write, functions.len() as _)?;
                for function in functions {
                    function.write(write)?;
                }
            }
            BinarySection::Modules { modules } => {
                write.write_u8(SECTION_MODULES)?;
                encode(write, modules.len() as _)?;
                for module in modules {
                    write_str(write, module)?;
                }
            }
            BinarySection::Imports { imports } => {
                write.write_u8(SECTION_IMPORTS)?;
                encode(write, imports.len() as _)?;
                for module in imports {
                    module.write(write)?;
                }
            }
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BinaryType {
    Int = 0x11,
    Float = 0x12,
    String = 0x13,
}

impl BinaryType {
    pub fn read(read: &mut impl Read) -> Result<Self> {
        let tag = read.read_u8()?;
        match tag {
            0x11 => Ok(Self::Int),
            0x12 => Ok(Self::Float),
            0x13 => Ok(Self::String),
            _ => Err(Error::new(ErrorKind::Other, "Invalid type tag")),
        }
    }

    pub fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_u8(*self as u8)
    }
}

pub struct BinaryFunction {
    pub name: String,
    pub params: Vec<BinaryType>,
    pub returns: Vec<BinaryType>,
    pub code: Vec<u8>,
}

impl BinaryFunction {
    pub fn read(read: &mut impl Read) -> Result<Self> {
        let name = read_str(read)?;
        let len = decode(read)?;
        let mut params = Vec::with_capacity(len as _);
        for _ in 0..len {
            params.push(BinaryType::read(read)?);
        }
        let len = decode(read)?;
        let mut returns = Vec::with_capacity(len as _);
        for _ in 0..len {
            returns.push(BinaryType::read(read)?);
        }
        let len = decode(read)?;
        let mut code = Vec::with_capacity(len as _);
        let read_len = read.take(len).read_to_end(&mut code)?;
        if len != read_len as u64 {
            return Err(Error::new(ErrorKind::Other, "Invalid code length"));
        }
        Ok(Self {
            name,
            params,
            returns,
            code,
        })
    }

    pub fn write(&self, write: &mut impl Write) -> Result<()> {
        write_str(write, &self.name)?;
        encode(write, self.params.len() as _)?;
        for param in &self.params {
            param.write(write)?;
        }
        encode(write, self.returns.len() as _)?;
        for return_type in &self.returns {
            return_type.write(write)?;
        }
        encode(write, self.code.len() as _)?;
        write.write_all(&self.code)
    }
}

pub struct BinaryImport {
    pub module: usize,
    pub name: String,
}

impl BinaryImport {
    pub fn read(read: &mut impl Read) -> Result<Self> {
        let module = decode(read)? as _;
        let name = read_str(read)?;
        Ok(Self { module, name })
    }

    pub fn write(&self, write: &mut impl Write) -> Result<()> {
        encode(write, self.module as _)?;
        write_str(write, &self.name)
    }
}

pub fn read_str(read: &mut impl Read) -> Result<String> {
    let len = decode(read)?;
    let mut buf = String::with_capacity(len as _);
    let read_len = read.take(len).read_to_string(&mut buf)?;
    if len != read_len as u64 {
        return Err(Error::new(ErrorKind::Other, "Invalid string length"));
    }
    Ok(buf)
}

pub fn write_str(write: &mut impl Write, value: &str) -> Result<()> {
    encode(write, value.len() as _)?;
    write.write_all(value.as_bytes())
}
