extern crate glob;
extern crate spritecooker;

use spritecooker::binfile::*;
use spritecooker::bmpto2bpp::*;
use spritecooker::previewimage::*;
use spritecooker::spritesheet::*;

use glob::glob;

use std::path::PathBuf;

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

    let mut spritesheet = Spritesheet::new();
    spritesheet.load(&path);

    // Convert the image to VGA color space
    let indexed_image_data = bmp_to_2bpp(&spritesheet.imagebuf, spritesheet.width);

    // Generates Bin file
    let bin_file = BinFile::new(&path, &indexed_image_data);
    bin_file.write();

    println!("{:?}", &indexed_image_data);

    // Generates Preview file
    let mut preview_image = PreviewImage::new(
        &path,
        &indexed_image_data,
        spritesheet.width,
        spritesheet.height,
    );
    preview_image.convert_from_2bpp();
    preview_image.write();

    println!("Done processing: {:?}", &path);

    Ok(())
}
