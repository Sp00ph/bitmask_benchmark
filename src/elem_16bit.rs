pub mod len_2 {
    use std::simd::{Mask, Simd, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_scalar(a: Mask<i16, 2>) -> u8 {
        let [a, b] = a.to_array();
        u8::from(a) | (2 * u8::from(b))
    }

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i16, 2>) -> u8 {
        (a.to_int().cast() & Simd::<u16, 2>::from_array([1, 2])).reduce_sum() as u8
    }

    #[inline]
    pub fn to_bitmask_shift(a: Mask<i16, 2>) -> u8 {
        let word: u32 = unsafe { std::mem::transmute(a) };
        ((word >> 15) & 0b11) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i16, 2> {
        let a = a as i16;
        // For some reason this generates shorter code than just using `Mask::from_array`
        unsafe {
            Mask::from_int_unchecked(
                Simd::from_array([-(a & 1), -((a & 2) / 2)]).cast(),
            )
        }
    }

    #[inline]
    pub fn from_bitmask_simd(a: u8) -> Mask<i16, 2> {
        (Simd::splat(a as u16) & Simd::from_array([1, 2])).simd_ne(Simd::splat(0))
    }
}

pub mod len_4 {
    use std::simd::{Mask, Simd, SimdUint, i16x4, u16x4, SimdPartialEq};

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i16, 4>) -> u8 {
        (a.to_int().cast() & Simd::<u16, 4>::from_array([1, 2, 4, 8])).reduce_sum() as u8
    }
    
    #[inline]
    pub fn to_bitmask_mul(a: Mask<i16, 4>) -> u8 {
        let word: u64 = unsafe { std::mem::transmute(a.to_int()) };
        ((word.wrapping_mul(0x8009800c) >> 47) & 0xf) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i16, 4> {
        let a = a as i16;
        unsafe {
            Mask::from_int_unchecked(
                i16x4::from_array([
                    -(a & 1),
                    -((a & 2) / 2),
                    -((a & 4) / 4),
                    -((a & 8) / 8),
                ])
                .cast(),
            )
        }
    }

    #[inline]
    // I suspect this will the the faster of the 2 implementations
    pub fn from_bitmask_simd(a: u8) -> Mask<i16, 4> {
        (u16x4::splat(a.into()) & u16x4::from_array([1, 2, 4, 8])).simd_ne(u16x4::splat(0))
    }
}

pub mod len_8 {
    use std::simd::{mask16x8, u16x8, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_simd(a: mask16x8) -> u8 {
        (a.to_int().cast() & u16x8::from_array([1, 2, 4, 8, 16, 32, 64, 128])).reduce_sum() as u8
    }

    #[inline]
    pub fn to_bitmask_mul(a: mask16x8) -> u8 {
        let word = u64::from_le_bytes(a.to_int().cast().to_array());
        ((word.wrapping_mul(0x2060e1e3e7f) >> 49) & 0xff) as u8
    }

    // At this point I'm not bothering with scalar implementations anymore

    #[inline]
    pub fn from_bitmask_simd(a: u8) -> mask16x8 {
        (u16x8::splat(a.into()) & u16x8::from_array([1, 2, 4, 8, 16, 32, 64, 128])).simd_ne(u16x8::splat(0))
    }
}