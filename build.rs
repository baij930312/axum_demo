use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("src");

    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&["proto/hello.proto3"], &["proto"])
        .unwrap();
}
