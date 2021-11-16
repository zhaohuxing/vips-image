use libvips::bindings;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;

pub const NULL: *const c_void = null_mut();

lazy_static! {
    static ref ImageTypes: HashMap<String, bool> = {
        let mut map = HashMap::new();
        map.insert("JPEG".to_string(), true);
        map.insert("PNG".to_string(), true);
        map.insert("WEBP".to_string(), true);
        map.insert("TIFF".to_string(), true);
        map.insert("GIF".to_string(), true);
        map.insert("PDF".to_string(), true);
        map.insert("SVG".to_string(), true);
        map.insert("MAGICK".to_string(), true);
        map.insert("HEIF".to_string(), true);
        map.insert("AVIF".to_string(), true);
        map
    };
}

pub fn vips_image_type(buf: &[u8]) -> String {
    if buf.len() < 12 {
        return "unknown".to_string();
    }
    if buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF {
        return "jpeg".to_string();
    }

    if is_type_supported("GIF") && buf[0] == 0x47 && buf[1] == 0x49 && buf[2] == 0x46 {
        return "gif".to_string();
    }
    if buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4E && buf[3] == 0x47 {
        return "png".to_string();
    }
    if is_type_supported("TIFF")
        && ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0)
            || (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A))
    {
        return "tiff".to_string();
    }
    if is_type_supported("PDF")
        && buf[0] == 0x25
        && buf[1] == 0x50
        && buf[2] == 0x44
        && buf[3] == 0x46
    {
        return "pdf".to_string();
    }
    if is_type_supported("WEBP")
        && buf[8] == 0x57
        && buf[9] == 0x45
        && buf[10] == 0x42
        && buf[11] == 0x50
    {
        return "webp".to_string();
    }
    // TODO: implement is_svg_image
    //if IsTypeSupported(SVG) && IsSVGImage(buf) {
    //	return SVG
    //}

    // TODO: Not implement magick
    //if IsTypeSupported(MAGICK) && strings.HasSuffix(readImageType(buf), "MagickBuffer") {
    //	return MAGICK
    //}
    //
    // NOTE: libheif currently only supports heic sub types; see:
    //   https://github.com/strukturag/libheif/issues/83#issuecomment-421427091
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x69
        && buf[11] == 0x63
    {
        // This is a HEIC file, ftypheic
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x6d
        && buf[9] == 0x69
        && buf[10] == 0x66
        && buf[11] == 0x31
    {
        // This is a HEIF file, ftypmif1
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x6d
        && buf[9] == 0x73
        && buf[10] == 0x66
        && buf[11] == 0x31
    {
        // This is a HEIFS file, ftypmsf1
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x69
        && buf[11] == 0x73
    {
        // This is a HEIFS file, ftypheis
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x76
        && buf[11] == 0x63
    {
        // This is a HEIFS file, ftyphevc
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x61
        && buf[9] == 0x76
        && buf[10] == 0x69
        && buf[11] == 0x66
    {
        return "avif".to_string();
    }
    return "unknown".to_string();
}

pub fn is_type_supported(t: &str) -> bool {
    return ImageTypes.contains_key(t) && vips_type_find_bridge(t) != 0;
}

fn vips_type_find_bridge(t: &str) -> u64 {
    match t {
        "GIF" => return vips_type_find("VipsOperation", "gifload"),
        "PDF" => return vips_type_find("VipsOperation", "pdfload"),
        "TIFF" => return vips_type_find("VipsOperation", "tiffload"),
        "SVG" => return vips_type_find("VipsOperation", "svgload"),
        "WEBP" => return vips_type_find("VipsOperation", "webpload"),
        "PNG" => return vips_type_find("VipsOperation", "pngload"),
        "JPEG" => return vips_type_find("VipsOperation", "jpegload"),
        "MAGICK" => return vips_type_find("VipsOperation", "magickload"),
        "HEIF" => return vips_type_find("VipsOperation", "heifload"),
        _ => return 0,
    }
}

fn vips_type_find(name: &str, nickname: &str) -> u64 {
    unsafe {
        let name_in = CString::new("VipsOperation").unwrap();
        let nickname_in = CString::new("gifload").unwrap();
        let ret = bindings::vips_type_find(name_in.as_ptr(), nickname_in.as_ptr());
        return ret;
    }
}

pub fn vips_image_write_to_buffer(inp: *mut bindings::VipsImage, suffix: &str) -> Vec<u8> {
    unsafe {
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();
        let suffix_c_str = CString::new(suffix).unwrap();
        let res = bindings::vips_image_write_to_buffer(
            inp,
            suffix_c_str.as_ptr(),
            &mut buffer_out,
            &mut buffer_buf_size,
            NULL,
        );
        return Vec::from_raw_parts(
            buffer_out as *mut u8,
            buffer_buf_size as usize,
            buffer_buf_size as usize,
        );
    }
}

pub fn gifsave_buffer(inp: *mut bindings::VipsImage) -> Vec<u8> {
    unsafe {
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out: *mut c_void = null_mut();
        let format_str = CString::new("format").unwrap();
        let format_val = CString::new("GIF").unwrap();

        let vips_op_response = bindings::vips_magicksave_buffer(
            inp,
            &mut buffer_out,
            &mut buffer_buf_size,
            format_str.as_ptr(),
            format_val.as_ptr(),
            NULL,
        );
        return Vec::from_raw_parts(
            buffer_out as *mut u8,
            buffer_buf_size as usize,
            buffer_buf_size as usize,
        );
    }
}

pub fn vips_image_new_from_buffer(buf: &[u8], option_str: &str) -> *mut bindings::VipsImage {
    unsafe {
        let options = CString::new(option_str).unwrap();
        let res = bindings::vips_image_new_from_buffer(
            buf.as_ptr() as *const c_void,
            buf.len() as u64,
            options.as_ptr(),
            NULL,
        );
        return res;
    }
}

pub fn vips_init(name: &str) {
    unsafe {
        let name_in = CString::new(name).unwrap();
        bindings::vips_init(name_in.as_ptr());
    }
}
