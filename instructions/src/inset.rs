use crate::set_bits;

type Instruction = u32;

pub enum InstructionClasses {
    DataProcessing,
    LoadStore,
    LoadStoreM,
    Media,
    Branch,
    SupervisorCall,
}

impl InstructionClasses {
    /// Encode the instruction class into the instruction
    pub fn encode(self, mut inst: Instruction) -> Instruction {
        match self {
            InstructionClasses::DataProcessing => {
                inst = set_bits!(inst, 0, 27, 26);
            }
            InstructionClasses::LoadStore => {
                inst = set_bits!(inst, 0, 27, 25);
                inst = set_bits!(inst, 1, 26);
            }
            InstructionClasses::LoadStoreM => {
                inst = set_bits!(inst, 1, 26, 25);
                inst = set_bits!(inst, 0, 27, 4);
            }
            InstructionClasses::Media => {
                inst = set_bits!(inst, 1, 26, 25, 4);
                inst = set_bits!(inst, 0, 27);
            }
            InstructionClasses::Branch => {
                inst = set_bits!(inst, 1, 27);
                inst = set_bits!(inst, 0, 26);
            }
            InstructionClasses::SupervisorCall => {
                inst = set_bits!(inst, 1, 27, 26);
            }
        }
        inst
    }

    /// Decode  instruction into an instruction class
    pub fn decode(inst: Instruction) -> Option<Self> {
        // Extract bits 27-26-25
        let op = ((inst >> 25) & 0b111) as u8;
        let op1 = ((inst >> 4) & 0b1) as u8;

        match op {
            0b001 | 0b000 => Some(InstructionClasses::DataProcessing),
            0b010 => Some(InstructionClasses::LoadStore),
            0b011 if (op1 == 0) => Some(InstructionClasses::LoadStoreM),
            0b011 if (op1 == 1) => Some(InstructionClasses::Media),

            0b110 | 0b111 => Some(InstructionClasses::Media),
            _ => None, // impossible, just for safety
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataProcessing {
    DataProcessingRegister,        // 0 not 10xx0 xxx0
    DataProcessingRegisterShifted, // 0xx1
    MiscInstructions,              // 10xx0 0xxx
    HalfwordMultiply,              // 1xx0
    MultiplyAccumulate,            // 0xxxx 1001
    SyncPrimitives,                // 1xxxx 1001
    ExtraLoadStore,                // not 0xx1x 1011
    ExtraLoadStoreUnpriv,          // 0xx1x 1011
    DataProcessingImmediate,       // 1 not 10xx0
    MovImmediate16,                // 10000-
    MovTImmediate16,               // 10100-
    MSRImmediateHints,             // 10x10-
}

impl DataProcessing {
    pub fn encode(self, inst: Instruction) -> u32 {
        match self {
            DataProcessing::DataProcessingRegister => {
                set_bits!(inst, 0, 25, 4)
            }
            DataProcessing::DataProcessingRegisterShifted => {
                let inst = set_bits!(inst, 0, 25, 27);
                set_bits!(inst, 1, 3)
            }
            DataProcessing::MiscInstructions => {
                // 10xx0 0xxx
                let inst = set_bits!(inst, 0, 25, 23, 20, 7);
                set_bits!(inst, 1, 24)
            }
            DataProcessing::HalfwordMultiply => {
                // 1xx0
                let inst = set_bits!(inst, 0, 25, 23, 20, 4);
                set_bits!(inst, 1, 7)
            }
            DataProcessing::MultiplyAccumulate => {
                let inst = set_bits!(inst, 0, 25, 24, 6, 5);
                set_bits!(inst, 1, 7, 4)
            }
            DataProcessing::SyncPrimitives => {
                let inst = set_bits!(inst, 0, 25, 5, 6);
                set_bits!(inst, 1, 24, 7, 4)
            }
            DataProcessing::ExtraLoadStore => {
                // not 0xx1x 1011
                let inst = set_bits!(inst, 0, 25, 6);
                set_bits!(inst, 1, 7, 5, 4)
            }
            DataProcessing::ExtraLoadStoreUnpriv => {
                let inst: u32 = set_bits!(inst, 0, 25, 24, 6);
                set_bits!(inst, 1, 21, 7, 5, 4)
            }
            DataProcessing::DataProcessingImmediate => {
                set_bits!(inst, 1, 25)
            }
            DataProcessing::MovImmediate16 => {
                let inst: u32 = set_bits!(inst, 0, 23, 22, 21, 20);

                set_bits!(inst, 1, 25, 24)
            }
            DataProcessing::MovTImmediate16 => {
                let inst: u32 = set_bits!(inst, 0, 23, 21, 20);

                set_bits!(inst, 1, 25, 24, 22)
            }
            DataProcessing::MSRImmediateHints => {
                let inst: u32 = set_bits!(inst, 0, 23, 20);

                set_bits!(inst, 1, 25, 21)
            }
        }
    }

    pub fn decode(inst: Instruction) -> Option<Self> {
        let op = ((inst >> 25) & 0b1) as u8; // bit 25
        let op1 = ((inst >> 20) & 0b11111) as u8; // bits 24..20
        let op2 = ((inst >> 4) & 0b1111) as u8; // bits 7..4

        // ----- Patterns -----

        // 0 not 10xx0 xxx0  -> DataProcessingRegister
        if (op & 0b1) == 0 && (op1 & 0b11001) != 0b10000 && (op2 & 0b1) == 0 {
            return Some(Self::DataProcessingRegister);
        }

        //  0 not 10xx0 0xx1 -> DataProcessingRegisterShifted
        if (op & 0b1) == 0 && (op1 & 0b11001) != 0b10000 && (op1 & 0b1001) == 0b0001 {
            return Some(Self::DataProcessingRegisterShifted);
        }

        //  0  10xx0 0xxx -> MiscInstructions
        if op == 0b1 && (op1 & 0b11001) == 0b10000 && (op2 & 0b1000) == 0 {
            return Some(Self::MiscInstructions);
        }

        // 0  10xx0 1xx0 -> HalfwordMultiply
        if (op & 0b1) != 0 && (op1 & 0b11001) == 0b10000 && (op1 & 0b1001) == 0b1000 {
            return Some(Self::HalfwordMultiply);
        }

        //0  0xxxx 1001 -> MultiplyAccumulate
        if op & 0b1 == 0 && (op1 & 0b10000) == 0 && (op2 == 0b1001) {
            return Some(Self::MultiplyAccumulate);
        }

        //0 1xxxx 1001 -> SyncPrimitives
        if op & 0b1 != 0 && (op1 & 0b10000) == 0b10000 && (op2 == 0b1001) {
            return Some(Self::SyncPrimitives);
        }

        // Extra load/store
        //0  not 0xx1x (1011 or 11x1)
        if op & 0b1 == 0
            && (op1 & 0b10010) != 0b00010
            && ((op2 == 0b1011) | (op2 & 0b1101 == 0b1101))
        {
            return Some(Self::ExtraLoadStore);
        }

        // unprivileged
        //0 0xx1x (1011 or 11x1)
        if op & 0b1 == 0
            && (op1 & 0b10010) == 0b00010
            && ((op2 == 0b1011) | (op2 & 0b1101 == 0b1101))
        {
            return Some(Self::ExtraLoadStoreUnpriv);
        }

        // Data-processing (immediate)
        // 1 not 10xx0
        if (op & 0b1) != 0 && !(op1 & 0b11001) == 0b10000 {
            return Some(Self::DataProcessingImmediate);
        }

        // MOV (immediate)
        // 1  10000
        if op == 0b100 && (op1 & 0b11111) == 0b10000 {
            return Some(Self::MovImmediate16);
        }

        // MOVT
        // 1 10100-
        if op == 0b101 && (op1 & 0b111111) == 0b10100 {
            return Some(Self::MovTImmediate16);
        }

        // MSR immediate
        // 1 10x10
        if op == 0b100 && (op1 & 0b11011) == 0b10010 {
            return Some(Self::MSRImmediateHints);
        }

        None
    }
}

pub enum InstructionSet {
    //  DATA PROCESSING INSTRUCTIONS (Register/Immediate)
    //  These operate on registers and update a destination register.

    // Logical AND: Rd = Rn & Operand2
    And,

    // Exclusive OR: Rd = Rn ^ Operand2
    Eor,

    // Subtract: Rd = Rn - Operand2
    Sub,

    // Reverse subtract: Rd = Operand2 - Rn
    Rsb,

    // Add: Rd = Rn + Operand2
    Add,

    // Add with carry: Rd = Rn + Operand2 + C flag
    Adc,

    // Subtract with carry: Rd = Rn - Operand2 - (1 - C)
    Sbc,

    // Reverse subtract with carry: Rd = Operand2 - Rn - (1 - C)
    Rsc,

    // ---- TEST INSTRUCTIONS: don't write to Rd, only set flags ----

    // Bitwise test: sets flags based on Rn & Operand2
    Tst,
    // XOR test: sets flags based on Rn ^ Operand2
    Teq,

    // Compare: sets flags based on Rn - Operand2
    Cmp,

    // Compare negative: sets flags based on Rn + Operand2
    Cmn,

    // ---- MOV / ORR / BIC / MVN ----

    // Bitwise OR: Rd = Rn | Operand2
    Orr,

    // Move: Rd = Operand2 (register or immediate)
    Mov,

    // Bit clear: Rd = Rn & ~Operand2
    Bic,

    // Move NOT: Rd = ~Operand2
    Mvn,

    //  LOAD / STORE INSTRUCTIONS
    //  These move data between registers and memory.

    // Load word (32 bits): Rd = MEM[address]
    Ldr,

    // Store word (32 bits): MEM[address] = Rd
    Str,

    // Load byte (8 bits): Rd = MEM[address]
    Ldrb,

    // Store byte (8 bits): MEM[address] = lower 8 bits of Rd
    Strb,

    //  BRANCH INSTRUCTIONS

    // Branch: PC = PC + offset
    B,

    // Branch with link: LR = PC + 4; PC = PC + offset
    // Used for function calls (BL saves return address)
    Bl,

    // Software interrupt (syscall):
    // In Linux: SWI #imm invokes a system call handler
    Swi,

    // Instruction could not be decoded or is not implemented yet
    Undefined(u32),
}
