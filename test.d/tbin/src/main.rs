extern crate tlib as lib;

use lib::mono;
use std::{path::Path, ptr::null_mut};

fn main() {
    mono::config::load_default();

    let domain = mono::Domain::new("test_domain").unwrap();

    domain
        .add_internal_call0("App.Hello::DoAThing", lib::Hello_DoAThing)
        .unwrap();
    domain
        .add_internal_call1("App.Hello::PrintString", lib::Hello_PrintMonoString)
        .unwrap();
    domain
        .add_internal_call0("App.Hello::GetString", lib::Hello_GetString)
        .unwrap();

    let assembly = domain.open_assembly(Path::new("Program.exe")).unwrap();

    let image = assembly.get_image();
    let class = image.get_class("App", "Program").unwrap();

    let csharp_method_call_from_rust = class.get_method_from_name("CallFromRust", 0).unwrap();
    unsafe { csharp_method_call_from_rust.invoke(None, null_mut(), null_mut()) };

    let csharp_method_print_num = class.get_method_from_name("PrintNumber", 2).unwrap();
    unsafe {
        let args = [
            alloc_to_ptr(36_u32) as *const _,
            alloc_to_ptr(42_u32) as *const _,
        ]
        .as_ptr() as *mut *mut _;
        csharp_method_print_num.invoke(None, args, null_mut());
    }

    println!();

    let person_class = image.get_class("App", "Person").unwrap();
    let person_method_set_name = person_class.get_method_from_name("SetName", 1).unwrap();
    let person_constructor = person_class.get_method_from_name(".ctor", 1).unwrap();
    let person_method_greet = person_class.get_method_from_name("Greet", 0).unwrap();

    let person_obj = domain.create_object(&person_class);

    unsafe {
        // person_obj.init()
        let args = [domain.create_string("RustPerson").unwrap().into_raw() as *const _].as_ptr()
            as *mut *mut _;
        person_constructor.invoke(Some(&person_obj), args, null_mut());

        // greet
        person_method_greet.invoke(Some(&person_obj), null_mut(), null_mut());

        let args2 = [domain.create_string("RustPerson2").unwrap().into_raw() as *const _].as_ptr()
            as *mut *mut _;
        person_method_set_name.invoke(Some(&person_obj), args2, null_mut());

        // greet
        person_method_greet.invoke(Some(&person_obj), null_mut(), null_mut());
    }

    println!();

    let ret = assembly.exec();
    println!("\nret: {}", ret);
}

fn alloc_to_ptr<T>(t: T) -> *const T {
    Box::into_raw(Box::new(t))
}
