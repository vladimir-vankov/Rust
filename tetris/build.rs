fn main() {
    // Specify the directory where the library is located
    println!("Hello from build.rs");
    println!("cargo:rustc-link-search=native=/media/vladimir/E6668E40668E118B/vlado/Rust/tetris/libs");
    // Link to the library
    println!("cargo:rustc-link-lib=SDL2-2.0");
    println!("cargo:rerun-if-changed=build.rs");
}
