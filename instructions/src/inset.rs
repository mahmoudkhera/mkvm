type Instruction = u32;

macro_rules! set_bits {
    ($value:expr, $set_to:expr, $($pos:expr),*) => {{
        let mut val = $value;
        $(
            if $pos < 32 {
                if $set_to==1 {
                    val |= 1 << $pos;
                } else  {
                    val &= !(1 << $pos);
                }
            }
        )*
        val
    }};
}

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
