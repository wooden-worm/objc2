use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

use crate::{objc_AssociationPolicy, objc_class, objc_object, OpaqueData, BOOL};

/// A type that represents an instance variable.
#[repr(C)]
pub struct objc_ivar {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// A pointer to the start of a method implementation.
///
/// Remember that this is non-null!
/// Use `Option<IMP>` where nullability is expected. TODO
pub type IMP = unsafe extern "C" fn();

/// Not available on macOS x86.
///
/// Remember that this is non-null!
#[cfg(not(all(target_os = "macos", target_arch = "x86")))]
pub type objc_hook_getClass =
    unsafe extern "C" fn(name: *const c_char, out_cls: *mut *const objc_class) -> BOOL;

/// Not available on macOS x86.
///
/// Remember that this is non-null!
#[cfg(not(all(target_os = "macos", target_arch = "x86")))]
pub type objc_hook_lazyClassNamer = unsafe extern "C" fn(cls: *const objc_class) -> *const c_char;

extern "C" {
    pub fn imp_getBlock(imp: IMP) -> *mut objc_object;
    pub fn imp_implementationWithBlock(block: *mut objc_object) -> IMP;
    pub fn imp_removeBlock(imp: IMP) -> BOOL;

    pub fn ivar_getName(ivar: *const objc_ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const objc_ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const objc_ivar) -> *const c_char;

    pub fn objc_copyClassNamesForImage(
        image: *const c_char,
        out_len: *mut c_uint,
    ) -> *mut *const c_char;
    pub fn objc_copyImageNames(out_len: *mut c_uint) -> *mut *const c_char;

    pub fn objc_enumerationMutation(obj: *mut objc_object);
    pub fn objc_setEnumerationMutationHandler(
        handler: Option<unsafe extern "C" fn(arg1: *mut objc_object)>,
    );

    pub fn objc_getAssociatedObject(
        object: *const objc_object,
        key: *const c_void,
    ) -> *const objc_object;
    pub fn objc_setAssociatedObject(
        object: *mut objc_object,
        key: *const c_void,
        value: *mut objc_object,
        policy: objc_AssociationPolicy,
    );
    pub fn objc_removeAssociatedObjects(object: *mut objc_object);

    pub fn objc_setForwardHandler(fwd: *mut c_void, fwd_stret: *mut c_void);
    pub fn objc_sync_enter(obj: *mut objc_object) -> c_int;
    pub fn objc_sync_exit(obj: *mut objc_object) -> c_int;

    /// Not available on macOS x86.
    ///
    /// Remember that this is non-null!
    #[cfg(not(all(target_os = "macos", target_arch = "x86")))]
    pub fn objc_setHook_getClass(
        new_value: objc_hook_getClass,
        out_old_value: *mut objc_hook_getClass,
    );
    /// Not available on macOS x86.
    ///
    /// Remember that this is non-null!
    #[cfg(not(all(target_os = "macos", target_arch = "x86")))]
    pub fn objc_setHook_lazyClassNamer(
        new_value: objc_hook_lazyClassNamer,
        out_old_value: *mut objc_hook_lazyClassNamer,
    );
}
