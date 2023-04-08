// SPDX-License-Identifier: GPL-2.0
//! Rust scull module.

use kernel::{self, file, prelude::*, str};

module! {
    type: Scull,
    name: "Scull",
    author: "PAblo Alessandro Santos Hugen",
    description: "Rust scull module",
    license: "GPL",
}

struct Scull;

impl kernel::Module for Scull {
    fn init(_name: &'static str::CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, world! from scull.rs");
        Ok(Scull)
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
}
