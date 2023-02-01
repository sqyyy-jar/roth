use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    io::{stdin, stdout, Write},
};

use crate::bytecode::*;

const ALIGNMENT: usize = 4096;

#[derive(Clone, Copy)]
pub union VMValue {
    pub int: i64,
    pub float: f64,
    pub string: *const String,
}

pub struct VirtualMachine<'a> {
    pub bp: *mut VMValue,
    pub sp: *mut VMValue,
    pub pc: usize,
    pub code: &'a [u8],
    pub stack_size: usize,
    pub layout: Layout,
    pub constants: Vec<String>,
    pub string_pool: Vec<String>,
    pub panic_handler: fn(PanicInfo) -> !,
}

//#[allow(unused)]
impl<'a> VirtualMachine<'a> {
    pub fn new(
        code: &'a [u8],
        pc: usize,
        stack_size: usize,
        panic_handler: fn(PanicInfo) -> !,
        constants: Vec<String>,
    ) -> Self {
        let layout = unsafe { Layout::from_size_align_unchecked(stack_size, ALIGNMENT) };
        let bp = unsafe { alloc_zeroed(layout) };
        Self {
            bp: bp as _,
            sp: bp as _,
            stack_size,
            layout,
            panic_handler,
            pc,
            constants,
            string_pool: Vec::with_capacity(0),
            code,
        }
    }

    fn push(&mut self, value: VMValue) {
        unsafe {
            *self.sp = value;
            self.sp = self.sp.add(1);
        }
    }

    fn pop(&mut self) -> VMValue {
        unsafe {
            self.sp = self.sp.sub(1);
            *self.sp
        }
    }

    fn is_at_end(&self) -> bool {
        self.pc >= self.code.len()
    }

    fn fetch_insn(&mut self) -> u16 {
        let insn = unsafe { *(self.code.as_ptr().add(self.pc) as *const u16) };
        self.pc += 2;
        insn
    }

    fn fetch_const(&mut self) -> VMValue {
        let insn = unsafe { *(self.code.as_ptr().add(self.pc) as *const VMValue) };
        self.pc += 8;
        insn
    }

