fn main() {
    let path = "../build";
    let lib = "rustnats";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=dylib={}", lib);
}