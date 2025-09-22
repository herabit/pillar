use crate::entity::{EntityGen, EntityIndex};
use core::mem::{self, offset_of};

/// The underlying data stored within an [`Entity`].
///
/// # Bit Representation
///
/// Do not reorder the fields.
///
/// They're placed carefully such that `index` lies within the
/// most significant bits of a [`u64`] when transmuted to one.
///
/// This ensures that when computing the [`Ord`] of an [`Entity`],
/// that the index is ordered *first* and then the generation, but
/// with better codegen as it gets reduced to a single [`Ord`] on
/// two [`u64`]s.
///
/// This is desireable because if two entities share an index but
/// have different generations, we want them to be ordered in terms
/// of their generation, as their indices are equivalent.
///
/// # Memory Layout
///
/// This is guaranteed to have the same size, alignment, and bit
/// validity as an [`Entity`]. Do note, however, that this does not
/// mean they share the same niche values. Just that it is safe to reinterpet
/// one as the other.
#[repr(C)]
#[non_exhaustive]
pub struct EntityData {
    // SAFETY: Do not remove, this ensures that this struct
    //         has the same alignment as a `u64`.
    pub(super) _align: [u64; 0],
    /// The index of the entity.
    #[cfg(target_endian = "big")]
    pub index: EntityIndex,
    /// The generation of the entity.
    pub generation: EntityGen,
    /// The index of the entity.
    #[cfg(target_endian = "little")]
    pub index: EntityIndex,
}

/// Something with an equivalent memory layout to [`EntityData`], but
/// is just a plain-old-datatype.
///
/// It must ***always match*** the layout of [`EntityData`], but has loser
/// bit validity requirements.
///
/// This is just an implementation detail for assisting a few things.
#[derive(Clone, Copy)]
#[repr(C)]
#[non_exhaustive]
pub(super) struct RawData {
    pub(super) _align: [u64; 0],
    #[cfg(target_endian = "big")]
    pub(super) index: u32,
    pub(super) generation: u32,
    #[cfg(target_endian = "little")]
    pub(super) index: u32,
}

#[allow(dead_code)]
impl RawData {
    #[inline(always)]
    #[must_use]
    pub(super) const fn decode(bits: u64) -> RawData {
        // SAFETY: `RawData` and `u64` are both POD.
        unsafe { mem::transmute(bits) }
    }

    #[inline(always)]
    #[must_use]
    pub(super) const fn encode(self) -> u64 {
        // SAFETY: `RawData` and `u64` are both POD.
        unsafe { mem::transmute(self) }
    }
}

const _: () = {
    // Ensure the layout constraints are the same.
    assert!(size_of::<RawData>() == size_of::<EntityData>());
    assert!(align_of::<RawData>() == align_of::<EntityData>());

    // Ensure that the fields are the same.
    assert!(offset_of!(RawData, _align) == offset_of!(EntityData, _align));
    assert!(offset_of!(RawData, index) == offset_of!(EntityData, index));
    assert!(offset_of!(RawData, generation) == offset_of!(EntityData, generation));
};
