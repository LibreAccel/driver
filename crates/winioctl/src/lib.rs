// TODO
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms
)]
#![deny(clippy::pedantic)] // clippy::all, clippy::nursery,

mod error;
mod ioctl;

pub use winapi::um::ioapiset::DeviceIoControl;

pub use crate::{
  error::Error,
  ioctl::{ControlCode, DeviceType, RequiredAccess, TransferMethod},
};
