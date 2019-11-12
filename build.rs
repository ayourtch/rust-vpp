extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I../vpp/build-root/install-vpp_debug-native/vpp/include")
        .no_convert_floats()
        .blacklist_type("u128")
        .blacklist_type("i128")
        .blacklist_type("vnet_sw_interface_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");

    bindings
        .write_to_file(&out_file_name)
        .expect("Couldn't write bindings!");

    Command::new("rustfmt")
        .args(&[out_file_name.to_str().unwrap()])
        .status()
        .unwrap();
}
