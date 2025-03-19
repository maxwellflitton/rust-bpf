use std::process::Command;
fn main() {
    println!("cargo:rerun-if-changed=src-ebpf/tracepoint.bpf.rs");
    Command::new("cargo")
        .args(&[
            "+nightly",
            "build",
            "--target",
            "bpf",
            "--release",
            "-p",
            "tracepoint",
        ])
        .status()
        .unwrap();
}
