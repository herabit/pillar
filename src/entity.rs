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

use crate::entity::data::EntityData;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entity {
    // SAFETY: This must be a valid `EntityData`.
    bits: NonZero<u64>,
}

const _: () = assert!(size_of::<Entity>() == size_of::<EntityData>());
const _: () = assert!(align_of::<Entity>() == align_of::<EntityData>());

impl Entity {
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
        // Since the index is NOT-ed as a workaround for custom niches, we need to undo that operation.
        self.bits.get() ^ ((u32::MAX as u64) << 32)
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
