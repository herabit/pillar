use core::fmt;

/// An entity's generation.
///
/// # Representation
///
/// The current representation is not guaranteed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EntityGen {
    bits: u32,
}

impl EntityGen {
    /// The minimum [`EntityGen`], `0`.
    pub const MIN: EntityGen = EntityGen::new(0).unwrap();
    /// The maximum [`EntityGen`], `u32::MAX`.
    pub const MAX: EntityGen = EntityGen::new(u32::MAX).unwrap();

    /// Create a new [`EntityGen`] from its underlying
    /// bit representation.
    ///
    /// # Returns
    ///
    /// Currently this never returns [`None`], but this is
    /// subject to change in the future.
    #[inline(always)]
    #[must_use]
    pub const fn from_bits(bits: u32) -> Option<EntityGen> {
        Some(EntityGen { bits })
    }

    /// Get the underlying bit representation.
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }

    /// Create a new [`EntityGen`].
    ///
    /// # Returns
    ///
    /// Currently this never returns [`None`], but this is
    /// subject to change in the future.
    #[inline(always)]
    #[must_use]
    pub const fn new(generation: u32) -> Option<EntityGen> {
        EntityGen::from_bits(generation)
    }

    /// Get the value of this generation.
    #[inline(always)]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.to_bits()
    }
}

impl fmt::Debug for EntityGen {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl fmt::Display for EntityGen {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl Default for EntityGen {
    #[inline(always)]
    fn default() -> Self {
        EntityGen::MIN
    }
}
