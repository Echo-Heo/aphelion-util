//! Useful stuff
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

const fn option_u64(v: Option<i64>) -> Option<u64> {
    if let Some(v) = v {
        Some(v as u64)
    } else {
        None
    }
}

#[must_use]
pub const fn sign_extend<const BIT_SIZE: u8>(val: u64) -> u64 {
    let shift = 64 - BIT_SIZE;
    (((val << shift) as i64) >> shift) as u64
}

pub mod ops {
    //! Operations
    use super::option_u64;
    use crate::{
        instruction::instruction_set::{FloatCastType, FloatPrecision},
        nibble::Nibble,
    };
    pub use half::f16;

    #[derive(Debug, Clone, Copy)]
    pub struct AddResult {
        pub result: u64,
        pub unsigned_overflow: bool,
        pub signed_overflow: bool,
    }
    #[must_use]
    pub const fn add(a: u64, b: u64, carry: bool) -> AddResult {
        const fn carrying_add_u(a: u64, b: u64, carry: bool) -> (u64, bool) {
            let (v, a) = a.overflowing_add(b);
            let (v, b) = v.overflowing_add(carry as u64);
            (v, a | b)
        }
        const fn carrying_add_i(a: u64, b: u64, carry: bool) -> (u64, bool) {
            let (v, a) = (a as i64).overflowing_add(b as i64);
            let (v, b) = v.overflowing_add(carry as i64);
            (v as u64, a ^ b)
        }
        let (result, unsigned_overflow) = carrying_add_u(a, b, carry);
        let (_, signed_overflow) = carrying_add_i(a, b, carry);
        AddResult {
            result,
            unsigned_overflow,
            signed_overflow,
        }
    }
    #[must_use]
    pub const fn sub(a: u64, b: u64, carry: bool) -> AddResult {
        const fn carrying_sub_u(a: u64, b: u64, carry: bool) -> (u64, bool) {
            let (v, a) = a.overflowing_sub(b);
            let (v, b) = v.overflowing_sub(carry as u64);
            (v, a | b)
        }
        const fn carrying_sub_i(a: u64, b: u64, carry: bool) -> (u64, bool) {
            let (v, a) = (a as i64).overflowing_sub(b as i64);
            let (v, b) = v.overflowing_sub(carry as i64);
            (v as u64, a ^ b)
        }
        let (result, unsigned_overflow) = carrying_sub_u(a, b, carry);
        let (_, signed_overflow) = carrying_sub_i(a, b, carry);
        AddResult {
            result,
            unsigned_overflow,
            signed_overflow,
        }
    }

    #[must_use]
    pub const fn imul(a: u64, b: u64) -> u64 {
        (a as i64).wrapping_mul(b as i64) as u64
    }

    #[must_use]
    pub const fn idiv(a: u64, b: u64) -> Option<u64> {
        option_u64((a as i64).checked_div(b as i64))
    }

    #[must_use]
    pub const fn umul(a: u64, b: u64) -> u64 {
        a.wrapping_mul(b)
    }

    #[must_use]
    pub const fn udiv(a: u64, b: u64) -> Option<u64> {
        a.checked_div(b)
    }

    #[must_use]
    pub const fn rem(a: u64, b: u64) -> Option<u64> {
        option_u64((a as i64).checked_rem(b as i64))
    }

