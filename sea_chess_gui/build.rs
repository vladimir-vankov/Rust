fn main() {
    // Specify the directory where the library is located
    println!("cargo:rustc-link-search=native=/mnt/E6668E40668E118B/vlado/Rust/snake/libs/");
    // Link to the library
    println!("cargo:rustc-link-lib=SDL2-2.0");
}