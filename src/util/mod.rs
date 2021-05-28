pub mod num;

pub fn bit_u32(val: u32, index: usize) -> bool {
    val & (1 << index) != 0
}

pub fn set_bit_u32(val: u32, index: usize, bit: bool) -> u32 {
    match bit {
        true => val | (1 << index),
        false => val & !(1 << index),
    }
}

pub fn ror(val: u32, shift_val: usize) -> u32 {
    // rotate right shift
    let shift_val = shift_val % 32usize;
    let v_0 = val >> shift_val;
    let v_1 = val << (32 - shift_val);
    v_0 | v_1
}

pub fn lsl(val: u32, shift_val: usize) -> u32 {
    // logical left shift
    val << shift_val
}

pub fn lsr(val: u32, shift_val: usize) -> u32 {
    // logical right shift
    val >> shift_val
}

pub fn asr(val: u32, shift_val: usize) -> u32 {
    // arithmetic right shift
    let mut res = val;
    let msb = val & 0b1000_0000_0000_0000_0000_0000_0000_0000; // msb: most significant bit
    for _ in 0..shift_val {
        res = (res >> 1) | msb
    }
    res
}

pub fn is_add_carry(val: u64) -> bool {
    val > 0b1111_1111_1111_1111_1111_1111_1111_1111 as u64
}

pub fn is_sub_carry(val: u64) -> bool {
    val < 0b0001_0000_0000_0000_0000_0000_0000_0000_0000 as u64
}

pub fn is_add_overflow(lhs: u32, rhs: u32, res: u32) -> bool {
    let v = !(lhs ^ rhs) & (lhs ^ res) & 0x8000_0000;
    v != 0
}

pub fn is_sub_overflow(lhs: u32, rhs: u32, res: u32) -> bool {
    let v = (lhs ^ rhs) & (lhs ^ res) & 0x8000_0000;
    v > 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bit_u32() {
        let val = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        assert_eq!(super::bit_u32(val, 18), true);
        assert_eq!(super::bit_u32(val, 19), false);
    }
    #[test]
    fn test_set_bit_u32() {
        let val = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        let res = 0b0000_1000_0000_0100_0000_0000_0000_0000;
        assert_eq!(super::set_bit_u32(val, 27, true), res);
    }
    #[test]
    fn test_ror() {
        let val: u32 = 0b0000_0000_0000_0100_0000_0000_0000_1011;
        let want_1: u32 = 0b1100_0000_0000_0001_0000_0000_0000_0010;
        let want_2: u32 = 0b1000_0000_0000_0010_0000_0000_0000_0101;
        assert_eq!(super::ror(val, 2), want_1);
        assert_eq!(super::ror(val, 1), want_2);
    }
    #[test]
    fn test_lsl() {
        let val: u32 = 0b0010_0000_0000_0100_0000_0000_0000_1011;
        let want: u32 = 0b0000_0000_0010_0000_0000_0000_0101_1000;
        assert_eq!(super::lsl(val, 3usize), want);
    }
    #[test]
    fn test_lsr() {
        let val: u32 = 0b0000_0000_0000_0100_0000_0000_0000_1011;
        let want: u32 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        assert_eq!(super::lsr(val, 4usize), want);
    }
    #[test]
    fn test_asr_1() {
        let val: u32 = 0b1110_0000_0000_0100_0000_0000_0000_1011;
        let want: u32 = 0b1111_1000_0000_0001_0000_0000_0000_0010;
        assert_eq!(super::asr(val, 2), want);
    }
    #[test]
    fn test_asr_2() {
        let val: u32 = 0b0110_0000_0000_0100_0000_0000_0000_1011;
        let want: u32 = 0b0000_0000_0110_0000_0000_0100_0000_0000;
        assert_eq!(super::asr(val, 8), want);

    }
}
