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
    params: {
        nr_devs: usize {
            default: 4,
            permissions: 0o644,
            description: "Number of devices to create",
        },
    },
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

struct Scull {
    _devs: Vec<Pin<Box<miscdev::Registration<Self>>>>,
}

impl kernel::Module for Scull {
    fn init(name: &'static str::CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, world! from {}", name.to_str()?);
        let count = {
            let lock = module.kernel_param_lock();
            *nr_devs.read(&lock)
        };
        let mut devs = Vec::try_with_capacity(count)?;

        for i in 0..count {
            let dev = Arc::try_new(Device {
                number: i,
                contents: Mutex::new(Vec::new()),
            })?;
            let reg = miscdev::Registration::new_pinned(fmt!("scull{}", i), dev)?;
            devs.try_push(reg)?;
        }

        Ok(Scull { _devs: devs })
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
