extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
};

fn get_maya_home_path() -> String {
    String::from(Path::new(env!("MAYA_HOME")).to_str().unwrap())
}

fn main() {
    let maya_home_path = get_maya_home_path();

    println!("cargo:rustc-link-search=native={}/lib", maya_home_path);
    println!("cargo:rustc-link-lib=Foundation");
    println!("cargo:rustc-link-lib=OpenMaya");
    println!("cargo:rustc-link-lib=OpenMayaUI");
    println!("cargo:rustc-link-lib=OpenMayaAnim");
    println!("cargo:rustc-link-lib=OpenMayaFx");
    println!("cargo:rustc-link-lib=OpenMayaRender");

    println!("cargo:rerun-if-changed=include/wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.hpp")
        .clang_arg(format!("-I{}/include", maya_home_path))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
