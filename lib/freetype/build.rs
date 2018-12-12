fn main() {
    use std::env::var;
    use std::path::Path;

    let dir = var("CARGO_MANIFEST_DIR").unwrap();

    if var("TARGET").map(|target| target == "x86_64-pc-windows-msvc").unwrap_or(false) {
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("x86_64").display());
    } else {
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("i686").display());
    }
    println!("cargo:rustc-link-lib=dylib=freetype");
}