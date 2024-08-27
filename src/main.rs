use std::{env::args, fs, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().skip(1).collect::<Vec<String>>();

    if let (Some(x), Some(y)) = (args.first(), args.get(1)) {
        let mut data = Vec::new();
        fs::File::open(x)?.read_to_end(&mut data)?;
        fs::write(y, data)?;
    };

    Ok(())
}
