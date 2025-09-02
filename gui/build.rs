fn main() {
    // Specify the directory where the library is located
    println!("cargo:rustc-link-search=native=/mnt/4b65def5-25ef-4bbf-9c9f-86800782edf4/Rust/gui/libs/");
    // Link to the library
    println!("cargo:rustc-link-lib=SDL2");
}