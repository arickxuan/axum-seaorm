use std::{env, path::PathBuf};
use std::io::Result;

fn main() {
    //dotenv::dotenv().ok();
    const OUTDIR: &str = "proto";
    let out_dir = PathBuf::from(OUTDIR);
    println!("out_dir {out_dir:#?}");
    // tonic_build::configure()
    //     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
    //     .compile(&["proto/helloworld.proto"], &["/proto"])
    //     .unwrap();

    tonic_build::configure()
        .build_server(true) // 是否编译生成用于服务端的代码
        .build_client(true) // 是否编译生成用于客户端的代码
        .out_dir(OUTDIR)  // 输出的路径，此处指定为项目根目录下的proto目录
        // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
        // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}

fn backmain() -> Result<()> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}