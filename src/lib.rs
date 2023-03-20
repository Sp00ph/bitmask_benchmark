#![feature(portable_simd)]

pub mod elem_16bit;
pub mod elem_8bit;
pub mod elem_32bit;

#[cfg(test)]
mod tests {
    use std::simd::{Mask, ToBitMask};

    #[test]
    fn test_bitmask_8bit_len2() {
        use crate::elem_8bit::*;
        for i in 0..4 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_2::from_bitmask_scalar(i), expected);
            assert_eq!(len_2::from_bitmask_simd(i), expected);

            assert_eq!(len_2::to_bitmask_scalar(expected), i);
            assert_eq!(len_2::to_bitmask_simd(expected), i);
            assert_eq!(len_2::to_bitmask_shift(expected), i);
        }
    }

    #[test]
    fn test_bitmask_8bit_len4() {
        use crate::elem_8bit::*;
        for i in 0..16 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_4::from_bitmask_scalar(i), expected);
            assert_eq!(len_4::from_bitmask_simd(i), expected);

            assert_eq!(len_4::to_bitmask_simd(expected), i);
            assert_eq!(len_4::to_bitmask_mul(expected), i);
        }
    }

    #[test]
    fn test_bitmask_8bit_len8() {
        use crate::elem_8bit::*;
        for i in 0..=u8::MAX {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_8::from_bitmask_simd(i), expected);

            assert_eq!(len_8::to_bitmask_simd(expected), i);
            assert_eq!(len_8::to_bitmask_mul(expected), i);
        }
    }

    #[test]
    fn test_bitmask_8bit_len16() {
        use crate::elem_8bit::*;
        for i in 0..=u16::MAX {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_16::from_bitmask_simd(i), expected);

            assert_eq!(len_16::to_bitmask_simd(expected), i);
            assert_eq!(len_16::to_bitmask_mul(expected), i);
        }
    }

    #[test]
    fn test_bitmask_16bit_len2() {
        use crate::elem_16bit::*;
        for i in 0..4 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_2::from_bitmask_scalar(i), expected);
            assert_eq!(len_2::from_bitmask_simd(i), expected);

            assert_eq!(len_2::to_bitmask_scalar(expected), i);
            assert_eq!(len_2::to_bitmask_simd(expected), i);
            assert_eq!(len_2::to_bitmask_shift(expected), i);
        }
    }

    #[test]
    fn test_bitmask_16bit_len4() {
        use crate::elem_16bit::*;
        for i in 0..16 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_4::from_bitmask_scalar(i), expected);
            assert_eq!(len_4::from_bitmask_simd(i), expected);

            assert_eq!(len_4::to_bitmask_simd(expected), i);
            assert_eq!(len_4::to_bitmask_mul(expected), i);
        }
    }

    #[test]
    fn test_bitmask_16bit_len8() {
        use crate::elem_16bit::*;
        for i in 0..=u8::MAX {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_8::from_bitmask_simd(i), expected);

            assert_eq!(len_8::to_bitmask_simd(expected), i);
            assert_eq!(len_8::to_bitmask_mul(expected), i);
        }
    }

    #[test]
    fn test_bitmask_32bit_len2() {
        use crate::elem_32bit::*;
        for i in 0..4 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_2::from_bitmask_scalar(i), expected);
            assert_eq!(len_2::from_bitmask_simd(i), expected);

            assert_eq!(len_2::to_bitmask_scalar(expected), i);
            assert_eq!(len_2::to_bitmask_simd(expected), i);
            assert_eq!(len_2::to_bitmask_shift(expected), i);
        }
    }

    #[test]
    fn test_bitmask_32bit_len4() {
        use crate::elem_32bit::*;
        for i in 0..16 {
            let expected = Mask::from_bitmask(i);
            assert_eq!(len_4::from_bitmask_scalar(i), expected);
            assert_eq!(len_4::from_bitmask_simd(i), expected);

            assert_eq!(len_4::to_bitmask_simd(expected), i);
            assert_eq!(len_4::to_bitmask_mul(expected), i);
        }
    }
}