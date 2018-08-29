use super::{Allocator, Error};
use sel4_sys::*;

impl Allocator {
    pub fn io_map(
        &mut self,
        paddr: seL4_Word,
        num_pages: usize,
        size_bits: usize,
    ) -> Result<seL4_Word, Error> {
        assert_eq!(size_bits, seL4_PageBits as usize);

        let vaddr = self.vspace_new_pages_at(
            Some(paddr),
            num_pages,
            size_bits,
            unsafe { seL4_CapRights_new(1, 1, 1) },
            // no attributes for memory mapped devices
            0,
            true,
            None,
        )?;

        Ok(vaddr)
    }
}
