use crate::vips::*;
use libvips::bindings;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;

#[derive(Debug)]
pub struct Metadata {
    pub width: i32,
    pub height: i32,
    pub orientation: String,
    pub alpha: bool,
    pub ttype: String,
    pub space: String,
    pub make: String,
    pub mode: String,
    pub datetime: String,
    pub exif_version: String,
    pub focal_length: String,
    pub gps_latitude_ref: String,
    pub gps_latitude: String,
    pub gps_longitude_ref: String,
    pub gps_longitude: String,
    pub gps_altitude_ref: String,
    pub gps_altitude: String,
}

pub fn get_metadata(buf: &[u8]) -> Metadata {
    let (image, ttype) = vips_read(&buf);
    let orientation = image_get_string(image, "exif-ifd0-Orientation");
    let make = image_get_string(image, "exif-ifd0-Make");
    let mode = image_get_string(image, "exif-ifd0-Model");
    let datetime = image_get_string(image, "exif-ifd0-DateTime");
    let exif_version = image_get_string(image, "exif-ifd2-ExifVersion");
    let focal_length = image_get_string(image, "exif-ifd2-FocalLength");
    let gps_latitude_ref = image_get_string(image, "exif-ifd3-GPSLatitudeRef");
    let gps_latitude = image_get_string(image, "exif-ifd3-GPSLatitude");
    let gps_longitude_ref = image_get_string(image, "exif-ifd3-GPSLongitudeRef");
    let gps_longitude = image_get_string(image, "exif-ifd3-GPSLongitude");
    let gps_altitude_ref = image_get_string(image, "exif-ifd3-GPSAltitudeRef");
    let gps_altitude = image_get_string(image, "exif-ifd3-GPSAltitude");
    let width = vips_image_get_width(image);
    let height = vips_image_get_height(image);
    let alpha = vips_has_alpha(image);
    let space = vips_space(image);
    let metadata = Metadata {
        width,
        height,
        ttype,
        space,
        orientation,
        alpha,
        make,
        mode,
        datetime,
        exif_version,
        focal_length,
        gps_latitude_ref,
        gps_latitude,
        gps_longitude_ref,
        gps_longitude,
        gps_altitude_ref,
        gps_altitude,
    };

    return metadata;
}

fn vips_read(buf: &[u8]) -> (*mut bindings::VipsImage, String) {
    let image_type = vips_image_type(&buf);
    //let image = VipsImage::new_from_buffer(buf, "").unwrap();
    let image = vips_image_new_from_buffer(buf, "");
    return (image, image_type);
}

pub fn image_get_string(inp: *mut bindings::VipsImage, name: &str) -> String {
    unsafe {
        let inp_in: *mut bindings::VipsImage = inp;
        let name_in = CString::new(name).unwrap();
        let mut out: *const c_char = null_mut();
        let vips_op_response = bindings::vips_image_get_string(inp_in, name_in.as_ptr(), &mut out);
        if vips_op_response == -1 {
            return "".to_string();
        }

        let out = CStr::from_ptr(out).to_str().unwrap();
        return String::from(out);
    }
}

fn vips_has_alpha(inp: *mut bindings::VipsImage) -> bool {
    unsafe {
        if ((*inp).Bands == 2
            && (*inp).Type == bindings::VipsInterpretation_VIPS_INTERPRETATION_B_W)
            || ((*inp).Bands == 4
                && (*inp).Type == bindings::VipsInterpretation_VIPS_INTERPRETATION_CMYK)
            || ((*inp).Bands == 5
                && (*inp).Type == bindings::VipsInterpretation_VIPS_INTERPRETATION_CMYK)
        {
            return true;
        } else {
            return false;
        }
    }
}

fn vips_space(inp: *mut bindings::VipsImage) -> String {
    unsafe {
        let res = bindings::vips_enum_nick(bindings::vips_interpretation_get_type(), (*inp).Type);
        if res.is_null() {
            return "".to_string();
        }
        let out = CStr::from_ptr(res).to_str().unwrap();
        return String::from(out);
    }
}

fn vips_image_get_width(inp: *mut bindings::VipsImage) -> i32 {
    unsafe { return bindings::vips_image_get_width(inp) }
}

fn vips_image_get_height(inp: *mut bindings::VipsImage) -> i32 {
    unsafe { return bindings::vips_image_get_height(inp) }
}
