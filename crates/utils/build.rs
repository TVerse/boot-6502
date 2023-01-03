use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    vasm();
}

fn vasm() {
    let archive_path = PathBuf::from("../../vasm.tar.gz")
        .canonicalize()
        .expect("cannot canonicalize path");

    println!("cargo:rerun-if-changed={}", archive_path.to_str().unwrap());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if !Command::new("tar")
        .arg("-xvf")
        .arg(&archive_path)
        .arg("-C")
        .arg(&out_dir)
        .output()
        .expect("could not spawn `tar`")
        .status
        .success()
    {
        panic!("Could not untar vasm")
    }

    if !Command::new("make")
        .current_dir(out_dir.join("vasm"))
        .arg("CPU=6502")
        .arg("SYNTAX=oldstyle")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("Could not make vasm")
    }

    std::fs::rename(
        out_dir.join("vasm").join("vasm6502_oldstyle"),
        out_dir.join("vasm6502_oldstyle"),
    )
    .unwrap()
}
