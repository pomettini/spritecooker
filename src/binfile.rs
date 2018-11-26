extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

use bitmap::Bitmap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct BinFile {
    root: PathBuf,
    image: Bitmap,
}

impl BinFile {
    pub fn new(root: &PathBuf, image: &[u8]) -> BinFile {
        BinFile {
            root: root.to_path_buf(),
            image: image.to_vec(),
        }
    }

    fn generate_bin_path(&self, root: &PathBuf) -> PathBuf {
        let mut output = root.clone();
        output.set_extension("bin");
        output
    }

    fn write_bin_file(&self, path: &PathBuf, image: &[u8]) {
        let mut file = File::create(&path).unwrap();
        file.write_all(&image).unwrap();
    }

    // TODO: Must check errors
    pub fn write(&self) {
        let path = BinFile::generate_bin_path(self, &self.root);
        BinFile::write_bin_file(self, &path, &self.image);
    }
}
