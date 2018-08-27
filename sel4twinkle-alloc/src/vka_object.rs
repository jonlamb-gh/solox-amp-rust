/// https://github.com/seL4/seL4_libs/blob/master/libsel4vka/include/vka/object.h
use sel4_sys::{seL4_CNode, seL4_CPtr, seL4_Word};

#[derive(Clone, Debug)]
pub struct VkaObject {
    cap_ptr: seL4_CPtr,
    ut: seL4_Word,
    item_type: seL4_Word,
    size_bits: seL4_Word,
}
