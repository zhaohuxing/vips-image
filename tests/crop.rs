use std::fs::File;
use std::io::prelude::*;
use vips_image::crop;

#[test]
fn test_crop() {
    let images = vec!["test.jpeg", "test.png", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/crop_{}", i)).unwrap();
        let result = crop(&buffer, 100, 100, 0);
        file.write(&result);
    }
}
