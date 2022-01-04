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

use windows_kernel_sys::{
  base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING},
  ntoskrnl::DbgPrint,
};

#[panic_handler]
const fn panic(_info: &core::panic::PanicInfo<'_>) -> ! { loop {} }

#[lang = "eh_personality"]
const extern "C" fn eh_personality() {}

/// # Safety
/// `unsafe`
#[no_mangle]
pub unsafe extern "system" fn driver_entry(
  driver: &mut DRIVER_OBJECT,
  _: &UNICODE_STRING,
) -> NTSTATUS {
  // DbgPrint("driver_entry()\0".as_ptr() as _);
  DbgPrint("driver_entry()\0".as_ptr().cast());

  driver.DriverUnload = Some(driver_exit);

  STATUS_SUCCESS
}

/// # Safety
/// `unsafe`
#[no_mangle]
pub unsafe extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
  DbgPrint("driver_exit()\0".as_ptr().cast());
}
