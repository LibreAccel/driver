#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]

use std::path::PathBuf;

use thiserror::Error;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error("cannot find the directory")]
  DirectoryNotFound,
}

pub enum DirectoryType {
  Include,
  Library,
}

/// Retrieves the path to the Windows Kits directory. The default should be
/// `C:\Program Files (x86)\Windows Kits\10`.
///
/// # Errors
/// if a key cannot be found.
pub fn get_windows_kits_dir() -> Result<PathBuf, Error> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
  let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

  Ok(dir.into())
}

/// Retrieves the path to the kernel mode libraries. The path may look something
/// like: `C:\Program Files (x86)\Windows Kits\10\lib\10.0.18362.0\km`.
///
/// # Errors
/// if a directory cannot be found.
pub fn get_km_dir(dir_type: &DirectoryType) -> Result<PathBuf, Error> {
  // We first append lib to the path and read the directory.
  let dir = get_windows_kits_dir()?
    .join(match dir_type {
      DirectoryType::Include => "Include",
      DirectoryType::Library => "Lib",
    })
    .read_dir()?;

  // In the lib directory we may have one or more directories named after the
  // version of Windows, we will be looking for the highest version number.
  let dir = dir
    .filter_map(std::result::Result::ok)
    .map(|dir| dir.path())
    .filter(|dir| {
      dir
        .components()
        .last()
        .and_then(|c| c.as_os_str().to_str())
        .map_or(false, |c| c.starts_with("10.") && dir.join("km").is_dir())
    })
    .max()
    .ok_or(Error::DirectoryNotFound)?;

  // Finally, append km to the path to get the path to the kernel mode libraries.
  Ok(dir.join("km"))
}

/// # Panics
/// if the `target` is currently not supported.
///
/// # Errors
/// - if the kernel libraries cannot be found
/// - if the `TARGET` environment variable is not present
/// - if the link path cannot be converted to a &str
pub fn build() -> Result<(), Error> {
  // Get the path to the kernel libraries.
  let dir = get_km_dir(&DirectoryType::Library).unwrap();

  // Append the architecture based on our target.
  let target = std::env::var("TARGET").unwrap();

  let arch = if target.contains("x86_64") {
    "x64"
  } else if target.contains("i686") {
    "x86"
  } else {
    panic!("The target {} is currently not supported.", target);
  };

  let dir = dir.join(arch);

  // Specify the link path.
  println!("cargo:rustc-link-search=native={}", dir.to_str().unwrap());

  // Ensure the right linker flags are passed for building a driver.
  println!("cargo:rustc-link-arg=/NODEFAULTLIB");
  println!("cargo:rustc-link-arg=/SUBSYSTEM:NATIVE");
  println!("cargo:rustc-link-arg=/DRIVER");
  println!("cargo:rustc-link-arg=/DYNAMICBASE");
  println!("cargo:rustc-link-arg=/MANIFEST:NO");
  println!("cargo:rustc-link-arg=/ENTRY:driver_entry");
  println!("cargo:rustc-link-arg=/MERGE:.edata=.rdata");
  println!("cargo:rustc-link-arg=/MERGE:.rustc=.data");
  println!("cargo:rustc-link-arg=/INTEGRITYCHECK");

  Ok(())
}
