extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

use stb_image::image::LoadResult;
use std::path::PathBuf;

#[derive(Default)]
pub struct Spritesheet {
    pub width: usize,
    pub height: usize,
    pub imagebuf: Vec<u8>,
}

impl Spritesheet {
    pub fn new() -> Spritesheet {
        Spritesheet {
            width: 0,
            height: 0,
            imagebuf: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &PathBuf) {
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

        self.imagebuf = image.data;
        self.width = image.width;
        self.height = image.height;
    }
}
