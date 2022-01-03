#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::base::*;

#[link(name = "wrapper_ntoskrnl")]
extern "C" {
  pub fn _ExInitializeFastMutex(mutex: PFAST_MUTEX);
  pub fn _ExAcquirePushLockExclusive(push_lock: PEX_PUSH_LOCK);
  pub fn _ExReleasePushLockExclusive(push_lock: PEX_PUSH_LOCK);
  pub fn _ExAcquirePushLockShared(push_lock: PEX_PUSH_LOCK);
  pub fn _ExReleasePushLockShared(push_lock: PEX_PUSH_LOCK);
  pub fn _IoGetCurrentIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
  pub fn _IoGetNextIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
  pub fn _IoSetCompletionRoutine(
    irp: PIRP,
    completion_routine: PIO_COMPLETION_ROUTINE,
    context: PVOID,
    invoke_on_success: BOOLEAN,
    invoke_on_error: BOOLEAN,
    invoke_on_cancel: BOOLEAN,
  );
  pub fn _IoCompleteRequest(irp: PIRP, priority_boost: CCHAR);
  pub fn _MmGetMdlByteCount(mdl: PMDL) -> ULONG;
  pub fn _MmGetMdlByteOffset(mdl: PMDL) -> ULONG;
  pub fn _MmGetSystemAddressForMdlSafe(mdl: PMDL, priority: ULONG) -> PVOID;
  pub fn _ObDereferenceObject(p: *mut cty::c_void);
  pub fn _ObReferenceObject(p: *mut cty::c_void);
}

pub use self::{
  IoGetCurrentProcess as PsGetCurrentProcess,
  _ExAcquirePushLockExclusive as ExAcquirePushLockExclusive,
  _ExAcquirePushLockShared as ExAcquirePushLockShared,
  _ExInitializeFastMutex as ExInitializeFastMutex,
  _ExReleasePushLockExclusive as ExReleasePushLockExclusive,
  _ExReleasePushLockShared as ExReleasePushLockShared,
  _IoCompleteRequest as IoCompleteRequest,
  _IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation,
  _IoGetNextIrpStackLocation as IoGetNextIrpStackLocation,
  _IoSetCompletionRoutine as IoSetCompletionRoutine,
  _MmGetMdlByteCount as MmGetMdlByteCount,
  _MmGetMdlByteOffset as MmGetMdlByteOffset,
  _MmGetSystemAddressForMdlSafe as MmGetSystemAddressForMdlSafe,
  _ObDereferenceObject as ObDereferenceObject,
  _ObReferenceObject as ObReferenceObject,
};

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));
