



/// this is most valid architecture i found on the internet but we will not use all of this in our vm

/// RISC-V RV32I Register File (Named ABI)
#[derive(Debug)]
pub struct Register {
    pub zero: u32, // x0 — Always 0. Writing has no effect. Used for comparisons, offsets, or null references.
    
    pub ra: u32,   // x1 — Return Address. Holds the address to return to after a function call (link register).
    pub sp: u32,   // x2 — Stack Pointer. Points to the top of the current stack frame in memory.
    pub gp: u32,   // x3 — Global Pointer. Points to the read-only or global data segment.
    pub tp: u32,   // x4 — Thread Pointer. Points to thread-local storage (TLS) for multithreading.
    
    pub t0: u32,   // x5 — Temporary register. Caller-saved, can be used freely in functions. Not preserved across calls.
    pub t1: u32,   // x6 — Temporary register.
    pub t2: u32,   // x7 — Temporary register.
    
    pub s0: u32,   // x8 — Saved register / Frame Pointer. Callee-saved. Often used as the frame pointer (fp).
    pub s1: u32,   // x9 — Saved register. Callee-saved, preserved across function calls.
    
    pub a0: u32,   // x10 — Argument register 0 / Return value 0. Used to pass first function argument or hold return value.
    pub a1: u32,   // x11 — Argument register 1 / Return value 1.
    pub a2: u32,   // x12 — Argument register 2.
    pub a3: u32,   // x13 — Argument register 3.
    pub a4: u32,   // x14 — Argument register 4.
    pub a5: u32,   // x15 — Argument register 5.
    pub a6: u32,   // x16 — Argument register 6.
    pub a7: u32,   // x17 — Argument register 7. Also used to hold syscall number.
    
    pub s2: u32,   // x18 — Saved register. 
    pub s3: u32,   // x19 — Saved register. 
    pub s4: u32,   // x20 — Saved register. 
    pub s5: u32,   // x21 — Saved register. 
    pub s6: u32,   // x22 — Saved register. 
    pub s7: u32,   // x23 — Saved register. 
    pub s8: u32,   // x24 — Saved register. 
    pub s9: u32,   // x25 — Saved register. 
    pub s10: u32,  // x26 — Saved register. 
    pub s11: u32,  // x27 — Saved register. 
    
    pub t3: u32,   // x28 — Temporary register. Caller-saved, volatile across calls.
    pub t4: u32,   // x29 — Temporary register.
    pub t5: u32,   // x30 — Temporary register.
    pub t6: u32,   // x31 — Temporary regist

}

impl Register {
    pub fn read_x(&self, idx: usize) -> u32 {
        match idx {
            0 => 0,
            1 => self.ra,
            2 => self.sp,
            3 => self.gp,
            4 => self.tp,
            5 => self.t0,
            6 => self.t1,
            7 => self.t2,
            8 => self.s0,
            9 => self.s1,
            10 => self.a0,
            11 => self.a1,
            12 => self.a2,
            13 => self.a3,
            14 => self.a4,
            15 => self.a5,
            16 => self.a6,
            17 => self.a7,
            18 => self.s2,
            19 => self.s3,
            20 => self.s4,
            21 => self.s5,
            22 => self.s6,
            23 => self.s7,
            24 => self.s8,
            25 => self.s9,
            26 => self.s10,
            27 => self.s11,
            28 => self.t3,
            29 => self.t4,
            30 => self.t5,
            31 => self.t6,
            _ => panic!("invalid x register"),
        }
    }
}


impl Register {
    pub fn write_x(&mut self, idx: usize, val: u32) {
        if idx == 0 {
            return; // x0 always zero
        }

        match idx {
            1 => self.ra = val,
            2 => self.sp = val,
            3 => self.gp = val,
            4 => self.tp = val,
            5 => self.t0 = val,
            6 => self.t1 = val,
            7 => self.t2 = val,
            8 => self.s0 = val,
            9 => self.s1 = val,
            10 => self.a0 = val,
            11 => self.a1 = val,
            12 => self.a2 = val,
            13 => self.a3 = val,
            14 => self.a4 = val,
            15 => self.a5 = val,
            16 => self.a6 = val,
            17 => self.a7 = val,
            18 => self.s2 = val,
            19 => self.s3 = val,
            20 => self.s4 = val,
            21 => self.s5 = val,
            22 => self.s6 = val,
            23 => self.s7 = val,
            24 => self.s8 = val,
            25 => self.s9 = val,
            26 => self.s10 = val,
            27 => self.s11 = val,
            28 => self.t3 = val,
            29 => self.t4 = val,
            30 => self.t5 = val,
            31 => self.t6 = val,
            _ => panic!("invalid x register"),
        }
    }
}


