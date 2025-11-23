#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    EQ, // Z set: Equal
    NE, // Z clear: Not equal
    CS, // C set: Unsigned >=
    CC, // C clear: Unsigned <
    MI, // N set: Negative
    PL, // N clear: Positive or zero
    VS, // V set: Overflow
    VC, // V clear: No overflow
    HI, // C set AND Z clear: Unsigned >
    LS, // C clear OR Z set: Unsigned <=
    GE, // N == V: Signed >=
    LT, // N != V: Signed <
    GT, // Z clear AND N == V: Signed >
    LE, // Z set OR N != V: Signed <=
    AL, // Always
}

impl Condition {
    /// Decode from 4-bit value
    pub fn decode(instruction: u32) -> Option<Self> {
        // Extract bits 28-31
        let cond: u8 = ((instruction >> 28) & 0xF) as u8;

        match cond & 0b1111 {
            0b0000 => Some(Condition::EQ),
            0b0001 => Some(Condition::NE),
            0b0010 => Some(Condition::CS),
            0b0011 => Some(Condition::CC),
            0b0100 => Some(Condition::MI),
            0b0101 => Some(Condition::PL),
            0b0110 => Some(Condition::VS),
            0b0111 => Some(Condition::VC),
            0b1000 => Some(Condition::HI),
            0b1001 => Some(Condition::LS),
            0b1010 => Some(Condition::GE),
            0b1011 => Some(Condition::LT),
            0b1100 => Some(Condition::GT),
            0b1101 => Some(Condition::LE),
            0b1110 => Some(Condition::AL),
            _ => None, // 0b1111 is undefined here
        }
    }

    /// Encode into 4-bit value
    pub fn encode(self) -> u8 {
        match self {
            Condition::EQ => 0b0000,
            Condition::NE => 0b0001,
            Condition::CS => 0b0010,
            Condition::CC => 0b0011,
            Condition::MI => 0b0100,
            Condition::PL => 0b0101,
            Condition::VS => 0b0110,
            Condition::VC => 0b0111,
            Condition::HI => 0b1000,
            Condition::LS => 0b1001,
            Condition::GE => 0b1010,
            Condition::LT => 0b1011,
            Condition::GT => 0b1100,
            Condition::LE => 0b1101,
            Condition::AL => 0b1110,
        }
    }
}
