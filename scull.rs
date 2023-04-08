// SPDX-License-Identifier: GPL-2.0
//! Rust scull module.

use kernel::{
    self, file, miscdev,
    prelude::*,
    str,
    sync::{smutex::Mutex, Arc, ArcBorrow},
};

module! {
    type: Scull,
    name: "Scull",
    author: "PAblo Alessandro Santos Hugen",
    description: "Rust scull module",
    license: "GPL",
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

struct Scull {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

impl kernel::Module for Scull {
    fn init(_name: &'static str::CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, world! from scull.rs");
        let dev = Arc::try_new(Device {
            number: 0,
            contents: Mutex::new(Vec::new()),
        })?;
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), dev)?;

        Ok(Scull { _dev: reg })
    }
}

impl Drop for Scull {
    fn drop(&mut self) {
        pr_info!("Goodbye, world! from scull.rs");
    }
}

#[vtable]
impl file::Operations for Scull {
    type Data = Arc<Device>;
    type OpenData = Arc<Device>;

    fn open(context: &Self::OpenData, file: &file::File) -> Result<Self::Data> {
        pr_info!("file for device {} opened", context.number);

        if file.flags() & file::flags::O_ACCMODE == file::flags::O_RDONLY {
            context.contents.lock().clear();
        }

        Ok(context.clone())
    }

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &file::File,
        writer: &mut impl kernel::io_buffer::IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("file for device {} read", data.number);

        let offset = offset.try_into()?;
        let contents = data.contents.lock();
        let len = core::cmp::min(writer.len(), contents.len().saturating_sub(offset));
        writer.write_slice(&contents[offset..][..len])?;
        Ok(len)
    }

    fn write(
        data: ArcBorrow<'_, Device>,
        _file: &file::File,
        reader: &mut impl kernel::io_buffer::IoBufferReader,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("file for device {} written", data.number);
        let offset = offset.try_into()?;
        let len = reader.len();
        let new_len = len.checked_add(offset).ok_or(EINVAL)?;
        let mut vec = data.contents.lock();

        if new_len > vec.len() {
            vec.try_resize(new_len, 0)?;
        }

        reader.read_slice(&mut vec[offset..][..len])?;

        Ok(len)
    }
}
