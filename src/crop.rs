use crate::vips::*;
use libvips::bindings;
use std::cmp;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;
use std::convert::TryInto;

pub fn crop(buf: &[u8], width: i32, height: i32, gravity: i32) -> Vec<u8> {
    let image = vips_image_new_from_buffer(buf, "");
    let _type = vips_image_type(&buf);
    let out = vips_extract_area(image, width, height, gravity);  
    return vips_image_write_to_buffer(out, &format!(".{}", _type));
}

fn vips_extract_area(
    inp: *mut bindings::VipsImage,
    width: i32,
    height: i32,
    gravity: i32,
) -> *mut bindings::VipsImage {
    unsafe {
        let width_in: i32 = cmp::min((*inp).Xsize, width);
        let height_in: i32 = cmp::min((*inp).Ysize, height);
        let (_left, _top) = calculate_crop((*inp).Xsize, (*inp).Ysize, width, height, gravity);
        let left_in: i32 = cmp::max(_left, 0);
        let top_in: i32 = cmp::max(_top, 0);
        let mut out: *mut bindings::VipsImage = null_mut();

        let ret = bindings::vips_extract_area(
            inp,
            &mut out,
            left_in.try_into().unwrap(),
            top_in.try_into().unwrap(),
            width_in.try_into().unwrap(),
            height_in.try_into().unwrap(),
            NULL,
        );
        return out;
    }
}

fn calculate_crop(
    in_width: i32,
    in_height: i32,
    out_width: i32,
    out_height: i32,
    gravity: i32,
) -> (i32, i32) {
    let mut left = 0;
    let mut top = 0;
    match gravity {
        1 => {
            left = (in_width - out_width + 1) / 2;
        }
        2 => {
            left = in_width - out_width;
            top = (in_height - out_height + 1) / 2;
        }
        3 => {
            left = (in_width - out_width + 1) / 2;
            top = in_height - out_height;
        }
        4 => {
            top = (in_width - out_width + 1) / 2;
        }
        _ => {
            left = (in_width - out_width + 1) / 2;
            top = (in_height - out_height + 1) / 2;
        }
    }
    return (left, top);
}
