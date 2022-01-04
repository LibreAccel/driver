#![no_std]
#![feature(lang_items, const_extern_fn)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]

use winapi::{
  km::wdm::{DbgPrint, DRIVER_OBJECT},
  shared::ntdef::{NTSTATUS, UNICODE_STRING},
};

#[panic_handler]
const fn panic(_info: &core::panic::PanicInfo<'_>) -> ! { loop {} }

#[lang = "eh_personality"]
const extern "C" fn eh_personality() {}

/// # Safety
/// `unsafe`
#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
  unsafe {
    DbgPrint("driver_entry()\0".as_ptr());
  }

  driver.DriverUnload = Some(driver_exit);

  winapi::shared::ntstatus::STATUS_SUCCESS
}

/// # Safety
/// `unsafe`
#[no_mangle]
pub extern "system" fn driver_exit(_driver: &mut DRIVER_OBJECT) {
  unsafe {
    DbgPrint("driver_exit()\0".as_ptr());
  }
}
