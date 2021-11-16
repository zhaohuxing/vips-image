use crate::resize;
use crate::vips::*;
use libvips::bindings;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

pub fn watermark_text(buf: &[u8], text: &str, dpi: i32, opacity: f32, color: &str) -> Vec<u8> {
    let _type = vips_image_type(&buf);
    let image = vips_image_new_from_buffer(&buf, "");
    let overlay = vips_text(text, dpi, color);
    let out = vips_composite2(image, overlay, bindings::VipsBlendMode_VIPS_BLEND_MODE_OVER);
    return vips_image_write_to_buffer(out, &format!(".{}", _type));
}

pub fn watermark_image(buf: &[u8], overlay: &[u8], x: i32, y: i32, opacity: i32) -> Vec<u8> {
    let _type = vips_image_type(&buf);
    let image = vips_image_new_from_buffer(&buf, "");
    let overlay_in = vips_image_new_from_buffer(&resize(overlay, 100, 100), "");
    let out = vips_composite2(
        image,
        overlay_in,
        bindings::VipsBlendMode_VIPS_BLEND_MODE_OVER,
    );
    return vips_image_write_to_buffer(out, &format!(".{}", _type));
}

fn vips_text(text: &str, dpi: i32, color: &str) -> *mut bindings::VipsImage {
    unsafe {
        let mut out_out: *mut bindings::VipsImage = null_mut();
        let text = format!("<span foreground='{}'>{}</span>", color, text);
        let text_in = CString::new(text.as_str()).unwrap();
        let rgba_in = CString::new("rgba").unwrap();
        let dpi_in = CString::new("dpi").unwrap();
        let ret = bindings::vips_text(
            &mut out_out,
            text_in.as_ptr(),
            dpi_in.as_ptr(),
            dpi,
            rgba_in.as_ptr(),
            1,
            NULL,
        );
        return out_out;
    }
}

fn vips_composite2(
    inp: *mut bindings::VipsImage,
    overlay: *mut bindings::VipsImage,
    mode: bindings::VipsBlendMode,
) -> *mut bindings::VipsImage {
    unsafe {
        //let mode_in: i32 = 12;
        let mode_in: i32 = mode as i32;
        let mut out_out: *mut bindings::VipsImage = null_mut();
        let ret = bindings::vips_composite2(
            inp,
            overlay,
            &mut out_out,
            mode_in.try_into().unwrap(),
            NULL,
        );
        return out_out;
    }
}
