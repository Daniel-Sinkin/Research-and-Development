use std::f32::{INFINITY, NEG_INFINITY};

const BIAS32: u32 = 127;
const SIGN32: u32 = 0b1_00000000_00000000000000000000000;
const EXP32: u32 = 0b0_11111111_00000000000000000000000;
const DIG32: u32 = 0b0_00000000_11111111111111111111111;

struct F32 {
    digits: u32,

}

impl F32 {
    fn print_digits(&self) {
        println!("{:032b}", &self.digits);
    }

    fn to_float(&self) -> f32 {
        let is_negative = ((&self.digits & SIGN32) != 0);

        if &self.digits & EXP32 == EXP32 {
            if &self.digits & DIG32 == 0 {
                return f32::NAN;
            } else {
                if is_negative {
                    println!("+Inf");
                    return f32::INFINITY;
                } else {
                    return f32::NEG_INFINITY;
                }
            }
        }

        let exp_biased = (&self.digits & EXP32) >> 22;
        return f32::NAN;
    }
}

fn main() {
    // binary32 -> 24 digits, 8 exp
    let mut x: u32 = 0b0_01111100_01000000000000000000000u32;
    let mut y: u32 = 0b0_11111111_00000000000000000000000u32;
    let mut exp_biased = (x & EXP32) >> 23;

    println!("{:032b}", exp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_float() {
        let x = F32{digits: 0b0_0111110_001000000000000000000000u32}.to_float();
        assert_eq!(x, 0.15625);
    }

    fn negative_float() {
        let x = F32{digits: 0b1_10000010_11101110000000000000000u32}.to_float();
        assert_eq!(x, -15.4375);
    }

    fn NAN() {
        let x= F32{digits: 0b0_11111111_00001000000000000000000u32}.to_float();
        assert_eq!(x, f32::NAN);
    }

    fn INF_POS() {
        let x = F32{digits: 0b0_11111111_00000000000000000000000}.to_float();
        assert_eq!(x, f32::INFINITY);
    }

    fn INF_NEG() {
        let x = F32{digits: 0b1_11111111_00000000000000000000000}.to_float();
        assert_eq!(x, f32::NEG_INFINITY);
    }
}