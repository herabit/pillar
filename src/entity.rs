//! Module for handling entities.
use core::{
    fmt, mem,
    num::NonZero,
    ops::{Deref, DerefMut},
};

pub(crate) mod data;
pub(crate) mod generation;
pub(crate) mod index;

#[doc(inline)]
pub use generation::*;
#[doc(inline)]
pub use index::*;

use crate::entity::data::{EntityData, RawData};

/// A pseudo-unique identifier entities.
///
/// # Uniqueness
///
/// There is a *chance* of aliasing.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entity {
    // SAFETY: This must be a valid `EntityData`.
    bits: NonZero<u64>,
}

const _: () = {
    assert!(size_of::<Entity>() == size_of::<EntityData>());
    assert!(align_of::<Entity>() == align_of::<EntityData>());
    assert!(align_of::<u32>() <= align_of::<u64>());
};

impl Entity {
    /// A placeholder [`Entity`].
    pub const PLACEHOLDER: Entity = Entity::new(EntityIndex::PLACEHOLDER, EntityGen::MIN);

    /// Create a new [`Entity`] given an index and generation.
    #[inline(always)]
    #[must_use]
    pub const fn new(index: EntityIndex, generation: EntityGen) -> Entity {
        // SAFETY: We guarantee that `EntityData` and `Entity` have the same
        //         memory layout and bit validity.
        unsafe {
            mem::transmute(EntityData {
                _align: [],
                generation,
                index,
            })
        }
    }

    /// Get a reference to this [`Entity`]'s underlying data.
    #[inline(always)]
    #[must_use]
    pub const fn data(&self) -> &EntityData {
        // SAFETY: We guarantee that `EntityData` and `Entity` have the same memory layout
        //         and bit validity.
        unsafe { &*(self as *const Entity as *const EntityData) }
    }

    /// Get a mutable reference to this [`Entity`]'s underlying data.
    #[inline(always)]
    #[must_use]
    pub const fn data_mut(&mut self) -> &mut EntityData {
        // SAFETY: We guarantee that `EntityData` and `Entity` have the same memory layout
        //         and bit validity.
        unsafe { &mut *(self as *mut Entity as *mut EntityData) }
    }

    /// Get the bit representation of this entity.
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u64 {
        // SAFETY: We guarantee that `EntityData` and `Entity` have the same memory layout
        //         and bit validity.
        //
        // NOTE: I haven't checked, but I'm pretty sure this helps to convey to LLVM about
        //       the bit validity of `Entity`. This will assist optimizations, or not at all
        //       but still be reduced to a no-op.
        let bits: EntityData = unsafe { mem::transmute(self) };

        // SAFETY: `u64` is a plain-old-datatype and `EntityData` contains no uninit bits.
        unsafe { mem::transmute(bits) }
    }

    // Create an [`Entity`] from its bit representation.
    //
    // # Returns
    //
    // This will return [`None`] if `bits` is not a valid [`Entity`].
    #[inline(always)]
    #[must_use]
    pub const fn from_bits(bits: u64) -> Option<Entity> {
        let RawData {
            index, generation, ..
        } = RawData::decode(bits);

        let index = EntityIndex::from_bits(index);
        let generation = EntityGen::from_bits(generation);

        match (index, generation) {
            (Some(index), Some(generation)) => Some(Entity::new(index, generation)),
            _ => None,
        }
    }

    /// Returns the index for this [`Entity`].
    #[inline(always)]
    #[must_use]
    pub const fn index(self) -> EntityIndex {
        self.data().index
    }

    /// Returns the generation for this [`Entity`].
    #[inline(always)]
    #[must_use]
    pub const fn generation(self) -> EntityGen {
        self.data().generation
    }
}

impl PartialOrd for Entity {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.to_bits() < other.to_bits()
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.to_bits() <= other.to_bits()
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.to_bits() > other.to_bits()
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.to_bits() >= other.to_bits()
    }
}

impl Ord for Entity {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_bits().cmp(&other.to_bits())
    }
}

impl Deref for Entity {
    type Target = EntityData;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl DerefMut for Entity {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("index", &self.index)
            .field("generation", &self.generation)
            .finish()
    }
}

impl Default for Entity {
    #[inline(always)]
    fn default() -> Self {
        Entity::PLACEHOLDER
    }
}

impl From<(EntityGen, EntityIndex)> for Entity {
    #[inline(always)]
    fn from(value: (EntityGen, EntityIndex)) -> Self {
        Entity::new(value.1, value.0)
    }
}

impl From<(EntityIndex, EntityGen)> for Entity {
    #[inline(always)]
    fn from(value: (EntityIndex, EntityGen)) -> Self {
        Entity::new(value.0, value.1)
    }
}
