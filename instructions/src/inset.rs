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
    pub fn encode(self, inst: Instruction) -> Instruction {
       let inst=match self {
            InstructionClasses::DataProcessing => set_bits!(inst, 0b00, 27, 26),

            InstructionClasses::LoadStore => set_bits!(inst, 0b010, 27, 26, 25),
            InstructionClasses::LoadStoreM => set_bits!(inst, 0b0110, 27, 26, 25, 4),
            InstructionClasses::Media => set_bits!(inst, 0b0111, 27, 26, 26, 4),
            InstructionClasses::Branch => set_bits!(inst, 0b10, 27, 26),
            InstructionClasses::SupervisorCall => set_bits!(inst, 0b11, 27, 26),
        };
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
    pub fn encode(self, inst: Instruction) -> Instruction {
        let inst= match self {
            DataProcessing::DataProcessingRegister => set_bits!(inst, 0b00, 25, 4),
            DataProcessing::DataProcessingRegisterShifted => set_bits!(inst, 0b01, 25, 4),
            DataProcessing::MiscInstructions => set_bits!(inst, 0b01000, 25, 24, 23, 20, 7),
            DataProcessing::HalfwordMultiply => set_bits!(inst, 0b010010, 25, 24, 23, 7, 4),
            DataProcessing::MultiplyAccumulate => set_bits!(inst, 0b001001, 25, 24, 7, 6, 5, 4),
            DataProcessing::SyncPrimitives => set_bits!(inst, 0b0101001, 25, 24, 7, 6, 5, 4),
            DataProcessing::ExtraLoadStore => set_bits!(inst, 0b01011, 25, 7, 6, 5, 4),
            DataProcessing::ExtraLoadStoreUnpriv => {
                set_bits!(inst, 0b0011011, 25, 24, 21, 7, 6, 5, 4)
            }
            DataProcessing::DataProcessingImmediate => set_bits!(inst, 0b1, 25),
            DataProcessing::MovImmediate16 => set_bits!(inst, 0b110000, 25, 24, 23, 22, 21, 20),
            DataProcessing::MovTImmediate16 => set_bits!(inst, 0b110100, 25, 24, 23, 22, 21, 20),
            DataProcessing::MSRImmediateHints => set_bits!(inst, 0b11010, 25, 24, 23, 21, 20),
        };
        inst
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

#[derive(Debug, Clone)]

/// Data-processing (register)
pub enum InstructionDPReg {
    And, // bitwise AND
    Eor, // bitwise XOR
    Sub, // subtract
    Rsb, // reverse subtract (op2 - op1)
    Add, // add
    Adc, // add with carry
    Sbc, // subtract with carry
    Rsc, // reverse subtract with carry
    Tst, // AND and set flags only
    Teq, // XOR and set flags only
    Cmp, // subtract and set flags only
    Cmn, // add and set flags only
    Orr, // bitwise OR
    Mov, // move value to register
    Bic, // bit clear (AND with NOT)
    Mvn, // bitwise NOT (move NOT)
}

impl InstructionDPReg {}

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
