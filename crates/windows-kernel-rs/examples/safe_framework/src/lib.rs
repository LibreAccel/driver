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

use windows_kernel_rs::{kernel_module, println, Driver, Error, KernelModule};

struct Module;
impl KernelModule for Module {
  fn init(_: Driver, _: &str) -> Result<Self, Error> {
    println!("Hello, world!");

    Ok(Module)
  }

  fn cleanup(&mut self, _: Driver) {
    println!("Bye bye!");
  }
}

kernel_module!(Module);
