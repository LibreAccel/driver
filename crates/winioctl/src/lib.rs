mod error;
mod ioctl;

pub use winapi::um::ioapiset::DeviceIoControl;

pub use crate::{
  error::Error,
  ioctl::{ControlCode, DeviceType, RequiredAccess, TransferMethod},
};
