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

    fn open(context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("file for device {} opened", context.number);
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
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("file for device {} written", data.number);

        let copy = reader.read_all()?;
        let len = copy.len();
        *data.contents.lock() = copy;
        Ok(len)
    }
}
