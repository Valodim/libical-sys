extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn print_library(lib: &pkg_config::Library) {
    for p in &lib.include_paths {
        println!("cargo:include={}", p.display());
    }

    for p in &lib.frameworks {
        println!("cargo:rustc-link-lib=framework={}", p);
    }

    for p in &lib.framework_paths {
        println!("cargo:rustc-link-search=framework={}", p.display());
    }

    for p in &lib.libs {
        println!("cargo:rustc-link-lib=dylib={}", p);
    }

    for p in &lib.link_paths {
        println!("cargo:rustc-link-search=native={}", p.display());
    }
}

fn main() {
    let libical = pkg_config::Config::new().atleast_version("3.0.0").probe("libical").unwrap();

    print_library(&libical);

    let mut builder = bindgen::Builder::default()
        .header("bindgen-wrapper.h");

    for p in libical.include_paths {
        builder = builder.clang_arg(format!("-I{}", p.display()));
    }

    builder = builder.whitelist_type("ical.+")
        .whitelist_var("ical.+")
        .whitelist_function("ical.+");

    let bindings = builder.generate().unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
