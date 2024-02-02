fn main() {
    let out_dir = std::env::var("NJECT_OUT_DIR")
        .unwrap_or_else(|_| std::env::var("OUT_DIR").unwrap_or_else(|_| String::from("./target")));
    // Expose the out directory to macros.
    println!("cargo:rustc-env=NJECT_OUT_DIR={}", out_dir)
}
