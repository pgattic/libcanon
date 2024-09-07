use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["book.proto"], &["src/"])?;
    Ok(())
}

