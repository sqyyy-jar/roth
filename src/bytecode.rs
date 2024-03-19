#![allow(unused)]

pub const TYPE_I64: u8 = 0x1;
pub const TYPE_F64: u8 = 0x2;
pub const TYPE_STR: u8 = 0x3;

const FLAG_I64: u16 = (TYPE_I64 as u16) << 8;
const FLAG_F64: u16 = (TYPE_F64 as u16) << 8;
const FLAG_STR: u16 = (TYPE_STR as u16) << 8;

// Memory operations
/// Drop top of the stack
pub const INSN_DROP: u16 = 0x0000;
/// Load constant
pub const INSN_LOAD: u16 = 0x0001;
/// Swap top two element on the stack
pub const INSN_SWAP: u16 = 0x0002;
/// Duplicate top element on the stack
pub const INSN_DUP: u16 = 0x0003;
/// Jump to address on top of the stack
pub const INSN_J: u16 = 0x0004;
/// Jump to address on top of the stack if the second element on the stack is non-zero
pub const INSN_JNZ: u16 = 0x0005;
/// Jump to address on top of the stack if the second element on the stack is zero
pub const INSN_JZ: u16 = 0x0006;
pub const INSN_TROT: u16 = 0x0007;
pub const INSN_DDUP: u16 = 0x000A;
pub const INSN_TDUP: u16 = 0x000B;

const INSN_PUSH: u16 = 0x0008;
const INSN_NUMCONV: u16 = 0x0009;

/// Push i64 onto stack
pub const INSN_PUSH_I64: u16 = INSN_PUSH | FLAG_I64;
/// Push f64 onto stack
pub const INSN_PUSH_F64: u16 = INSN_PUSH | FLAG_F64;
/// Convert f64 to i64
pub const INSN_NUMCONV_I64: u16 = INSN_NUMCONV | FLAG_I64;
/// Convert i64 to f64
pub const INSN_NUMCONV_F64: u16 = INSN_NUMCONV | FLAG_F64;

// System operations
/// Abort virtual machine
pub const INSN_ABORT: u16 = 0x1000;
/// Exit virtual machine
pub const INSN_EXIT: u16 = 0x1001;
/// Panic virtual machine
pub const INSN_PANIC: u16 = 0x1002;
/// Print newline to standard output
pub const INSN_PRINTLN: u16 = 0x1003;
/// Read line from standard input
pub const INSN_INPUT: u16 = 0x1004;
/// Start to remove unused strings
pub const INSN_GC: u16 = 0x1005;

const INSN_PRINT: u16 = 0x1008;

/// Print integer to standard output
pub const INSN_PRINT_I64: u16 = INSN_PRINT | FLAG_I64;
/// Print float to standard output
pub const INSN_PRINT_F64: u16 = INSN_PRINT | FLAG_F64;
/// Print string to standard output
pub const INSN_PRINT_STR: u16 = INSN_PRINT | FLAG_STR;

// Arithmetic operations
const INSN_ADD: u16 = 0x2000;
const INSN_SUB: u16 = 0x2001;
const INSN_MUL: u16 = 0x2002;
const INSN_DIV: u16 = 0x2003;

pub const INSN_ADD_I64: u16 = INSN_ADD | FLAG_I64;
pub const INSN_SUB_I64: u16 = INSN_SUB | FLAG_I64;
pub const INSN_MUL_I64: u16 = INSN_MUL | FLAG_I64;
pub const INSN_DIV_I64: u16 = INSN_DIV | FLAG_I64;

pub const INSN_ADD_F64: u16 = INSN_ADD | FLAG_F64;
pub const INSN_SUB_F64: u16 = INSN_SUB | FLAG_F64;
pub const INSN_MUL_F64: u16 = INSN_MUL | FLAG_F64;
pub const INSN_DIV_F64: u16 = INSN_DIV | FLAG_F64;

pub const INSN_ADD_STR: u16 = INSN_ADD | FLAG_STR;

const INSN_EQ: u16 = 0x3000;
const INSN_LT: u16 = 0x3001;
const INSN_GT: u16 = 0x3002;
const INSN_LE: u16 = 0x3003;
const INSN_GE: u16 = 0x3004;

pub const INSN_EQ_I64: u16 = INSN_EQ | FLAG_I64;
pub const INSN_LT_I64: u16 = INSN_LT | FLAG_I64;
pub const INSN_GT_I64: u16 = INSN_GT | FLAG_I64;
pub const INSN_LE_I64: u16 = INSN_LE | FLAG_I64;
pub const INSN_GE_I64: u16 = INSN_GE | FLAG_I64;

pub const INSN_EQ_F64: u16 = INSN_EQ | FLAG_F64;
pub const INSN_LT_F64: u16 = INSN_LT | FLAG_F64;
pub const INSN_GT_F64: u16 = INSN_GT | FLAG_F64;
pub const INSN_LE_F64: u16 = INSN_LE | FLAG_F64;
pub const INSN_GE_F64: u16 = INSN_GE | FLAG_F64;

pub const INSN_EQ_STR: u16 = INSN_EQ | FLAG_STR;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    CodeAddress,
}

impl Type {
    pub fn is_int(&self) -> bool {
        *self == Type::Int
    }

    pub fn is_float(&self) -> bool {
        *self == Type::Float
    }

    pub fn is_string(&self) -> bool {
        *self == Type::String
    }
}
