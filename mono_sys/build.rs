extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // link
    println!("cargo:rustc-link-lib=mono-2.0");
    println!("cargo:rustc-link-search=/usr/include/mono-2.0");
    std::env::set_var(
        "BINDGEN_EXTRA_CLANG_ARGS",
        "-I/usr/include/mono-2.0 -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include",
    );

    // generate
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(std::path::Path::new("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
