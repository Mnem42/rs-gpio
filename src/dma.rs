use std::{fs::OpenOptions, fs::File, path::PathBuf, ptr};
use memmap2::MmapMut;

const DMA_OFFSET: isize = 0x7e007000;

pub unsafe fn test() -> u8{
    let path: PathBuf = "/dev/mem".into();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Failed to open /dev/mem");

    unsafe { 
        let mmap = MmapMut::map_mut(&file).expect("Failed to mmap");
        mmap.as_ptr().offset(DMA_OFFSET).read()
    }
}