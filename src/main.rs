use clap::Parser;
use common::decompress;
use image::RgbaImage;
use image::imageops::overlay;
use ncer::Ncer;
use ncgr::Ncgr;
use nclr::Nclr;
use nscr::Nscr;
use std::fs::File;
use std::io;
use std::io::Cursor;

mod common;
mod ncer;
mod ncgr;
mod nclr;
mod nscr;

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
struct Action {
    #[arg(short, long)]
    decode: bool,

    #[arg(short, long)]
    encode: bool,
}

#[derive(Parser)]
struct Args {
    filename: String,

    #[command(flatten)]
    action: Action,

    // tileset
    #[arg(long)]
    ncgr: Option<String>,

    // palette
    #[arg(long)]
    nclr: Option<String>,

    // scene/image
    #[arg(long)]
    nscr: Option<String>,

    // animation
    #[arg(long)]
    nanr: Option<String>,

    // cell/frame
    #[arg(long)]
    ncer: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.nscr.is_some() {
        decode_image(args)?;
    } else if args.nanr.is_some() {
        decode_animation(args)?;
    }

    Ok(())
}

fn decode_image(args: Args) -> io::Result<()> {
    let palette = if let Some(palette) = args.nclr {
        let mut file = File::open(palette)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Nclr::new(&mut reader)?
    } else {
        panic!("Palette missing!");
    };

    let tileset = if let Some(tileset) = args.ncgr {
        let mut file = File::open(tileset)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Ncgr::new(&mut reader)?
    } else {
        panic!("Tileset missing!");
    };

    let tilemap = if let Some(tilemap) = args.nscr {
        let mut file = File::open(tilemap)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Nscr::new(&mut reader)?
    } else {
        panic!("Tilemap missing!");
    };

    let mut image = RgbaImage::new(tilemap.width * 8, tilemap.height * 8);

    let mut tiles = tilemap.map.iter();
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let tile = tiles.next().unwrap();

            let mut pixels_data = tileset.tiles[tile.id as usize].iter();
            let palette = &palette.palettes[tile.palette as usize];

            let mut tile_img = RgbaImage::new(8, 8);
            for py in 0..8 {
                for px in 0..8 {
                    let pixel = pixels_data.next().unwrap();
                    let pixel = &palette[*pixel as usize];
                    let colour = [pixel.r, pixel.g, pixel.b, pixel.a];
                    tile_img.put_pixel(px, py, image::Rgba(colour));
                }
            }

            overlay(&mut image, &tile_img, x as i64 * 8, y as i64 * 8);
        }
    }

    let _ = image.save(&args.filename);

    Ok(())
}

fn decode_animation(args: Args) -> io::Result<()> {
    let _palette = if let Some(palette) = args.nclr {
        let mut file = File::open(palette)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Nclr::new(&mut reader)?
    } else {
        panic!("Palette missing!");
    };

    let _tileset = if let Some(tileset) = args.ncgr {
        let mut file = File::open(tileset)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Ncgr::new(&mut reader)?
    } else {
        panic!("Tileset missing!");
    };

    let _cells = if let Some(cells) = args.ncer {
        let mut file = File::open(cells)?;
        let buffer = decompress(&mut file)?;
        let mut reader = Cursor::new(buffer);
        Ncer::new(&mut reader)?
    } else {
        panic!("cells missing!");
    };

    Ok(())
}
