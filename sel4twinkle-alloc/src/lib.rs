#![no_std]

/// A Rust port of libsel4twinkle allocator.
///
/// https://github.com/smaccm/libsel4twinkle
///
/// TODO docs and such
///
extern crate sel4_sys;

use sel4_sys::seL4_CPtr;

mod allocator;
mod cspacepath;
mod first_stage_allocator;
mod object_allocator;
mod vka;
mod vka_object;

pub const MIN_UNTYPED_SIZE: usize = 4;
pub const MAX_UNTYPED_SIZE: usize = 32;

pub const MAX_UNTYPED_ITEMS: usize = 256;

#[derive(Clone)]
pub struct UntypedItem {
    cap: seL4_CPtr,
    size_bits: usize,
}

#[derive(Clone)]
pub struct CapRange {
    first: usize,
    count: usize,
}

#[derive(Clone)]
struct InitUntypedItem {
    item: UntypedItem,
    is_free: bool,
}

pub struct Allocator {
    /// CNode we allocate from
    root_cnode: seL4_CPtr,
    root_cnode_depth: seL4_CPtr,
    root_cnode_offset: seL4_CPtr,

    /// Range of free slots in the root cnode
    cslots: CapRange,

    /// Number fo slots we've used
    num_slots_used: usize,

    /// Initial memory items
    num_init_untyped_items: usize,
    init_untyped_items: [InitUntypedItem; MAX_UNTYPED_ITEMS],

    /// Untyped memory items we have created
    untyped_items: [CapRange; (MAX_UNTYPED_SIZE - MIN_UNTYPED_SIZE) + 1],
}
