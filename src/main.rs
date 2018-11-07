extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

pub mod bmptovga;
pub mod vgapalette;

use std::fs::File;
use std::io::prelude::*;

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
                println!("Begin processing: {:?}", &path);

                let image = match stb_image::image::load(&path) {
                    LoadResult::ImageU8(data) => data,
                    LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
                    LoadResult::Error(string) => panic!(string),
                };

                if image.width != 256 {
                    println!("The image width must be exactly 256 px");
                    continue;
                }

                if image.height != 256 {
                    println!("The image height must be exactly 256 px");
                    continue;
                }

                // Generates the bin path
                let mut output = path.clone();
                output.set_extension("bin");

                // Generates the preview image path
                let mut preview_image_path = path.clone();
                preview_image_path.set_extension("png");

                // This contains the image in VGA color space
                let indexed_image_data = bmptovga::bmp_to_vga(&image.data, 256);

                // Writes the bin file
                let mut file = File::create(&output).unwrap();
                file.write_all(&indexed_image_data).unwrap();

                // Adds a grid to the preview
                let mut color_output = bmptovga::vga_to_bmp(&indexed_image_data);
                bmptovga::add_grid(&mut color_output);
                add_offsets(&mut color_output);

                // Saves the preview image
                image::save_buffer(preview_image_path, &color_output, 256, 256, image::RGB(8))
                    .unwrap();

                println!("Done processing: {:?}", &path);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

// Please don't look at this code, it's a mess, I know. Sorry :(
pub fn add_offsets(image: &mut Vec<u8>) {
    let args: Vec<String> = std::env::args().collect();
    let mut path = PathBuf::from(&args[0]);
    path.pop();
    path.push("impostor_offsets.png");

    if !path.exists() {
        println!("Cannot find offsets image");
        return;
    }

    let offsets = match stb_image::image::load(&path) {
        LoadResult::ImageU8(data) => data,
        LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
        LoadResult::Error(string) => panic!(string),
    };

    let mut origin_counter = 0;
    let mut destination_counter = 0;

    for i in (0..65536) {
        if offsets.data[origin_counter + 3] != 0 {
            image[destination_counter + 0] = offsets.data[origin_counter + 0];
            image[destination_counter + 1] = offsets.data[origin_counter + 1];
            image[destination_counter + 2] = offsets.data[origin_counter + 2];
        }
        origin_counter += 4;
        destination_counter += 3;
    }
}
