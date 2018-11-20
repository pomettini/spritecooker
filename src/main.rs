extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

pub mod binfile;
pub mod bmptovga;
pub mod previewimage;
pub mod vgapalette;

use std::fs::File;
use std::io::prelude::*;

use binfile::*;
use previewimage::*;

use stb_image::image::LoadResult;

use glob::glob;
use std::path::PathBuf;

use exoquant::*;

fn main() {
    // Gets the current directory
    let args: Vec<String> = std::env::args().collect();
    let mut path = PathBuf::from(&args[0]);
    path.pop();

    // Gets only bmp files
    path.push("*.bmp");

    // For each bitmaps does the job
    for entry in glob(path.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match process_image(&path) {
                    Ok(_) => (),
                    Err(_) => continue,
                };
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn process_image(path: &PathBuf) -> Result<(), ()> {
    println!("Begin processing: {:?}", &path);

    let image = match stb_image::image::load(&path) {
        LoadResult::ImageU8(data) => data,
        LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
        LoadResult::Error(string) => panic!(string),
    };

    if image.width != 256 {
        println!("The image width must be exactly 256 px");
    }

    if image.height != 256 {
        println!("The image height must be exactly 256 px");
    }

    // This contains the image in VGA color space
    let indexed_image_data = bmptovga::bmp_to_vga(&image.data, 256);

    // Generates Bin file
    let bin_file = BinFile::new(&path, &indexed_image_data);
    bin_file.write();

    // Generates Preview file
    let mut preview_image = PreviewImage::new(&path, &indexed_image_data);
    preview_image.add_grid();
    preview_image.write();

    println!("Done processing: {:?}", &path);

    Ok(())
}
