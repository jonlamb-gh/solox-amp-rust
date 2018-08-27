/// TODO - need a proper VKA abstration and implementation

use super::Allocator;
use cspacepath::CSpacePath;
use sel4_sys::{
    api_object_seL4_CapTableObject, api_object_seL4_EndpointObject,
    api_object_seL4_NotificationObject, api_object_seL4_TCBObject, api_object_seL4_UntypedObject,
    seL4_CPtr, seL4_CapInitThreadCNode, seL4_EndpointBits, seL4_NotificationBits, seL4_SlotBits,
    seL4_TCBBits, seL4_Untyped_Retype, seL4_Word,
};

impl Allocator {
    /// Get the size (in bits) of the untyped memory required to create an
    /// object of the given size.
    ///
    /// TODO - see vka/object.h, not handling all cases yet (feature gating for
    /// RT/etc)
    /// TODO - move this once vka_object/vka works
    pub fn vka_get_object_size(&self, obj_type: seL4_Word, obj_size_bits: usize) -> usize {
        match obj_type {
            api_object_seL4_UntypedObject => obj_size_bits as _,
            api_object_seL4_TCBObject => seL4_TCBBits as _,
            api_object_seL4_EndpointObject => seL4_EndpointBits as _,
            api_object_seL4_NotificationObject => seL4_NotificationBits as _,
            api_object_seL4_CapTableObject => (seL4_SlotBits as usize + obj_size_bits),
            //seL4_KernelImageObject => seL4_KernelImageBits,
            _ => panic!("vka_arch_get_object_size() not implemented"),
        }
    }

    pub fn vka_cspace_alloc(&mut self) -> Option<seL4_CPtr> {
        self.alloc_cslot()
    }

    pub fn vka_cspace_free(&mut self, slot: seL4_CPtr) {
        self.free_cslot(slot)
    }

    pub fn vka_cspace_make_path(&self, slot: seL4_CPtr, path: &mut CSpacePath) {
        path.cap_ptr = slot;
        path.cap_depth = 32;
        path.root = self.root_cnode;
        path.dest = self.root_cnode;
        path.dest_depth = self.root_cnode_depth;
        path.offset = slot;
        path.window = 1;
    }

    // TODO - result
    pub fn vka_utspace_alloc(
        &mut self,
        dest: &CSpacePath,
        item_type: seL4_Word,
        size_bits: seL4_Word,
    ) -> Option<seL4_CPtr> {
        let ut_size_bits = self.vka_get_object_size(item_type, size_bits as _);

        // allocate untyped memory the size we want
        let untyped_memory = self.alloc_untyped(ut_size_bits)?;

        let err = unsafe {
            seL4_Untyped_Retype(
                untyped_memory,
                item_type,
                size_bits,
                seL4_CapInitThreadCNode,
                self.root_cnode,
                self.root_cnode_depth,
                dest.cap_ptr,
                1,
            )
        };

        if err == 0 {
            Some(untyped_memory)
        } else {
            None
        }
    }
}
