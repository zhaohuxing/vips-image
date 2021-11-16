use crate::vips::*;
use libvips::bindings;
use std::convert::TryInto;
use std::ptr::null_mut;

pub fn rotate(buf: &[u8], angle: i32) -> Vec<u8> {
    let _type = vips_image_type(&buf);
    let image = vips_image_new_from_buffer(&buf, "");
    let angle_in = match angle {
        90_i32 => 1,  // Angle 90
        180_i32 => 2, // Angle 180
        270_i32 => 3, // Angle 270
        _ => 0,       // Angle 0
    };
    let inp = vips_rot(image, angle_in);
    return vips_image_write_to_buffer(inp, &format!(".{}", _type));
}

fn vips_rot(inp: *mut bindings::VipsImage, angle: i32) -> *mut bindings::VipsImage {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();
        let ret = bindings::vips_rot(inp, &mut out_out, angle.try_into().unwrap(), NULL);
        return out_out;
    }
}
