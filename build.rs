extern crate prost_build;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let proto_path = PathBuf::from("src/savant_rs.proto");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out_path = out_dir.join("protocol.rs");
    let module_path = src_dir.join("generated.rs");

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config.enum_attribute(".", "#[allow(clippy::large_enum_variant)]");
    config
        .compile_protos(
            &[proto_path.to_str().unwrap()],
            &[proto_path.parent().unwrap().to_str().unwrap()],
        )
        .expect("Failed to compile protobuf definitions");

    fs::copy(out_path, module_path).unwrap();
}
