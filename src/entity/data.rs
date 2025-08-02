use core::mem::offset_of;

use crate::entity::{EntityGen, EntityIndex};

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
