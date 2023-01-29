#![allow(unused)]

pub const TYPE_I64: u32 = 0x1;
pub const TYPE_F64: u32 = 0x2;
pub const TYPE_STR: u32 = 0x3;

const FLAG_I64: u16 = (TYPE_I64 as u16) << 8;
const FLAG_F64: u16 = (TYPE_F64 as u16) << 8;
const FLAG_STR: u16 = (TYPE_STR as u16) << 8;

// Memory operations
pub const INSN_POP: u16 = 0x0000;
pub const INSN_LDC: u16 = 0x0001;
pub const INSN_SWP: u16 = 0x0002;
pub const INSN_DUP: u16 = 0x0003;
pub const INSN_JMP: u16 = 0x0004;

const INSN_PUSH: u16 = 0x0008;

pub const INSN_PUSH_I64: u16 = INSN_PUSH | FLAG_I64;
pub const INSN_PUSH_F64: u16 = INSN_PUSH | FLAG_F64;

// System operations
pub const INSN_ABRT: u16 = 0x1000;
pub const INSN_EXIT: u16 = 0x1001;
pub const INSN_PANIC: u16 = 0x1002;
pub const INSN_PRINTLN: u16 = 0x1003;
pub const INSN_INPUT: u16 = 0x1004;

const INSN_PRINT: u16 = 0x1008;

pub const INSN_PRINT_I64: u16 = INSN_PRINT | FLAG_I64;
pub const INSN_PRINT_F64: u16 = INSN_PRINT | FLAG_F64;
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
