use std::{fs, path::PathBuf, process::Command};

use prost_wkt_build::{FileDescriptorSet, Message};

fn main() {
    let out_dir = PathBuf::from("src/pb");
    if fs::metadata(&out_dir).is_err() {
        fs::create_dir(&out_dir).unwrap();
    }
    let descriptor_file = out_dir.join("descriptors.bin");

    tonic_build::configure()
        .out_dir(&out_dir)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .file_descriptor_set_path(&descriptor_file)
        .compile(&["protos/document_collection.proto"], &["protos"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    let descriptor_bytes = std::fs::read(descriptor_file.clone()).unwrap();

    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out_dir, descriptor);

    Command::new("cargo").args(&["fmt"]).status().unwrap();

    println!("cargo:rerun-if-changed=protos/document_collection.proto");

    fs::remove_file(descriptor_file).unwrap();
    fs::remove_file("src/pb/google.protobuf.rs").unwrap();
}
