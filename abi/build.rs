use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/messages.proto"], &["protos"])
        .unwrap();

    // execute cargo fmt command
    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo: rerun-if-changed=abi/protos/messages.proto");
}
