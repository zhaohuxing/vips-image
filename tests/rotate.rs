use std::fs::File;
use std::io::prelude::*;
use vips_image::rotate;

#[test]
fn test_rotate() {
    let images = vec!["test.jpeg", "test.png", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/rotate_90_{}", i)).unwrap();
        let result = rotate(&buffer, 90);
        file.write(&result);
    }
}
