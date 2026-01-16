use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use clap::Parser;
use nclr::Nclr;
use std::fs::File;
use std::io;

mod nclr;

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct Action {
    #[arg(short, long)]
    decode: bool,

    #[arg(short, long)]
    encode: bool,
}

#[derive(Debug, Parser)]
struct Args {
    filename: String,

    #[command(flatten)]
    action: Action,

    #[arg(long)]
    ncgr: Option<String>,

    #[arg(long)]
    nclr: Option<String>,

    #[arg(long)]
    nscr: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let palette = if let Some(palette) = args.nclr {
        let mut file = File::open(palette)?;
        let magic = file.read_u32::<LittleEndian>()?;
        assert_eq!(magic, 0x4e434c52);
        Nclr::new(&mut file)?
    } else {
        panic!("Palette missing!");
    };
    println!("{} palettes", palette.palettes.len());

    Ok(())
}
