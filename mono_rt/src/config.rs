use anyhow::Result;
use std::path::Path;

pub fn load_default() {
    unsafe { mono_sys::mono_config_parse(std::ptr::null()) };
}

pub fn load_custom(path: &Path) -> Result<()> {
    let cstr = crate::osstr_to_cstring(path.as_os_str())?;
    unsafe { mono_sys::mono_config_parse(cstr.as_ptr()) };
    Ok(())
}

pub fn set_dirs(lib_dir: &Path, etc_dir: &Path) -> Result<()> {
    let lib_cstr = crate::osstr_to_cstring(lib_dir.as_os_str())?;
    let etc_cstr = crate::osstr_to_cstring(etc_dir.as_os_str())?;
    unsafe { mono_sys::mono_set_dirs(lib_cstr.as_ptr(), etc_cstr.as_ptr()) };
    Ok(())
}
