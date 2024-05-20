use std::process::Command;

pub trait BuilderExt {
    fn with_serde(self, path: &[&str]) -> Self;
}

impl BuilderExt for tonic_build::Builder {
    fn with_serde(self, path: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(serde::Serialize, serde::Deserialize)]")
        })
    }
}

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_serde(&[
            "Msg",
            "union",
            "ChatMsg",
            "UserControlMsg",
            "Ping",
            "Pong",
            "LoginRequest",
            "LoginResponse",
            "LogoutResponse",
            "LogoutRequest",
        ])
        .compile(&["protos/messages.proto"], &["protos"])
        .unwrap();

    // execute cargo fmt command
    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo: rerun-if-changed=abi/protos/messages.proto");
}
