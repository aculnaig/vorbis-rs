fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    // Tell cargo to tell rustc to link the system's vorbis
    // shared library.
    println!("cargo:rustc-link-lib=vorbis");
    println!("cargo:rustc-link-lib=vorbisfile");
    println!("cargo:rustc-link-lib=vorbisenc");

    // The binding::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("vorbis_sys.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Add the include path for the vorbis library
        .clang_arg("-I/usr/include/x86_64-linux-gnu/vorbisenc")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
