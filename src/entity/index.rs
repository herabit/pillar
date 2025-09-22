use core::fmt;
use core::num::NonZero;
use core::num::TryFromIntError;

/// An [`Entity`](super::Entity)'s index.
///
/// # Representation
///
/// Currently one should ***not*** rely upon the bit representation of this type. It is subject to
/// change when the Rust type system gets new features in stable.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityIndex {
    /// The index incremented by one. This representation should not be relied upon.
    bits: NonZero<u32>,
}

impl EntityIndex {
    /// The minimum [`EntityIndex`], `0`.
    pub const MIN: EntityIndex = EntityIndex::new(0).unwrap();
    /// The maximum [`EntityIndex`], `u32::MAX - 1`.
    pub const MAX: EntityIndex = EntityIndex::new(u32::MAX - 1).unwrap();

    /// The placeholder [`EntityIndex`], which is just [`EntityIndex::MAX`].
    pub const PLACEHOLDER: EntityIndex = EntityIndex::MAX;

    /// Create a new [`EntityIndex`] from its bit representation.
    ///
    /// # Returns
    ///
    /// Returns [`None`] if `bits` is not a valid bit representation for
    /// an [`EntityIndex`].
    #[inline(always)]
    #[must_use]
    pub const fn from_bits(bits: u32) -> Option<EntityIndex> {
        match NonZero::new(bits) {
            Some(bits) => Some(EntityIndex { bits }),
            None => None,
        }
    }

    /// Get the underlying bit representation of this [`EntityIndex`].
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.bits.get()
    }

    /// Create a new [`EntityIndex`] from an index.
    ///
    /// # Returns
    ///
    /// Returns [`None`] if `index` is equal to [`u32::MAX`].
    #[inline(always)]
    #[must_use]
    pub const fn new(index: u32) -> Option<EntityIndex> {
        // NOTE: If `index == u32::MAX`, then the result will wrap to zero.
        //       Just in case this isn't immediately obvious to others.
        EntityIndex::from_bits(index.wrapping_add(1))
    }

    /// Get the underlying value of this [`EntityIndex`].
    #[inline(always)]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.bits.get() - 1
    }

    /// Returns whether this [`EntityIndex`] is the placeholder
    /// value.
    #[inline(always)]
    #[must_use]
    pub const fn is_placeholder(self) -> bool {
        matches!(self, EntityIndex::PLACEHOLDER)
    }
}

impl fmt::Debug for EntityIndex {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

impl fmt::Display for EntityIndex {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

impl Default for EntityIndex {
    #[inline(always)]
    fn default() -> Self {
        EntityIndex::PLACEHOLDER
    }
}

impl PartialOrd for EntityIndex {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityIndex {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

macro_rules! try_from {
    (
        $ty:ident
    ) => {
        impl TryFrom<$ty> for EntityIndex {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                let error = NonZero::try_from(0).unwrap_err();

                u32::try_from(value)
                    .map_err(From::from)
                    .and_then(|value| EntityIndex::new(value).ok_or(error))
            }
        }

        impl TryFrom<NonZero<$ty>> for EntityIndex {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: NonZero<$ty>) -> Result<Self, Self::Error> {
                value.get().try_into()
            }
        }
    };
}

try_from!(u32);
try_from!(u64);
try_from!(u128);
try_from!(usize);

try_from!(i8);
try_from!(i16);
try_from!(i32);
try_from!(i64);
try_from!(i128);
try_from!(isize);

macro_rules! from {
    ($ty:ident) => {
        impl From<$ty> for EntityIndex {
            #[inline(always)]
            fn from(value: $ty) -> Self {
                EntityIndex::new(value.into()).unwrap()
            }
        }

        impl From<NonZero<$ty>> for EntityIndex {
            #[inline(always)]
            fn from(value: NonZero<$ty>) -> Self {
                value.get().into()
            }
        }
    };
}

from!(u8);
from!(u16);

macro_rules! try_into {
    ($ty:ident) => {
        impl TryFrom<EntityIndex> for $ty {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: EntityIndex) -> Result<Self, Self::Error> {
                $ty::try_from(value.get())
            }
        }

        impl TryFrom<EntityIndex> for NonZero<$ty> {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: EntityIndex) -> Result<Self, Self::Error> {
                NonZero::try_from(value.get()).and_then(TryFrom::try_from)
            }
        }
    };
}

try_into!(u8);
try_into!(u16);
try_into!(usize);

try_into!(i8);
try_into!(i16);
try_into!(i32);
try_into!(isize);

macro_rules! into {
    ($ty:ident) => {
        impl From<EntityIndex> for $ty {
            #[inline(always)]
            fn from(value: EntityIndex) -> Self {
                value.get().into()
            }
        }

        impl TryFrom<EntityIndex> for NonZero<$ty> {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: EntityIndex) -> Result<Self, Self::Error> {
                NonZero::try_from(value.get()).map(From::from)
            }
        }
    };
}

into!(u32);
into!(u64);
into!(u128);

into!(i64);
into!(i128);
