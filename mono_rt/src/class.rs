use crate::{Domain, MonoObject};
use std::ffi::{c_void, CString};
use std::marker::PhantomData;
use std::ptr::{null_mut, NonNull};

#[non_exhaustive]
#[repr(transparent)]
pub struct MonoClass {
    pub(crate) raw: NonNull<sys::MonoClass>,
}

impl MonoClass {
    pub(crate) fn new(raw: NonNull<sys::MonoClass>) -> Self {
        Self { raw }
    }

    pub fn get_method_from_name(&self, name: &str, param_count: i32) -> Option<Method> {
        let name_cstr = CString::new(name).ok()?;

        let ptr = unsafe {
            sys::mono_class_get_method_from_name(self.raw.as_ptr(), name_cstr.as_ptr(), param_count)
        };
        let raw = NonNull::new(ptr)?;

        Some(Method::new(raw))
    }

    pub fn get_parent(&self) -> Option<MonoClass> {
        let ptr = unsafe { sys::mono_class_get_parent(self.raw.as_ptr()) };
        let raw = NonNull::new(ptr)?;
        Some(MonoClass::new(raw))
    }

    pub fn get_methods(&self) -> Vec<Method> {
        let mut methods = Vec::new();
        unsafe {
            let iter_ptr = Box::into_raw(Box::new(Box::into_raw(Box::new(0)))) as *mut *mut _;
            loop {
                let x = sys::mono_class_get_methods(self.raw.as_ptr(), iter_ptr);
                if x.is_null() {
                    break;
                }
                let raw = NonNull::new_unchecked(x);
                methods.push(Method::new(raw))
            }
        }
        methods
    }
}

#[non_exhaustive]
#[repr(transparent)]
pub struct Method<'d> {
    raw: NonNull<sys::MonoMethod>,
    _m: PhantomData<&'d Domain>,
}

impl PartialEq for Method<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.signature() == other.signature()
    }
}

impl<'d> Method<'d> {
    pub(crate) fn new(raw: NonNull<sys::MonoMethod>) -> Self {
        Self {
            raw,
            _m: PhantomData,
        }
    }

    /// this   - the 'this' ptr <br/>
    /// params - arguments <br/>
    /// exc    - exception information <br/>
    pub unsafe fn invoke(
        &self,
        // this: *mut c_void,
        this: Option<&MonoObject>,
        params: *mut *mut c_void,
        exc: *mut *mut sys::MonoObject,
    ) -> MonoObject {
        let ptr = sys::mono_runtime_invoke(
            self.raw.as_ptr(),
            this.map_or(null_mut(), |s| s.ptr as *mut _),
            params,
            exc,
        );
        MonoObject::new(ptr)
    }

    pub unsafe fn invoke_array(
        &self,
        this: *mut c_void,
        params: *mut sys::MonoArray,
        exc: *mut *mut sys::MonoObject,
    ) -> MonoObject {
        let ptr = sys::mono_runtime_invoke_array(self.raw.as_ptr(), this, params, exc);
        MonoObject::new(ptr)
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let char_ptr = sys::mono_method_get_name(self.raw.as_ptr()) as *mut _;
            let cstr = CString::from_raw(char_ptr);
            cstr.to_str().unwrap().to_string()
        }
    }

    pub fn signature(&self) -> MethodSignature {
        MethodSignature::new(self)
    }
}

#[non_exhaustive]
#[repr(transparent)]
pub struct MethodSignature<'d> {
    raw: NonNull<sys::MonoMethodSignature>,
    _m: PhantomData<&'d Domain>,
}

impl PartialEq for MethodSignature<'_> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::mono_metadata_signature_equal(self.raw.as_ptr(), other.raw.as_ptr()) == 0 }
    }
}

impl MethodSignature<'_> {
    pub(crate) fn new_(raw: NonNull<sys::MonoMethodSignature>) -> Self {
        Self {
            raw,
            _m: PhantomData,
        }
    }

    pub fn new(method: &Method) -> Self {
        let ptr = unsafe { sys::mono_method_signature(method.raw.as_ptr()) };
        let raw = NonNull::new(ptr).unwrap();
        Self::new_(raw)
    }

    pub fn param_count(&self) -> u32 {
        unsafe { sys::mono_signature_get_param_count(self.raw.as_ptr()) }
    }
}
