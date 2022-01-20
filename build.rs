fn main() {
    #[cfg(target_vendor = "apple")]
    {
        println!("cargo:rustc-link-lib=framework=AppKit");
    }
}
