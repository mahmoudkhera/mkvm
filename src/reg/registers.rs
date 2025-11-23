



#[derive(Debug, Default, Clone, Copy)]
pub struct CpuRegisters {
    // General-purpose registers r0-r12
    pub r: [u32; 13], // r[0] = r0, r[1] = r1, ..., r[12] = r12

    // Stack pointers
    pub sp_main: u32,    // Main stack pointer
    pub sp_process: u32, // Process stack pointer

    // Link register
    pub lr: u32, // r14

    // Program counter
    pub pc: u32, // r15

    // Program status register (xPSR)
    pub xpsr: u32,
}

