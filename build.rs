fn main() {
    println!("cargo:rerun-if-changed=./go");
    println!("cargo:rerun-if-changed=build.rs");

    std::env::set_current_dir("./go").expect("should be able to find go directory");
    std::process::Command::new("go")
        .args([
            "build",
            "-buildmode=c-shared",
            "-o",
            "../build/librustnats.so",
            "main.go",
        ])
        .output()
        .expect("should be able to build go library");
    std::env::set_current_dir("../").expect("should be able to find go directory");
    let path = std::env::current_dir()
        .expect("should get current dir")
        .join("build");
    let lib = "rustnats";

    println!("cargo:rustc-link-lib=dylib={}", lib);
    println!("cargo:rustc-link-search=native={}", path.to_string_lossy());
}
