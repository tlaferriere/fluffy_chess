fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../Cargo.lock");
    println!("cargo:rerun-if-changed=../schema/fluffy_chess.capnp");

    capnpc::CompilerCommand::new()
        .src_prefix("../schema")
        .file("../schema/fluffy_chess.capnp")
        .output_path("proto")
        .run()
        .expect("schema compiler command");
}
