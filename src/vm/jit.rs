use libc;

use std::mem;

const PAGE_SIZE: usize = 4096;
const RET_INSTRUCTION: i32 = 0xc3;

use std::ops::{Index, IndexMut};
pub struct ExecMemory {
    contents: *mut u8,
}

impl ExecMemory {
    pub fn new(num_pages: usize) -> Self{
        let contents: *mut u8;

        unsafe {
            let size = PAGE_SIZE * num_pages;
            let mut page: *mut libc::c_void = mem::MaybeUninit::uninit().assume_init();
            libc::posix_memalign(&mut page, PAGE_SIZE, size);
            libc::mprotect(page, size,
                libc::PROT_EXEC
                | libc::PROT_READ
                | libc::PROT_WRITE
            );
            libc::memset(page, RET_INSTRUCTION, size);    // Populate with "ret"

            contents = mem::transmute(page);
        }

        Self { contents }
    }

    pub fn run(&self) -> i64 {
        unsafe { mem::transmute::<*mut u8, fn() -> i64>(self.contents)()  }
    }
}

impl Index<usize> for ExecMemory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
} 

impl IndexMut<usize> for ExecMemory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.contents.offset(index as isize) }
    }
}