/// https://github.com/seL4/seL4_libs/blob/master/libsel4vka/include/vka/object.h
use super::{Allocator, Error};
use sel4_sys::*;

/// A wrapper to hold all the allocation information for an 'object'.
///
/// An object here is just combination of cptr and untyped allocation
/// The type and size of the allocation is also stored to make free
/// more convenient.
#[derive(Clone, Debug)]
pub struct VkaObject {
    pub cptr: seL4_CPtr,
    pub ut: seL4_Word,
    pub item_type: seL4_Word,
    pub size_bits: seL4_Word,
}

impl VkaObject {
    pub fn new() -> Self {
        VkaObject {
            cptr: 0,
            ut: 0,
            item_type: 0,
            size_bits: 0,
        }
    }
}

impl Allocator {
    pub fn vka_alloc_untyped(&mut self, size_bits: usize) -> Result<VkaObject, Error> {
        self.vka_alloc_object(api_object_seL4_UntypedObject, size_bits)
    }

    pub fn vka_alloc_tcb(&mut self) -> Result<VkaObject, Error> {
        self.vka_alloc_object(api_object_seL4_TCBObject, seL4_TCBBits as _)
    }

    pub fn vka_alloc_endpoint(&mut self) -> Result<VkaObject, Error> {
        self.vka_alloc_object(api_object_seL4_EndpointObject, seL4_EndpointBits as _)
    }

    pub fn vka_alloc_notification(&mut self) -> Result<VkaObject, Error> {
        self.vka_alloc_object(
            api_object_seL4_NotificationObject,
            seL4_NotificationBits as _,
        )
    }

    // TODO - need to do kobject_get_type()
    pub fn vka_alloc_frame(&mut self, size_bits: usize) -> Result<VkaObject, Error> {
        self.vka_alloc_object(_object_seL4_ARM_SmallPageObject, size_bits)
    }

    pub fn vka_alloc_page_table(&mut self) -> Result<VkaObject, Error> {
        self.vka_alloc_object(_object_seL4_ARM_PageTableObject, seL4_PageTableBits as _)
    }

    /// Generic object allocator.
    /// TODO - use latest from seL4, this is from SMACCM repo
    /// https://github.com/smaccm/seL4_libs/blob/master/libsel4vka/include/vka/object.h#L38
    /// https://github.com/seL4/seL4_libs/blob/master/libsel4vka/include/vka/object.h#L75
    pub fn vka_alloc_object(
        &mut self,
        obj_type: seL4_Word,
        size_bits: usize,
    ) -> Result<VkaObject, Error> {
        let mut result: VkaObject = VkaObject::new();

        result.cptr = self.vka_cspace_alloc()?;

        let path = self.vka_cspace_make_path(result.cptr);

        result.ut = self.vka_utspace_alloc(&path, obj_type, size_bits)?;

        result.item_type = obj_type;
        result.size_bits = size_bits as _;

        Ok(result)
    }
}
