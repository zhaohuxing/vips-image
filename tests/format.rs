use std::fs::File;
use std::io::prelude::*;
use vips_image::format;

#[test]
fn test_format() {
    let images = vec!["test.png", "test.webp", "test.tiff"];
    format_type(images, "jpeg");

    let images = vec!["test.jpeg", "test.webp", "test.tiff"];
    format_type(images, "png");

    let images = vec!["test.jpeg", "test.png", "test.tiff"];
    format_type(images, "webp");

    let images = vec!["test.jpeg", "test.png", "test.webp"];
    format_type(images, "tiff")
}

fn format_type(images: Vec<&str>, ftype: &str) {
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let strs: Vec<&str> = i.split(".").collect();
        let mut file = File::create(format!(
            "images/format_{}_from_{}.{}",
            ftype, strs[1], ftype
        ))
        .unwrap();
        let result = format(&buffer, ftype);
        file.write(&result);
    }
}
