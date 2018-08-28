// NOTE: this is not a proper vspace impl, just a testing area for now

// https://github.com/seL4/seL4_libs/blob/master/libsel4vspace/include/vspace/vspace.h
// https://github.com/seL4/seL4_libs/blob/master/libsel4utils/src/vspace/vspace.c
// https://github.com/seL4/seL4_libs/blob/master/libsel4utils/src/vspace/bootstrap.c
// https://github.com/seL4/seL4_libs/blob/master/libsel4vspace/src/sel4_arch/aarch32/mapping.c

// https://github.com/seL4/seL4_libs/blob/master/libsel4platsupport/src/io.c#L206

use super::{Allocator, Error};
use sel4_sys::*;

impl Allocator {
    pub fn bootstrap_vspace(&mut self, pd_cap: seL4_CPtr) -> Result<(), Error> {
        // set our vspace root page directory
        self.page_directory = pd_cap;

        // TODO - this is leaky
        // create a page table
        //let pt_obj = self.vka_alloc_page_table()?;
        //self.page_table = pt_obj.cptr;

        Ok(())
    }

    /*
    pub fn vspace_new_ipc_buffer(&self) -> Result<seL4_CPtr, Error> {
        void *vaddr = vspace_new_pages(vspace, seL4_AllRights, 1, seL4_PageBits);

        if (vaddr == NULL) {
            return NULL;
        }

        *page = vspace_get_cap(vspace, vaddr);

        return vaddr;
    }
    */

    //pub fn vspace_new_pages_at_vaddr() {
    // for each page
    //   vka alloc frame
    //   map_page
    //   vaddr += size...

    //pub fn vspace_map_pages_at_vaddr()
    //assert_neq!(size_bits, seL4_PageBits);

    //#define KERNEL_RESERVED_START 0xE0000000
    //#define VSPACE_MAP_PAGING_OBJECTS 2
    //#define VSPACE_LEVEL_BITS 10
    //#define VSPACE_NUM_LEVELS 2

    // VSPACE_RESERVE_START = (KERNEL_RESERVED_START - VSPACE_RESERVE_SIZE)
    // data->last_allocated = 0x10000000;

    // seL4_ARM_VMAttributes_seL4_ARM_Default_VMAttributes:

    // TODO - derive clone for CapRights
    pub fn map_page(
        &mut self,
        cap: seL4_CPtr,
        vaddr: seL4_Word,
        _rights: seL4_CapRights,
        cache_attributes: seL4_ARM_VMAttributes,
    ) -> Result<(), Error> {
        let map_err: seL4_Error = unsafe {
            seL4_ARM_Page_Map(
                cap,
                self.page_directory,
                vaddr,
                //rights,
                seL4_CapRights_new(1, 1, 1),
                cache_attributes,
            )
        };

        if map_err != 0 {
            // create a page table
            // TODO - is leaky
            let pt_obj = self.vka_alloc_page_table()?;
            self.page_table = pt_obj.cptr;

            // map the page table
            let err: seL4_Error = unsafe {
                seL4_ARM_PageTable_Map(
                    self.page_table,
                    self.page_directory,
                    vaddr,
                    cache_attributes,
                )
            };

            if err != 0 {
                return Err(Error::Other);
            }

            // map the frame in
            let err: seL4_Error = unsafe {
                seL4_ARM_Page_Map(
                    cap,
                    self.page_directory,
                    vaddr,
                    //rights,
                    seL4_CapRights_new(1, 1, 1),
                    cache_attributes,
                )
            };

            if err != 0 {
                return Err(Error::Other);
            }
        }

        Ok(())
    }
}
