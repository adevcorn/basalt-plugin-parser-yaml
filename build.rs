fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") { return; }
    println!("cargo:rustc-link-arg=--whole-archive");
    println!("cargo:rustc-link-arg=-lparser-scanner");
    println!("cargo:rustc-link-arg=--no-whole-archive");
}