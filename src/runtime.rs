use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    io::{stdin, stdout, Write},
    mem::size_of,
};

use crate::{bytecode::*, Flags};

const ALIGNMENT: usize = 4096;

#[derive(Clone, Copy)]
pub union Value {
    pub int: i64,
    pub float: f64,
    pub string: *const String,
}

pub struct Runtime<'a> {
    pub bp: *mut Value,
    pub sp: *mut Value,
    pub pc: usize,
    pub code: &'a [u8],
    pub stack_size: usize,
    pub layout: Layout,
    pub constants: Vec<String>,
    pub string_pool: Vec<String>,
    pub string_pool_marks: Vec<u8>,
    pub panic_handler: fn(PanicInfo) -> !,
    pub flags: Flags,
}

//#[allow(unused)]
impl<'a> Runtime<'a> {
    pub fn new(
        code: &'a [u8],
        pc: usize,
        stack_size: usize,
        panic_handler: fn(PanicInfo) -> !,
        constants: Vec<String>,
        flags: Flags,
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
            string_pool_marks: Vec::with_capacity(0),
            code,
            flags,
        }
    }

    fn push(&mut self, value: Value) {
        unsafe {
            *self.sp = value;
            self.sp = self.sp.add(1);
        }
    }

    fn pop(&mut self) -> Value {
        unsafe {
            self.sp = self.sp.sub(1);
            *self.sp
        }
    }

    fn alloc_string(&mut self, value: String) -> Value {
        for i in 0..self.string_pool_marks.len() {
            if self.string_pool_marks[i] == 0 {
                self.string_pool[i] = value;
                return Value {
                    string: &self.string_pool[i],
                };
            }
        }
        self.string_pool_marks.push(1);
        self.string_pool.push(value);
        Value {
            string: self.string_pool.last().unwrap(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.pc >= self.code.len()
    }

    fn fetch_insn(&mut self) -> u16 {
        let insn = unsafe { (self.code.as_ptr().add(self.pc) as *const u16).read_unaligned() };
        self.pc += 2;
        insn
    }

    fn fetch_const(&mut self) -> Value {
        let insn = unsafe { (self.code.as_ptr().add(self.pc) as *const Value).read_unaligned() };
        self.pc += 8;
        insn
    }

    fn collect_garbage(&mut self) {
        unsafe {
            let mut gp = self.bp;
            let spsp = self.string_pool.as_mut_ptr();
            let spep = self.string_pool.as_mut_ptr().add(self.string_pool.len());
            for i in 0..self.string_pool_marks.len() {
                self.string_pool_marks[i] = 0;
            }
            while gp != self.sp {
                let entry = (*gp).string;
                if entry >= spsp && entry < spep {
                    let offset = entry.sub(spsp as _) as usize / size_of::<String>();
                    self.string_pool_marks[offset] = 1;
                }
                gp = gp.add(1);
            }
            for i in 0..self.string_pool_marks.len() {
                if self.string_pool_marks[i] == 0 {
                    self.string_pool[i] = String::with_capacity(0);
                }
            }
        }
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
                    INSN_LOAD => {
                        let i = self.pop().int;
                        if i < 0 {
                            (self.panic_handler)(PanicInfo::InvalidConstant { vm: self, index: i });
                        }
                        let Some(constant) = self.constants.get(i as usize) else {
                            (self.panic_handler)(PanicInfo::InvalidConstant { vm: self, index: i });
                        };
                        self.push(Value { string: constant });
                    }
                    INSN_SWAP => {
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
                    INSN_J => {
                        self.sp = self.sp.sub(1);
                        self.pc = (*self.sp).int as _;
                    }
                    INSN_JNZ => {
                        self.sp = self.sp.sub(2);
                        if (*self.sp).int != 0 {
                            self.pc = (*self.sp.add(1)).int as _;
                        }
                    }
                    INSN_JZ => {
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
                        *self.sp.sub(1) = Value {
                            int: (*self.sp.sub(1)).float as i64,
                        };
                    }
                    INSN_NUMCONV_F64 => {
                        *self.sp.sub(1) = Value {
                            float: (*self.sp.sub(1)).int as f64,
                        };
                    }
                    INSN_ABORT => (self.panic_handler)(PanicInfo::Abort { vm: self }),
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
                        let mut buf = String::with_capacity(self.flags.prealloc);
                        stdin.read_line(&mut buf).expect("Read from stdin");
                        if buf.ends_with('\n') {
                            buf.pop().unwrap();
                        }
                        if buf.ends_with('\r') {
                            buf.pop().unwrap();
                        }
                        *self.sp = self.alloc_string(buf);
                        self.sp = self.sp.add(1);
                    }
                    INSN_GC => self.collect_garbage(),
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
                        self.push(Value { int: y + x });
                    }
                    INSN_SUB_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value { int: y - x });
                    }
                    INSN_MUL_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value { int: y * x });
                    }
                    INSN_DIV_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value { int: y / x });
                    }
                    INSN_ADD_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value { float: y + x });
                    }
                    INSN_SUB_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value { float: y - x });
                    }
                    INSN_MUL_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value { float: y * x });
                    }
                    INSN_DIV_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value { float: y / x });
                    }
                    INSN_ADD_STR => {
                        let x = self.pop().string;
                        let y = self.pop().string;
                        let a = self.alloc_string((*y).clone() + (*x).as_str());
                        self.push(a);
                    }
                    INSN_EQ_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value {
                            int: (x == y) as i64,
                        });
                    }
                    INSN_LT_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value {
                            int: (y < x) as i64,
                        });
                    }
                    INSN_GT_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value {
                            int: (y > x) as i64,
                        });
                    }
                    INSN_LE_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value {
                            int: (y <= x) as i64,
                        });
                    }
                    INSN_GE_I64 => {
                        let x = self.pop().int;
                        let y = self.pop().int;
                        self.push(Value {
                            int: (y >= x) as i64,
                        });
                    }
                    INSN_EQ_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value {
                            int: (y == x) as i64,
                        });
                    }
                    INSN_LT_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value {
                            int: (y < x) as i64,
                        });
                    }
                    INSN_GT_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value {
                            int: (y > x) as i64,
                        });
                    }
                    INSN_LE_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value {
                            int: (y <= x) as i64,
                        });
                    }
                    INSN_GE_F64 => {
                        let x = self.pop().float;
                        let y = self.pop().float;
                        self.push(Value {
                            int: (y >= x) as i64,
                        });
                    }
                    INSN_EQ_STR => {
                        let x = self.pop().string;
                        let y = self.pop().string;
                        self.push(Value {
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

impl Drop for Runtime<'_> {
    fn drop(&mut self) {
        unsafe { dealloc(self.bp as _, self.layout) };
    }
}

pub enum PanicInfo<'a, 'b: 'a> {
    Pop {
        vm: &'a mut Runtime<'b>,
        expected: u32,
        got: u32,
    },
    IllegalInstruction {
        vm: &'a mut Runtime<'b>,
        insn: u16,
    },
    Abort {
        vm: &'a mut Runtime<'b>,
    },
    Exit {
        vm: &'a mut Runtime<'b>,
        code: i64,
    },
    Panic {
        vm: &'a mut Runtime<'b>,
        msg: *const String,
    },
    InvalidConstant {
        vm: &'a mut Runtime<'b>,
        index: i64,
    },
}
