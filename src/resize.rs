use crate::vips::*;
use libvips::bindings;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;

pub fn resize(buf: &[u8], width: i32, height: i32) -> Vec<u8> {
    let _type = vips_image_type(&buf);
    let inp = thumbnail_buffer(&buf, width, height, &_type);
    if _type == "gif" {
        return gifsave_buffer(inp);
    } else {
        return vips_image_write_to_buffer(inp, &format!(".{}", _type));
    }
}

pub fn thumbnail_buffer(
    buffer: &[u8],
    width: i32,
    height: i32,
    _type: &str,
) -> *mut bindings::VipsImage {
    unsafe {
        let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
        let width_in: i32 = width;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let height_in: i32 = height;
        let height_str = CString::new("height").unwrap();
        // https://github.com/libvips/pyvips/issues/129
        // if loader == "gifload" or loader == "webpload":
        // # an animated format -- make a thumbnail of all frames
        // options = "n=-1"

        let option_str = CString::new("option_string").unwrap();
        let option_val = CString::new("n=-1").unwrap();
        let mut vips_op_response = 0;
        if _type == "gif" || _type == "webp" {
            if height == 0 {
                vips_op_response = bindings::vips_thumbnail_buffer(
                    buffer_in,
                    buffer.len() as u64,
                    &mut out_out,
                    width_in,
                    option_str.as_ptr(),
                    option_val.as_ptr(),
                    NULL,
                );
            } else {
                vips_op_response = bindings::vips_thumbnail_buffer(
                    buffer_in,
                    buffer.len() as u64,
                    &mut out_out,
                    width_in,
                    height_str.as_ptr(),
                    height_in,
                    option_str.as_ptr(),
                    option_val.as_ptr(),
                    NULL,
                );
            }
        } else {
            if height == 0 {
                vips_op_response = bindings::vips_thumbnail_buffer(
                    buffer_in,
                    buffer.len() as u64,
                    &mut out_out,
                    width_in,
                    NULL,
                );
            } else {
                vips_op_response = bindings::vips_thumbnail_buffer(
                    buffer_in,
                    buffer.len() as u64,
                    &mut out_out,
                    width_in,
                    height_str.as_ptr(),
                    height_in,
                    NULL,
                );
            }
        }
        return out_out;
    }
}
