use crate::{Domain, MonoClass};
use anyhow::Result;
use std::ffi::CString;
use std::ptr::NonNull;

#[non_exhaustive]
pub struct Assembly {
    pub(crate) raw: NonNull<mono_sys::MonoAssembly>,
    pub(crate) path_cstr: CString,
}

impl Assembly {
    unsafe fn exec_helper(&self, args: &[CString]) -> i32 {
        let mut v = vec![self.path_cstr.clone().into_raw()];
        v.extend(args.to_vec().into_iter().map(|s| s.into_raw()));

        mono_sys::mono_jit_exec(
            // self.domain.raw.as_ptr(),
            Domain::get().unwrap().raw.as_ptr(),
            self.raw.as_ptr(),
            v.len() as i32,
            v.as_mut_ptr(),
        )
    }

    pub fn as_ptr(&self) -> *mut sys::MonoAssembly {
        self.raw.as_ptr()
    }

    pub fn exec(&self) -> i32 {
        unsafe { self.exec_helper(&[]) }
    }

    pub fn exec_with_args(&self, args: Vec<String>) -> Result<i32> {
        let args = {
            let mut v = Vec::new();
            for arg in args {
                v.push(CString::new(arg)?);
            }
            v
        };

        Ok(unsafe { self.exec_helper(args.as_slice()) })
    }

    pub fn get_image(&self) -> Image {
        let ptr = unsafe { sys::mono_assembly_get_image(self.raw.as_ptr()) };
        let raw = NonNull::new(ptr).unwrap();
        Image { raw }
    }
}

#[non_exhaustive]
#[repr(transparent)]
pub struct Image {
    raw: NonNull<sys::MonoImage>,
}

impl Image {
    pub fn get_class(&self, namespace: &str, name: &str) -> Option<MonoClass> {
        let namespace_cstr = CString::new(namespace).ok()?;
        let name_cstr = CString::new(name).ok()?;
        let ptr = unsafe {
            sys::mono_class_from_name_case(
                self.raw.as_ptr(),
                namespace_cstr.as_ptr(),
                name_cstr.as_ptr(),
            )
        };
        let raw = NonNull::new(ptr)?;
        Some(MonoClass::new(raw))
    }
}
