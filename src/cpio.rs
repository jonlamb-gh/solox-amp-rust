use core::str;
/// This is just a bare minimum utility.
///
/// Only supports the first file, doesn't look at the checksum.
///
/// Pulling most of this from:
/// - https://github.com/jcreekmore/cpio-rs
/// - https://github.com/seL4/util_libs/blob/master/libcpio/include/cpio/cpio.h
use rlibc;

const HEADER_LEN: usize = 110;

const MAGIC_NUMBER: &[u8] = b"070701";

//const TRAILER_NAME: &str = "TRAILER!!!";

#[repr(C, packed)]
pub struct RawHeader {
    magic: [u8; 6],
    inode: [u8; 8],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    nlink: [u8; 8],
    mtime: [u8; 8],
    file_size: [u8; 8],
    dev_major: [u8; 8],
    dev_minor: [u8; 8],
    rdev_major: [u8; 8],
    rdev_minor: [u8; 8],
    file_name_size: [u8; 8],
    checksum: [u8; 8],
}

#[derive(Debug)]
pub struct FileEntry {
    name: [u8; 16],
    size: usize,
    data_ptr: *const u8,
}

#[derive(Debug)]
pub struct Reader {
    archive_size: usize,
    base_ptr: *const u8,
}

impl Reader {
    pub fn new(base_ptr: *const u8, archive_size: usize) -> Reader {
        // TODO - do something safe
        unsafe {
            let magic_diff = rlibc::memcmp(base_ptr, MAGIC_NUMBER.as_ptr(), MAGIC_NUMBER.len());

            assert!(
                magic_diff == 0,
                "CPIO archive is missing magic header bytes"
            );
        }

        Reader {
            archive_size,
            base_ptr,
        }
    }

    pub fn raw_header(&self) -> *const RawHeader {
        self.base_ptr as *const RawHeader
    }

    pub fn parse_entry(&self) -> FileEntry {
        unsafe {
            let hdr = self.raw_header();

            let file_size = read_hex8_u32(&(*hdr).file_size) as usize;

            // includes NULL terminating byte
            let file_name_size = read_hex8_u32(&(*hdr).file_name_size) as usize;

            //let mut file_name: [char; 16] = ['\0'; 16];
            let mut file_name: [u8; 16] = [0; 16];
            assert!(file_name_size <= file_name.len());

            for byte in 0..file_name.len() {
                file_name[byte] = *self.base_ptr.offset((HEADER_LEN + byte) as isize);
            }

            /*
             * TODO fix this
            let _ = rlibc::memcpy(
                entry.name.as_mut_ptr() as *mut u8,
                self.base_ptr.offset(HEADER_LEN as isize),
                file_name_size);
            */

            // align-up to 4 bytes
            let entry_data_index: usize = ((HEADER_LEN + file_name_size) + 4 - 1) & (!(4 - 1));

            let cpio_file = FileEntry {
                name: file_name,
                size: file_size,
                data_ptr: self.base_ptr.offset(entry_data_index as isize),
            };

            cpio_file
        }
    }
}

impl FileEntry {
    pub fn file_name(&self) -> &str {
        str::from_utf8(&self.name[0..15]).unwrap()
    }

    pub fn file_size(&self) -> usize {
        self.size
    }

    pub fn data_ptr(&self) -> *const u8 {
        self.data_ptr
    }
}

// TODO - handle error, use from_str_radix()
fn read_hex8_u32(s: &[u8; 8]) -> u32 {
    //u32::from_str_radix(hex_string.as_ptr() as &str, 16).unwrap()
    let mut r: u32 = 0;

    for i in 0..s.len() {
        r *= 16;
        if s[i] >= '0' as u8 && s[i] <= '9' as u8 {
            r += (s[i] - '0' as u8) as u32;
        } else if s[i] >= 'a' as u8 && s[i] <= 'f' as u8 {
            r += (s[i] - 'a' as u8 + 10) as u32;
        } else if s[i] >= 'A' as u8 && s[i] <= 'F' as u8 {
            r += (s[i] - 'A' as u8 + 10) as u32;
        } else {
            return r;
        }
    }

    return r;
}
