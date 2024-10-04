use std::{
    env, fs,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    if let (Some(x), Some(y)) = (args.nth(1), args.nth(0)) {
        let mut buf = Vec::new();
        fs::File::open(x)?.read_to_end(&mut buf)?;
        fs::File::options()
            .write(true)
            .open(y)?
            .write_vectored(&[io::IoSlice::new(&buf[..])])?;
    }

    Ok(())
}
