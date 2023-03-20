pub mod len_2 {
    use std::simd::{Mask, Simd, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_scalar(a: Mask<i32, 2>) -> u8 {
        let [a, b] = a.to_array();
        u8::from(a) | (2 * u8::from(b))
    }

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i32, 2>) -> u8 {
        (a.to_int().cast() & Simd::<u32, 2>::from_array([1, 2])).reduce_sum() as u8
    }

    #[inline]
    pub fn to_bitmask_shift(a: Mask<i32, 2>) -> u8 {
        let word: u64 = unsafe { std::mem::transmute(a) };
        ((word >> 31) & 0b11) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i32, 2> {
        let a = a as i32;
        // For some reason this generates shorter code than just using `Mask::from_array`
        unsafe {
            Mask::from_int_unchecked(
                Simd::from_array([-(a & 1), -((a & 2) / 2)]).cast(),
            )
        }
    }

    #[inline]
    pub fn from_bitmask_simd(a: u8) -> Mask<i32, 2> {
        (Simd::splat(a as u32) & Simd::from_array([1, 2])).simd_ne(Simd::splat(0))
    }
}

pub mod len_4 {
    use std::simd::{Mask, Simd, SimdUint, i32x4, u32x4, SimdPartialEq};

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i32, 4>) -> u8 {
        (a.to_int().cast() & Simd::<u32, 4>::from_array([1, 2, 4, 8])).reduce_sum() as u8
    }
    
    #[inline]
    pub fn to_bitmask_mul(a: Mask<i32, 4>) -> u8 {
        let word: u128 = unsafe { std::mem::transmute(a.to_int()) };
        ((word.wrapping_mul(0xf0f0f0f1f1f1f1f3f3f3f3f7f7f7f80) >> 100) & 0xf) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i32, 4> {
        let a = a as i32;
        unsafe {
            Mask::from_int_unchecked(
                i32x4::from_array([
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
    pub fn from_bitmask_simd(a: u8) -> Mask<i32, 4> {
        (u32x4::splat(a.into()) & u32x4::from_array([1, 2, 4, 8])).simd_ne(u32x4::splat(0))
    }
}