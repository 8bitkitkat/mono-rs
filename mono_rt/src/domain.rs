use crate::{osstr_to_cstring, Assembly, MonoArray, MonoClass, MonoObject, MonoString};
use anyhow::{anyhow, Result};
use std::ffi::{c_void, CString};
use std::path::Path;
use std::ptr::NonNull;

static mut DOMAIN_CREATED: bool = false;
static mut DOMAIN_DROPPED: bool = false;

#[non_exhaustive]
pub struct Domain {
    pub(crate) raw: NonNull<mono_sys::MonoDomain>,
}

impl Drop for Domain {
    fn drop(&mut self) {
        unsafe {
            if !DOMAIN_DROPPED {
                DOMAIN_DROPPED = true;
                mono_sys::mono_jit_cleanup(self.raw.as_ptr());
            } else {
                panic!("Cannot drop a domain twice")
            }
        }
    }
}

impl Domain {
    pub fn new(name: &str) -> Result<Self> {
        unsafe {
            if DOMAIN_CREATED || DOMAIN_DROPPED {
                panic!("Cannot create a domain twice");
            }
            DOMAIN_CREATED = true;
        }

        let name_cstr = CString::new(name)?;

        let ptr = unsafe { mono_sys::mono_jit_init(name_cstr.as_ptr()) };
        let raw = NonNull::new(ptr).expect("mono_jit_init returned null");
        Ok(Self { raw })
    }

    pub fn new_with_version(name: &str, version: &str) -> Result<Self> {
        unsafe {
            if DOMAIN_CREATED {
                panic!("Cannot create a domain twice");
            }
            DOMAIN_CREATED = true;
        }

        let name_cstr = CString::new(name)?;
        let version_cstr = CString::new(version)?;

        let ptr =
            unsafe { mono_sys::mono_jit_init_version(name_cstr.as_ptr(), version_cstr.as_ptr()) };
        let raw = NonNull::new(ptr).ok_or(anyhow!("mono_jit_init_version returned null"))?;
        Ok(Self { raw })
    }

    /// The domain needs to exist for this to work, if the domain has been dropped,
    /// or not created in the first place this will return None
    pub fn get<'d>() -> Option<&'d mut Domain> {
        if unsafe { !DOMAIN_CREATED || DOMAIN_DROPPED } {
            None
        } else {
            let ptr = unsafe { mono_sys::mono_domain_get() };
            let x = Box::leak(Box::new(Self {
                raw: NonNull::new(ptr)?,
            }));
            Some(x)
        }
    }

    // Although it does not use the domain to will fail without a domain loaded
    unsafe fn add_internal_call_raw(&self, path: &str, f: *const c_void) -> Result<()> {
        let cstr = CString::new(path)?;
        mono_sys::mono_add_internal_call(cstr.as_ptr(), f);
        Ok(())
    }

    // pub fn add_internal_call(&self, path: &str, f: &dyn FunctionToPtr) -> Result<()> {
    //     unsafe { self.add_internal_call_raw(path, f.to_raw_ptr()) }
    // }
    // pub fn add_internal_call<T: FunctionToPtr>(&self, path: &str, f: T) -> Result<()> {
    //     unsafe { self.add_internal_call_raw(path, f.to_raw_ptr()) }
    // }

    pub fn add_internal_call0<Ret>(&self, path: &str, f: extern "C" fn() -> Ret) -> Result<()> {
        unsafe { self.add_internal_call_raw(path, f as *const _) }
    }
    pub fn add_internal_call1<Ret, A>(&self, path: &str, f: extern "C" fn(A) -> Ret) -> Result<()> {
        unsafe { self.add_internal_call_raw(path, f as *const _) }
    }
    pub fn add_internal_call2<Ret, A, B>(
        &self,
        path: &str,
        f: extern "C" fn(A, B) -> Ret,
    ) -> Result<()> {
        unsafe { self.add_internal_call_raw(path, f as *const _) }
    }
    pub fn add_internal_call3<Ret, A, B, C>(
        &self,
        path: &str,
        f: extern "C" fn(A, B, C) -> Ret,
    ) -> Result<()> {
        unsafe { self.add_internal_call_raw(path, f as *const _) }
    }

    pub fn open_assembly(&self, path: &Path) -> Result<Assembly> {
        let path_cstr = osstr_to_cstring(path.as_os_str())?;

        let ptr =
            unsafe { mono_sys::mono_domain_assembly_open(self.raw.as_ptr(), path_cstr.as_ptr()) };

        let raw = NonNull::new(ptr).ok_or(anyhow!("mono_domain_assembly_open returned null"))?;

        Ok(Assembly {
            raw,
            path_cstr: path_cstr.clone(),
        })
    }

    pub fn create_object(&self, class: &MonoClass) -> MonoObject {
        let ptr = unsafe { sys::mono_object_new(self.raw.as_ptr(), class.raw.as_ptr()) };
        MonoObject::new(ptr)
    }

    pub fn create_array(&self, eclass: &MonoClass, n: usize) -> MonoArray {
        let ptr = unsafe { sys::mono_array_new(self.raw.as_ptr(), eclass.raw.as_ptr(), n) };
        MonoArray::new(ptr)
    }

    pub fn create_string(&self, str: &str) -> Result<MonoString> {
        let cstr = CString::new(str)?;
        let ptr = unsafe { mono_sys::mono_string_new(self.raw.as_ptr(), cstr.as_ptr()) };
        Ok(MonoString::new(ptr))
    }
}

// pub trait FunctionToPtr {
//     fn to_raw_ptr(self) -> *const std::os::raw::c_void;
// }

// macro_rules! function_to_ptr {
//     ($($arg:tt)*) => {
//         impl<Ret, $($arg)*> FunctionToPtr for extern "C" fn($($arg)*) -> Ret {
//             fn to_raw_ptr(self) -> *const std::os::raw::c_void {
//                 self as *const _
//             }
//         }
//     };
// }

// function_to_ptr!();
// function_to_ptr!(A);
// function_to_ptr!(A, B);
// function_to_ptr!(A, B, C);