    #[must_use]
    pub const fn r#mod(a: u64, b: u64) -> Option<u64> {
        option_u64((a as i64).checked_rem_euclid(b as i64))
    }

    #[must_use]
    pub const fn and(a: u64, b: u64) -> u64 {
        a & b
    }

    #[must_use]
    pub const fn or(a: u64, b: u64) -> u64 {
        a | b
    }

    #[must_use]
    pub const fn nor(a: u64, b: u64) -> u64 {
        !(a | b)
    }

    #[must_use]
    pub const fn xor(a: u64, b: u64) -> u64 {
        a ^ b
    }

    #[must_use]
    pub const fn shl(a: u64, b: u64) -> u64 {
        a << b
    }

    #[must_use]
    pub const fn asr(a: u64, b: u64) -> u64 {
        ((a as i64) >> b) as u64
    }

    #[must_use]
    pub const fn shr(a: u64, b: u64) -> u64 {
        a >> b
    }

    #[must_use]
    pub const fn bit(a: u64, b: u64) -> u64 {
        (a >> b) & 1
    }

    pub trait BitAccessTo<To: BitAccess<Self>>: Copy {
        fn access_to<const INDEX: u8>(to: To) -> Self;
        fn write_to<const INDEX: u8>(to: &mut To, v: Self);
    }

    pub trait BitAccess<From: Copy>: Copy {
        fn access<const INDEX: u8>(self) -> From;
        fn write<const INDEX: u8>(&mut self, v: From);
    }

    impl<U: Copy, T: BitAccessTo<U>> BitAccess<T> for U {
        fn access<const INDEX: u8>(self) -> T {
            T::access_to::<INDEX>(self)
        }
        fn write<const INDEX: u8>(&mut self, v: T) {
            T::write_to::<INDEX>(self, v);
        }
    }

    macro_rules! impl_bit_access_from_bool {
        (Nibble) => {
            impl BitAccessTo<Nibble> for bool {
                #[inline]
                fn access_to<const INDEX: u8>(to: Nibble) -> Self { (to as u8 >> INDEX) != 0 }
                #[inline]
                fn write_to<const INDEX: u8>(to: &mut Nibble, v: Self) {
                    if INDEX > 4 {
                        return;
                    }
                    if v {
                        *to = Nibble::from_u8(*to as u8 | 1 << INDEX);
                    } else {
                        *to = Nibble::from_u8(*to as u8 & !(1 << INDEX));
                    }
                }
            }
        };
        ($type: ident) => {
            impl BitAccessTo<$type> for bool {
                #[inline]
                fn access_to<const INDEX: u8>(to: $type) -> Self { (to >> (INDEX as $type)) != 0 }
                #[inline]
                fn write_to<const INDEX: u8>(to: &mut $type, v: Self) {
                    if v {
                        *to |= 1 << (INDEX as $type);
                    } else {
                        *to &= !(1 << (INDEX as $type));
                    }
                }
            }
        };
        ($($type: ident),* $(,)*) => {
            $(impl_bit_access_from_bool! {$type})*
        }
    }

    impl_bit_access_from_bool! {u8, u16, u32, u64, Nibble}

    macro_rules! impl_bit_access_to {
        ($to: ident, [$type: ident, $zero: expr, $size: expr]) => {
            impl BitAccessTo<$to> for $type {
                #[inline]
                fn access_to<const INDEX: u8>(to: $to) -> Self { (to >> ($to::from(INDEX) * $size)) as Self }
                #[inline]
                fn write_to<const INDEX: u8>(to: &mut $to, v: Self) {
                    *to &= !($to::from(!$zero) << ($to::from(INDEX) * $size));
                    *to |= $to::from(v) << ($to::from(INDEX) * $size);
                }
            }
        };
        ($to: ident; $([$type: ident, $zero: expr, $size: expr]),* $(,)*) => {
            $(impl_bit_access_to!{$to, [$type, $zero, $size]})*
        };
    }

    impl_bit_access_to! {
        u64; [u8, 0u8, 8], [u16, 0u16, 16], [u32, 0u32, 32], [u64, 0u64, 64],
    }

    impl_bit_access_to! {
        u32; [u8, 0u8, 8], [u16, 0u16, 16], [u32, 0u32, 32],
    }

    impl_bit_access_to! {
        u16; [u8, 0u8, 8], [u16, 0u16, 16],
    }

    impl_bit_access_to! {
        u8; [u8, 0u8, 8],
    }

    pub trait Float: Copy + num_traits::float::Float {
        type Bits: Into<u64> + BitAccessTo<u64> + Copy;
        #[must_use]
        fn from_bits(v: Self::Bits) -> Self;
        fn to_bits(self) -> Self::Bits;
        #[must_use]
        fn from_u64(v: u64) -> Self {
            Self::from_bits(v.access::<0>())
        }
        fn write_u64(self, v: &mut u64) {
            *v = self.to_bits().into();
        }
        fn to_u64(self) -> u64 {
            self.to_bits().into()
        }
        fn cast_to_int(self) -> i64 {
            self.to_i64().unwrap_or(0)
        }
        fn cast_from_int(v: i64) -> Self;
        #[must_use]
        fn feq(a: u64, b: u64) -> bool {
            Self::from_u64(a) == Self::from_u64(b)
        }
        #[must_use]
        fn flt(a: u64, b: u64) -> bool {
            Self::from_u64(a) < Self::from_u64(b)
        }
        #[must_use]
        fn flz(a: u64) -> bool {
            Self::from_u64(a).is_sign_negative()
        }

        #[must_use]
        fn fez(a: u64) -> bool {
            Self::from_u64(a).is_zero()
        }
        #[must_use]
        fn fto(int: u64) -> u64 {
            Self::cast_from_int(int as i64).to_u64()
        }
        #[must_use]
        fn ffrom(float: u64) -> u64 {
            Self::from_u64(float).cast_to_int() as u64
        }
        #[must_use]
        fn fneg(a: u64) -> u64 {
            Self::from_u64(a).neg().to_u64()
        }
        #[must_use]
        fn fabs(a: u64) -> u64 {
            Self::from_u64(a).abs().to_u64()
        }
        #[must_use]
        fn fadd(a: u64, b: u64) -> u64 {
            (Self::from_u64(a) + Self::from_u64(b)).to_u64()
        }
        #[must_use]
        fn fsub(a: u64, b: u64) -> u64 {
            (Self::from_u64(a) - Self::from_u64(b)).to_u64()
        }
        #[must_use]
        fn fmul(a: u64, b: u64) -> u64 {
            (Self::from_u64(a) * Self::from_u64(b)).to_u64()
        }
        #[must_use]
        fn fdiv(a: u64, b: u64) -> u64 {
            (Self::from_u64(a) / Self::from_u64(b)).to_u64()
        }
        fn fma(a: u64, b: u64, to: &mut u64) {
            (Self::from_u64(*to) + Self::from_u64(a) * Self::from_u64(b)).write_u64(to);
        }
        #[must_use]
        fn fsqrt(a: u64) -> u64 {
            Self::from_u64(a).sqrt().to_u64()
        }
        #[must_use]
        fn fmin(a: u64, b: u64) -> u64 {
            Self::from_u64(a).min(Self::from_u64(b)).to_u64()
        }
        #[must_use]
        fn fmax(a: u64, b: u64) -> u64 {
            Self::from_u64(a).max(Self::from_u64(b)).to_u64()
        }
        #[must_use]
        fn fsat(a: u64) -> u64 {
            Self::from_u64(a).ceil().to_u64()
        }
        #[must_use]
        fn fnan(a: u64) -> u64 {
            u64::from(Self::from_u64(a).is_nan())
        }
    }

    impl<F: Float> BitAccessTo<u64> for F {
        fn access_to<const INDEX: u8>(to: u64) -> Self {
            Self::from_bits(to.access::<INDEX>())
        }
        fn write_to<const INDEX: u8>(to: &mut u64, v: Self) {
            to.write::<INDEX>(v.to_bits());
        }
    }

    impl Float for f16 {
        type Bits = u16;
        fn from_bits(v: Self::Bits) -> Self {
            f16::from_bits(v)
        }
        fn to_bits(self) -> Self::Bits {
            self.to_bits()
        }
        #[allow(clippy::cast_precision_loss)]
        fn cast_from_int(v: i64) -> Self {
            Self::from_f32(v as f32)
        }
    }

    impl Float for f32 {
        type Bits = u32;
        fn from_bits(v: Self::Bits) -> Self {
            f32::from_bits(v)
        }
        fn to_bits(self) -> Self::Bits {
            self.to_bits()
        }
        #[allow(clippy::cast_precision_loss)]
        fn cast_from_int(v: i64) -> Self {
            v as Self
        }
    }

    impl Float for f64 {
        type Bits = u64;
        fn from_bits(v: Self::Bits) -> Self {
            f64::from_bits(v)
        }
        fn to_bits(self) -> Self::Bits {
            self.to_bits()
        }
        #[allow(clippy::cast_precision_loss)]
        fn cast_from_int(v: i64) -> Self {
            v as Self
        }
    }

    macro_rules! impl_float_precision {
        {
            impl $FloatPrecision: ident {
                $($(#[$attr: meta])?
                fn $fn_name: ident (self, $($name: ident: $type: ty),*) $( -> $return: ty)?;)*
            }
        } => {
            impl $FloatPrecision {
                $($(#[$attr])?
                pub fn $fn_name(self, $($name: $type),*) $(-> $return)? {
                    match self {
                        Self::F16 => <f16 as Float>::$fn_name($($name),*),
                        Self::F32 => <f32 as Float>::$fn_name($($name),*),
                        Self::F64 => <f64 as Float>::$fn_name($($name),*),
                    }
                })*
            }
        };
    }

    impl_float_precision! {
        impl FloatPrecision {
            #[must_use]
            fn feq(self, a: u64, b: u64) -> bool;
            #[must_use]
            fn flt(self, a: u64, b: u64) -> bool;
            #[must_use]
            fn flz(self, a: u64) -> bool;
            #[must_use]
            fn fez(self, a: u64) -> bool;
            #[must_use]
            fn fto(self, int: u64) -> u64;
            #[must_use]
            fn ffrom(self, float: u64) -> u64;
            #[must_use]
            fn fneg(self, a: u64) -> u64;
            #[must_use]
            fn fabs(self, a: u64) -> u64;
            #[must_use]
            fn fadd(self, a: u64, b: u64) -> u64;
            #[must_use]
            fn fsub(self, a: u64, b: u64) -> u64;
            #[must_use]
            fn fmul(self, a: u64, b: u64) -> u64;
            #[must_use]
            fn fdiv(self, a: u64, b: u64) -> u64;
            fn fma(self, a: u64, b: u64, to: &mut u64);
            #[must_use]
            fn fsqrt(self, a: u64) -> u64;
            #[must_use]
            fn fmin(self, a: u64, b: u64) -> u64;
            #[must_use]
            fn fmax(self, a: u64, b: u64) -> u64;
            #[must_use]
            fn fsat(self, a: u64) -> u64;
            #[must_use]
            fn fnan(self, a: u64) -> u64;
        }
    }

    impl FloatCastType {
        #[must_use]
        #[allow(clippy::cast_lossless)]
        #[allow(clippy::cast_possible_truncation)]
        pub fn cast(self, a: u64) -> u64 {
            match self.from {
                FloatPrecision::F16 => {
                    let from = <f16 as Float>::from_u64(a);
                    match self.to {
                        FloatPrecision::F16 => a,
                        FloatPrecision::F32 => from.to_f32().to_u64(),
                        FloatPrecision::F64 => from.to_f64().to_u64(),
                    }
                }
                FloatPrecision::F32 => {
                    let from = <f32 as Float>::from_u64(a);
                    match self.to {
                        FloatPrecision::F16 => f16::from_f32(from).to_u64(),
                        FloatPrecision::F32 => a,
                        FloatPrecision::F64 => (from as f64).to_u64(),
                    }
                }
                FloatPrecision::F64 => {
                    let from = <f64 as Float>::from_u64(a);
                    match self.to {
                        FloatPrecision::F16 => f16::from_f64(from).to_u64(),
                        FloatPrecision::F32 => (from as f32).to_u64(),
                        FloatPrecision::F64 => a,
                    }
                }
            }
        }
    }
}
