use std::mem::transmute;
use ul_sys::*;

pub fn from_ul_string(x: ULString) -> String {
    unsafe {
        let str_raw = *transmute::<_, *const (*const u16, usize)>(x);
        let slice = std::slice::from_raw_parts(str_raw.0, str_raw.1);
        String::from_utf16(slice).unwrap()
    }
}
