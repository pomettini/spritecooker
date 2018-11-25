extern crate spritecooker;
extern crate glob;

use spritecooker::binfile::*;
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
    let indexed_image_data = bmptovga::bmp_to_vga(&spritesheet.imagebuf, spritesheet.width);

    // Generates Bin file
    let bin_file = BinFile::new(&path, &indexed_image_data);
    bin_file.write();

    let mut preview_image = PreviewImage::new(
        &path,
        &indexed_image_data,
        spritesheet.width,
        spritesheet.height,
    );
    preview_image.convert_from_vga();
    preview_image.add_grid(spritesheet.width, 16);
    preview_image.write();

    println!("Done processing: {:?}", &path);

    Ok(())
}
