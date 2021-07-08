pub extern crate mono_rt as mono;

#[no_mangle]
pub extern "C" fn HelloWorldRust() {
    println!("Hello, from rust!");
}

#[no_mangle]
pub extern "C" fn Hello_DoAThing() {
    println!("Hello, from hello, from rust");
}

#[no_mangle]
pub extern "C" fn Hello_PrintMonoString(s: mono::MonoString) {
    println!("rust print: {}", s);
}

#[no_mangle]
pub extern "C" fn Hello_GetString() -> mono::MonoString {
    mono::Domain::get()
        .unwrap()
        .create_string("string from rust")
        .unwrap()
}
