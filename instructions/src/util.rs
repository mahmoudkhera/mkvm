/// Set specific bits in a 32-bit instruction.
/// Usage: set_bits!(inst, value, bit_pos1, bit_pos2, ..., bit_posN)
/// The least significant bit of `value` goes to the first bit position, next LSB to the next position, etc.
///
#[macro_export]
macro_rules! set_bits {
    ($inst:expr, $value:expr, $( $bit:expr ),+ ) => {{
        let mut inst_local = $inst;
        let v = $value;
        let bits = [$($bit),+];
        let len=bits.len()-1;

        for (i, &bit_pos) in bits.iter().enumerate() {
            inst_local &= !(1 << bit_pos);
            inst_local |= ((v >> len-i) & 1) << bit_pos;

        }
        inst_local
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_single_bit() {
        let inst: u32 = 0;
        let inst = set_bits!(inst, 1, 0); // set LSB
        assert_eq!(inst, 0b1);

        let inst = set_bits!(inst, 0, 0); // clear LSB
        assert_eq!(inst, 0b0);
    }

    #[test]
    fn test_multiple_bits() {
        let inst: u32 = 0;
        let inst = set_bits!(inst, 0b101, 2, 1, 0);
        assert_eq!(inst, 0b101);

        let inst = set_bits!(inst, 0b011, 2, 1, 0); // overwrite bits

        println!("{inst:03b}");
        assert_eq!(inst, 0b011);
    }

    #[test]
    fn test_non_consecutive_bits() {
        let inst: u32 = 0;
        let inst = set_bits!(inst, 0b101, 4, 2, 0); // bits 4,2,0
        // 1 << 4 = 0b10000, 0 << 2 = 0b0, 1 << 0 = 0b1 → combined = 0b10001
        assert_eq!(inst, 0b10001);
    }

 
}
