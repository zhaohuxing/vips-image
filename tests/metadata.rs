use std::fs::File;
use std::io::prelude::*;
use vips_image::get_metadata;

#[test]
fn test_get_metadata() {
    let images = vec![
        "test.jpeg",
        "test.png",
        "test.webp",
        "test.tiff",
        "test.gif",
    ];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);
        get_metadata(&buffer);
    }
}
