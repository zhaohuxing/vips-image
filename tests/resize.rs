use std::fs::File;
use std::io::prelude::*;
use vips_image::resize;

#[test]
fn test_resize() {
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

        let mut file = File::create(format!("images/resize_{}", i)).unwrap();
        let result = resize(&buffer, 100, 100);
        file.write(&result);
    }
}
