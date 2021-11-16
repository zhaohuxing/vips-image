use std::fs::File;
use std::io::prelude::*;
use vips_image::{watermark_image, watermark_text};
use libvips::bindings;
use std::ffi::{CStr, CString};

#[test]
fn test_watermark() {
    let images = vec!["test.jpeg", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/watermark_text_{}", i)).unwrap();
        let result = watermark_text(&buffer, "文字水印", 300, 0.32, "#FF1493");
        file.write(&result);
    }

    let i = "test.jpeg";
    let mut f = File::open(format!("images/{}", i)).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer);

    let mut file = File::create(format!("images/watermark_image_{}", i)).unwrap();
    let result = watermark_image(&buffer, &buffer, 0, 0, 0);
    file.write(&result);
}
