extern crate exoquant;
extern crate glob;
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

                // This will get the name of the output file
                let mut output = path.clone();
                // TODO: the output file name must be the same as the input, except for the bin extension
                output.set_extension("bin");


                let indexed_image_data = bmptovga::bmp_to_vga(&image.data);

                let mut file = File::create(&output).unwrap();
                file.write_all(&indexed_image_data).unwrap();

                println!("Done processing: {:?}", &path);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
