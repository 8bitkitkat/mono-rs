use crate::MonoClass;
use std::ffi::c_void;
use std::ptr::NonNull;

#[repr(transparent)]
pub struct MonoObject {
    pub(crate) ptr: *mut sys::MonoObject,
}

impl Clone for MonoObject {
    fn clone(&self) -> Self {
        let ptr = unsafe { sys::mono_object_clone(self.ptr) };
        Self::new(ptr)
    }
}

impl std::fmt::Display for MonoObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            self.to_mono_string(std::ptr::null_mut())
        })
    }
}

impl MonoObject {
    pub(crate) fn new(ptr: *mut sys::MonoObject) -> Self {
        Self { ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *mut sys::MonoObject {
        self.ptr
    }

    pub fn into_raw(self) -> *mut sys::MonoObject {
        self.ptr
    }

    /// # Safety
    /// Only run if the default constructor takes no arguments,
    /// otherwise find the constructor method and run it with custom arguments
    pub unsafe fn init(&self) {
        sys::mono_runtime_object_init(self.ptr);
    }

    pub unsafe fn unbox(&self) -> *mut c_void {
        sys::mono_object_unbox(self.ptr)
    }

    pub fn get_class(&self) -> MonoClass {
        let ptr = unsafe { sys::mono_object_get_class(self.ptr) };
        let raw = NonNull::new(ptr).unwrap();
        MonoClass::new(raw)
    }

    pub fn get_size(&self) -> u32 {
        unsafe { sys::mono_object_get_size(self.ptr) }
    }

    pub unsafe fn to_mono_string(&self, exc: *mut *mut sys::MonoObject) -> MonoString {
        let ptr = sys::mono_object_to_string(self.ptr, exc);
        MonoString::new(ptr)
    }
}

#[repr(transparent)]
pub struct MonoArray {
    pub(crate) ptr: *mut sys::MonoArray,
}

impl MonoArray {
    pub(crate) fn new(ptr: *mut sys::MonoArray) -> Self {
        Self { ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn into_raw(self) -> *mut sys::MonoArray {
        self.ptr
    }

    pub fn length(&self) -> usize {
        unsafe { sys::mono_array_length(self.ptr) }
    }

    pub unsafe fn get(&self, size: i32, index: usize) -> *mut u8 {
        sys::mono_array_addr_with_size(self.ptr, size, index) as *mut u8
    }
}

#[repr(transparent)]
pub struct MonoString {
    pub(crate) ptr: *mut sys::MonoString,
}

impl std::fmt::Display for MonoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len() as usize;
        let str = unsafe {
            let x = sys::mono_string_to_utf8(self.ptr) as *mut u8;
            String::from_raw_parts(x, len, len)
        };
        write!(f, "{}", str)
    }
}

impl MonoString {
    pub(crate) fn new(ptr: *mut sys::MonoString) -> Self {
        Self { ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn into_raw(self) -> *mut sys::MonoString {
        self.ptr
    }

    pub fn intern(&self) {
        unsafe { sys::mono_string_intern(self.ptr) };
    }

    pub fn is_interned(&self) -> bool {
        unsafe {
            let ptr = sys::mono_string_is_interned(self.ptr);
            !ptr.is_null()
        }
    }

    pub fn len(&self) -> i32 {
        unsafe { sys::mono_string_length(self.ptr) }
    }
}

#[repr(transparent)]
pub struct MonoType {
    pub(crate) raw: NonNull<sys::MonoType>,
}

impl MonoType {
    // pub(crate) fn new(ptr: *mut sys::MonoType) -> Self {
    //     Self { ptr }
    // }

    // pub fn as_ptr(&self) -> *mut sys::MonoType {
    //     self.ptr
    // }

    pub fn is_void(&self) -> bool {
        let x = unsafe { sys::mono_type_is_void(self.raw.as_ptr()) };
        x != 0
    }

    /// (size, alignment)
    pub fn size(&self) -> (i32, i32) {
        let mut alignment: i32 = 0;
        let size = unsafe { sys::mono_type_size(self.raw.as_ptr(), (&mut alignment) as *mut i32) };

        (size, alignment)
    }

    pub fn get_class(&self) -> MonoClass {
        unsafe {
            MonoClass::new(NonNull::new(sys::mono_type_get_class(self.raw.as_ptr())).unwrap())
        }
    }
}
