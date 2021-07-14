pub extern crate anyhow;
pub extern crate mono_sys as sys;

pub mod assembly;
pub mod class;
pub mod config;
pub mod domain;
pub mod obj;

pub use assembly::{Assembly, Image};
pub use class::{Method, MethodSignature, MonoClass, MonoClassField};
pub use domain::Domain;
pub use obj::*;

fn osstr_to_cstring(osstr: &std::ffi::OsStr) -> anyhow::Result<std::ffi::CString> {
    use std::ffi::CString;

    if cfg!(target_os = "linux") {
        use std::os::unix::ffi::OsStrExt;
        let str = String::from_utf8(osstr.as_bytes().to_vec())?;
        Ok(CString::new(str)?)
    } else {
        let str = osstr.to_str().ok_or(anyhow::anyhow!(
            "Failed to turn &OsStr to &str on non linux platform"
        ))?;
        Ok(CString::new(str)?)
    }
}
