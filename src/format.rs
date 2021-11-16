use crate::vips::*;
use libvips::bindings;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;

pub fn format(buf: &[u8], format_type: &str) -> Vec<u8> {
    let image = vips_image_new_from_buffer(buf, "");
    return vips_image_write_to_buffer(image, &format!(".{}", format_type));
}
