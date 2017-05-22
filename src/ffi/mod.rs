extern crate libc;

use std::ffi::CStr;
use std::ptr;
use self::libc::{c_char, ssize_t, size_t};

use error::ErrorKind;

// Error codes related to input values
const ERR_NULL_PATH: ssize_t = 1;
const ERR_NON_UTF8: ssize_t = 2;

// Error codes related to SLP decoding
const ERR_PALETTE_INVALID: ssize_t = -1;
const ERR_UNKNOWN: ssize_t = -32767;

#[no_mangle]
pub extern "C" fn palette_free(rgb_data_buff: *const c_char, rgb_data_buff_len: size_t) {
    if rgb_data_buff.is_null() || rgb_data_buff_len == 0 {
        return;
    }

    unsafe {
        Vec::from_raw_parts(rgb_data_buff as *mut u8, rgb_data_buff_len, rgb_data_buff_len);
    }
}

#[no_mangle]
pub extern "C" fn palette_new_from_file(file_path: *const c_char,
         /* ptr to the 1D byte array */ out_rgb_data_buff: *mut *const c_char,
         /* ptr to number of entries */ out_len: *mut size_t) -> ssize_t {

    unsafe {
        *out_rgb_data_buff = ptr::null_mut();
        *out_len = 0;
    }

    let c_str = unsafe {
        if file_path.is_null() {
            return ERR_NULL_PATH;
        }

        CStr::from_ptr(file_path)
    };

    let file_path = match c_str.to_str() {
        Ok(p) => p,
        Err(_) => return ERR_NON_UTF8,
    };

    let colors = match ::palette::read_from_file(file_path) {
        Ok(pal) => pal,
        Err(e) => match *e.kind() {
            ErrorKind::ParseIntError(ref err) => {
                println!("ERR(ParseInt): {}", err);
                return ERR_PALETTE_INVALID;
            },
            ErrorKind::InvalidPalette(s) => {
                println!("ERR(InvalidPalette): {}", s);
                return ERR_PALETTE_INVALID;
            },
            _ => return ERR_UNKNOWN,
        }
    };

    unsafe {
        *out_rgb_data_buff = colors.as_ptr() as *const c_char;
        *out_len = colors.len();
    }

    ::std::mem::forget(colors);

    0 as ssize_t
}