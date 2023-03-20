pub mod len_2 {
    use std::simd::{Mask, Simd, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_scalar(a: Mask<i8, 2>) -> u8 {
        let [a, b] = a.to_array();
        u8::from(a) | (2 * u8::from(b))
    }

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i8, 2>) -> u8 {
        (a.to_int().cast() & Simd::<u8, 2>::from_array([1, 2])).reduce_sum()
    }

    // I very strongly suspect that this will be the fastest of the three implementations.
    #[inline]
    pub fn to_bitmask_shift(a: Mask<i8, 2>) -> u8 {
        ((u16::from_le_bytes(a.to_int().cast().to_array()) >> 7) & 0b11) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i8, 2> {
        // For some reason this generates shorter code than just using `Mask::from_array`
        unsafe {
            Mask::from_int_unchecked(
                Simd::from_array([(a & 1).wrapping_neg(), ((a & 2) / 2).wrapping_neg()]).cast(),
            )
        }
    }

    #[inline]
    pub fn from_bitmask_simd(a: u8) -> Mask<i8, 2> {
        (Simd::splat(a) & Simd::from_array([1, 2])).simd_ne(Simd::splat(0))
    }
}

pub mod len_4 {
    use std::simd::{u8x4, Mask, Simd, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_simd(a: Mask<i8, 4>) -> u8 {
        (a.to_int().cast() & Simd::<u8, 4>::from_array([1, 2, 4, 8])).reduce_sum()
    }

    // I suspect this will be the faster of the 2 implementations.
    #[inline]
    pub fn to_bitmask_mul(a: Mask<i8, 4>) -> u8 {
        let word = u32::from_le_bytes(a.to_int().cast().to_array());
        // the magic constants for the factor and shift value were found using Z3 as described in this article: <https://zeux.io/2022/09/02/vpexpandb-neon-z3/>
        // the smallest possible factor was used because loading larger numbers into registers can take multiple instructions on aarch64.
        ((word.wrapping_mul(0xa1a4) >> 23) & 0xf) as u8
    }

    #[inline]
    pub fn from_bitmask_scalar(a: u8) -> Mask<i8, 4> {
        unsafe {
            Mask::from_int_unchecked(
                u8x4::from_array([
                    (a & 1).wrapping_neg(),
                    ((a & 2) / 2).wrapping_neg(),
                    ((a & 4) / 4).wrapping_neg(),
                    ((a & 8) / 8).wrapping_neg(),
                ])
                .cast(),
            )
        }
    }

    #[inline]
    // I suspect this will the the faster of the 2 implementations
    pub fn from_bitmask_simd(a: u8) -> Mask<i8, 4> {
        (u8x4::splat(a) & u8x4::from_array([1, 2, 4, 8])).simd_ne(u8x4::splat(0))
    }
}

pub mod len_8 {
    use std::simd::{mask8x8, u8x8, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_simd(a: mask8x8) -> u8 {
        (a.to_int().cast() & u8x8::from_array([1, 2, 4, 8, 16, 32, 64, 128])).reduce_sum()
    }

    #[inline]
    pub fn to_bitmask_mul(a: mask8x8) -> u8 {
        let word = u64::from_le_bytes(a.to_int().cast().to_array());
        ((word.wrapping_mul(0x2060e1e3e7f) >> 49) & 0xff) as u8
    }

    // At this point I'm not bothering with scalar implementations anymore

    #[inline]
    pub fn from_bitmask_simd(a: u8) -> mask8x8 {
        (u8x8::splat(a) & u8x8::from_array([1, 2, 4, 8, 16, 32, 64, 128])).simd_ne(u8x8::splat(0))
    }
}

pub mod len_16 {
    use std::simd::{mask8x16, mask8x8, simd_swizzle, u8x16, Simd, SimdPartialEq, SimdUint};

    #[inline]
    pub fn to_bitmask_simd(a: mask8x16) -> u16 {
        let vec = a.to_int().cast()
            & u8x16::from_array([1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128]);
        let lo = simd_swizzle!(vec, [0, 1, 2, 3, 4, 5, 6, 7]);
        let hi = simd_swizzle!(vec, [8, 9, 10, 11, 12, 13, 14, 15]);
        u16::from(hi.reduce_sum()) << 8 | u16::from(lo.reduce_sum())
    }

    #[inline]
    pub fn to_bitmask_mul(a: mask8x16) -> u16 {
        use crate::elem_8bit::len_8::to_bitmask_mul as to_bitmask_8;

        // there's no support for swizzling masks, so we need to round trip to int.
        let vec = a.to_int().cast();
        let lo = simd_swizzle!(vec, [0, 1, 2, 3, 4, 5, 6, 7]);
        let hi = simd_swizzle!(vec, [8, 9, 10, 11, 12, 13, 14, 15]);
        let lo = to_bitmask_8(unsafe { mask8x8::from_int_unchecked(lo) });
        let hi = to_bitmask_8(unsafe { mask8x8::from_int_unchecked(hi) });

        u16::from(hi) << 8 | u16::from(lo)
    }

    #[inline]
    pub fn from_bitmask_simd(a: u16) -> mask8x16 {
        let a = Simd::from_array(a.to_le_bytes());
        let a = simd_swizzle!(a, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]);
        (a & u8x16::from_array([1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128]))
            .simd_ne(u8x16::splat(0))
    }
}