    pub fn execute(&mut self) {
        let mut stdout = stdout();
        let stdin = stdin();
        unsafe {
            while !self.is_at_end() {
                let insn = self.fetch_insn();
                match insn {
                    INSN_DROP => {
                        self.sp = self.sp.sub(1);
                    }
                    INSN_LDC => {
                        let i = self.pop().int;
                        if i < 0 {
                            (self.panic_handler)(PanicInfo::InvalidConstant { vm: self, index: i });
                        }
                        let Some(constant) = self.constants.get(i as usize) else {
                            (self.panic_handler)(PanicInfo::InvalidConstant { vm: self, index: i });
                        };
                        self.push(VMValue { string: constant });
                    }
                    INSN_SWP => {
                        let tmp = *self.sp.sub(2);
                        *self.sp.sub(2) = *self.sp.sub(1);
                        *self.sp.sub(1) = tmp;
                    }
                    INSN_TROT => {
                        let tmp_x = *self.sp.sub(1);
                        let tmp_y = *self.sp.sub(2);
                        *self.sp.sub(1) = *self.sp.sub(3);
                        *self.sp.sub(3) = tmp_y;
                        *self.sp.sub(2) = tmp_x;
                    }
                    INSN_DUP => {
                        *self.sp = *self.sp.sub(1);
                        self.sp = self.sp.add(1);
                    }
                    INSN_DDUP => {
                        *self.sp = *self.sp.sub(2);
                        self.sp = self.sp.add(1);
                    }
                    INSN_TDUP => {
                        *self.sp = *self.sp.sub(3);
                        self.sp = self.sp.add(1);
                    }
                    INSN_JMP => {
                        self.sp = self.sp.sub(1);
                        self.pc = (*self.sp).int as _;
                    }
                    INSN_JMPIF => {
                        self.sp = self.sp.sub(2);
                        if (*self.sp).int != 0 {
                            self.pc = (*self.sp.add(1)).int as _;
                        }
                    }
                    INSN_JMPIFZ => {
                        self.sp = self.sp.sub(2);
                        if (*self.sp).int == 0 {
                            self.pc = (*self.sp.add(1)).int as _;
                        }
                    }
                    INSN_PUSH_I64 | INSN_PUSH_F64 => {
                        *self.sp = self.fetch_const();
                        self.sp = self.sp.add(1);
                    }
                    INSN_NUMCONV_I64 => {
                        *self.sp.sub(1) = VMValue {
                            int: (*self.sp.sub(1)).float as i64,
                        };
                    }
                    INSN_NUMCONV_F64 => {
                        *self.sp.sub(1) = VMValue {
                            float: (*self.sp.sub(1)).int as f64,
                        };
                    }
                    INSN_ABRT => (self.panic_handler)(PanicInfo::Abort { vm: self }),
                    INSN_EXIT => {
                        self.sp = self.sp.sub(1);
                        let code = (*self.sp).int;
                        (self.panic_handler)(PanicInfo::Exit { vm: self, code });
                    }
                    INSN_PANIC => {
                        self.sp = self.sp.sub(1);
                        let msg = (*self.sp).string;
                        (self.panic_handler)(PanicInfo::Panic { vm: self, msg });
                    }
                    INSN_PRINTLN => {
                        stdout.write_all(&[0xA]).expect("Write to stdout");
                        stdout.flush().expect("Write to stdout");
                    }
                    INSN_INPUT => {
                        let mut buf = String::with_capacity(8);
                        stdin.read_line(&mut buf).expect("Read from stdin");
                        if buf.ends_with('\n') {
                            buf.pop().unwrap();
                        }
                        if buf.ends_with('\r') {
                            buf.pop().unwrap();
                        }
                        self.string_pool.push(buf);
                        *self.sp = VMValue {
                            string: self.string_pool.last().unwrap(),
                        };
                        self.sp = self.sp.add(1);
                    }
                    INSN_PRINT_I64 => {
                        self.sp = self.sp.sub(1);
                        stdout
                            .write_all((*self.sp).int.to_string().as_bytes())
                            .expect("Write to stdout");
                        stdout.flush().expect("Write to stdout");
                    }
                    INSN_PRINT_F64 => {
                        self.sp = self.sp.sub(1);
                        stdout
                            .write_all((*self.sp).float.to_string().as_bytes())
                            .expect("Write to stdout");
                        stdout.flush().expect("Write to stdout");
                    }
                    INSN_PRINT_STR => {
                        self.sp = self.sp.sub(1);
                        stdout
                            .write_all((*(*self.sp).string).as_bytes())
                            .expect("Write to stdout");
                        stdout.flush().expect("Write to stdout");
                    }
                    INSN_ADD_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue { int: y + x });
                    }
                    INSN_SUB_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue { int: y - x });
                    }
                    INSN_MUL_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue { int: y * x });
                    }
                    INSN_DIV_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue { int: y / x });
                    }
                    INSN_ADD_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue { float: y + x });
                    }
                    INSN_SUB_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue { float: y - x });
                    }
                    INSN_MUL_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue { float: y * x });
                    }
                    INSN_DIV_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue { float: y / x });
                    }
                    INSN_ADD_STR => {
                        let x = self.pop().string;
                        let y = self.pop().string;
                        self.string_pool.push((*y).clone() + (*x).as_str());
                        self.push(VMValue {
                            string: self.string_pool.last().unwrap(),
                        });
                    }
                    INSN_EQ_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue {
                            int: (x == y) as i64,
                        });
                    }
                    INSN_LT_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue {
                            int: (y < x) as i64,
                        });
                    }
                    INSN_GT_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue {
                            int: (y > x) as i64,
                        });
                    }
                    INSN_LE_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue {
                            int: (y <= x) as i64,
                        });
                    }
                    INSN_GE_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(VMValue {
                            int: (y >= x) as i64,
                        });
                    }
                    INSN_EQ_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue {
                            int: (y == x) as i64,
                        });
                    }
                    INSN_LT_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue {
                            int: (y < x) as i64,
                        });
                    }
                    INSN_GT_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue {
                            int: (y > x) as i64,
                        });
                    }
                    INSN_LE_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue {
                            int: (y <= x) as i64,
                        });
                    }
                    INSN_GE_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(VMValue {
                            int: (y >= x) as i64,
                        });
                    }
                    INSN_EQ_STR => {
                        let x = self.pop().string;
                        let y = self.pop().string;
                        self.push(VMValue {
                            int: (*x == *y) as i64,
                        });
                    }
                    insn => {
                        (self.panic_handler)(PanicInfo::IllegalInstruction { vm: self, insn });
                    }
                }
            }
        }
    }
}

impl Drop for VirtualMachine<'_> {
    fn drop(&mut self) {
        unsafe { dealloc(self.bp as _, self.layout) };
    }
}

pub enum PanicInfo<'a, 'b: 'a> {
    Pop {
        vm: &'a mut VirtualMachine<'b>,
        expected: u32,
        got: u32,
    },
    IllegalInstruction {
        vm: &'a mut VirtualMachine<'b>,
        insn: u16,
    },
    Abort {
        vm: &'a mut VirtualMachine<'b>,
    },
    Exit {
        vm: &'a mut VirtualMachine<'b>,
        code: i64,
    },
    Panic {
        vm: &'a mut VirtualMachine<'b>,
        msg: *const String,
    },
    InvalidConstant {
        vm: &'a mut VirtualMachine<'b>,
        index: i64,
    },
}
