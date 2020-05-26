extern crate protobuf_codegen_pure;

use protobuf_codegen_pure::Customize;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["src/protos/logic.proto"],
        includes: &["src/protos"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
