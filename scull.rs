// SPDX-License-Identifier: GPL-2.0
//! Rust scull module.

use kernel::{self, file, miscdev, prelude::*, str};

module! {
    type: Scull,
    name: "Scull",
    author: "PAblo Alessandro Santos Hugen",
    description: "Rust scull module",
    license: "GPL",
}

struct Scull {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

impl kernel::Module for Scull {
    fn init(_name: &'static str::CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, world! from scull.rs");
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), ())?;

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
    fn open(_context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("open");
        Ok(())
    }

    fn read(
        _data: <Self::Data as kernel::ForeignOwnable>::Borrowed<'_>,
        _file: &file::File,
        _writer: &mut impl kernel::io_buffer::IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("read");
        Ok(0)
    }
}
