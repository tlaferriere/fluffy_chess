fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("../schema")
        .output_path("../proto")
        .file("../schema/fluffy_chess.capnp")
        .run()
        .expect("schema compiler command");
}
