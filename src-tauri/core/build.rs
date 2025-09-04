use std::{env, path::PathBuf, process::Command};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let gen_dir = PathBuf::from(&crate_dir).join("../gen");
    std::fs::create_dir_all(&gen_dir).unwrap();
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(gen_dir.join("spacebar_core.h"));

    if env::var_os("CARGO_FEATURE_WITH_GO").is_some() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let lib_path = out_dir.join("libgobridge.a");
        let go_dir = PathBuf::from(&crate_dir).join("../go");
        let status = Command::new("go")
            .current_dir(&go_dir)
            .args(["build", "-buildmode=c-archive", "-o"])
            .arg(&lib_path)
            .status()
            .expect("failed to build go library");
        if !status.success() {
            panic!("go build failed");
        }
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=gobridge");
        println!(
            "cargo:rerun-if-changed={}",
            go_dir.join("bridge.go").display()
        );
    }
}
