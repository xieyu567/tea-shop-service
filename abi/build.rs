use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/order.proto"], &["protos"])
        .unwrap();

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/cart.proto"], &["protos"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
