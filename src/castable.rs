/// `CastableInto : CastableFrom = Into : From`, It's easier to use to
/// specify bounds on generic variables
pub trait CastableInto<W>: Sized {
    /// Call `W::cast_from(self)`
    fn cast(self) -> W;
}

/// Trait for primitive integers, this is the combination of
/// [`DowncastableFrom`](`crate::downcastable::DowncastableFrom`) and [`UpcastableFrom`](`crate::upcastable::UpcastableFrom`). Prefer using the other two
/// traits, as casting without knowing which value will be bigger might result
/// in hard to find bugs.
///
/// This is equivalent to calling `as` between two types
pub trait CastableFrom<W>: Sized {
    /// Call `Self as W`
    fn cast_from(value: W) -> Self;
}

/// Riflexivity
impl<T> CastableFrom<T> for T {
    #[inline(always)]
    fn cast_from(value: T) -> Self {
        value
    }
}

/// UpcastableFrom implies UpcastableInto
impl<T, U> CastableInto<U> for T
where
    U: CastableFrom<T>,
{
    #[inline(always)]
    fn cast(self) -> U {
        U::cast_from(self)
    }
}

macro_rules! impl_casts {
    ($base_type:ty, $($ty:ty,)*) => {$(
impl CastableFrom<$base_type> for $ty {
    #[inline(always)]
    fn cast_from(value: $base_type) -> Self {
        value as $ty
    }
}
impl CastableFrom<$ty> for $base_type {
    #[inline(always)]
    fn cast_from(value: $ty) -> $base_type {
        value as $base_type
    }
}
    )*
    impl_casts!($($ty,)*);
};
    () => {};
}

impl_casts!(u8, u16, u32, u64, u128, usize,);
impl_casts!(i8, i16, i32, i64, i128, isize,);

impl_casts!(f32, f64,);

#[cfg(feature = "half")]
mod half_impl {
    use super::*;

    impl CastableFrom<f32> for half::f16 {
        #[inline(always)]
        fn cast_from(value: f32) -> Self {
            Self::from_f32(value)
        }
    }
    impl CastableFrom<f64> for half::f16 {
        #[inline(always)]
        fn cast_from(value: f64) -> Self {
            Self::from_f64(value)
        }
    }
    impl CastableFrom<f32> for half::bf16 {
        #[inline(always)]
        fn cast_from(value: f32) -> Self {
            Self::from_f32(value)
        }
    }
    impl CastableFrom<f64> for half::bf16 {
        #[inline(always)]
        fn cast_from(value: f64) -> Self {
            Self::from_f64(value)
        }
    }
    impl CastableFrom<half::f16> for f32 {
        #[inline(always)]
        fn cast_from(value: half::f16) -> Self {
            value.to_f32()
        }
    }
    impl CastableFrom<half::bf16> for f32 {
        #[inline(always)]
        fn cast_from(value: half::bf16) -> Self {
            value.to_f32()
        }
    }
    impl CastableFrom<half::f16> for f64 {
        #[inline(always)]
        fn cast_from(value: half::f16) -> Self {
            value.to_f64()
        }
    }
    impl CastableFrom<half::bf16> for f64 {
        #[inline(always)]
        fn cast_from(value: half::bf16) -> Self {
            value.to_f64()
        }
    }
}

#[cfg(feature = "nightly_f16")]
mod nightly_f16_impl {
    use super::*;

    impl CastableFrom<f32> for f16 {
        #[inline(always)]
        fn cast_from(value: f32) -> Self {
            value as f16
        }
    }
    impl CastableFrom<f64> for f16 {
        #[inline(always)]
        fn cast_from(value: f64) -> Self {
            value as f16
        }
    }
    impl CastableFrom<f16> for f32 {
        #[inline(always)]
        fn cast_from(value: f16) -> Self {
            value as f32
        }
    }
    impl CastableFrom<f16> for f64 {
        #[inline(always)]
        fn cast_from(value: f16) -> Self {
            value as f64
        }
    }
}
