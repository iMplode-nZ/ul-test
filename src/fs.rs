use crate::conversions::from_ul_string;
use std::os::raw::c_char;
use std::os::raw::c_longlong;
use ul_sys::*;

#[allow(unused_variables)]
unsafe extern "C" fn file_exists(path: ULString) -> bool {
    println!("Finding: {:?}", from_ul_string(path));
    false
}
#[allow(unused_variables)]
unsafe extern "C" fn get_file_size(handle: ULFileHandle, result: *mut c_longlong) -> bool {
    false
}
#[allow(unused_variables)]
unsafe extern "C" fn get_file_mime_type(path: ULString, result: ULString) -> bool {
    println!("Finding Mime Type: {:?}", from_ul_string(path));
    false
}
#[allow(unused_variables)]
unsafe extern "C" fn open_file(path: ULString, open_for_writing: bool) -> ULFileHandle {
    println!("Opening: {}", from_ul_string(path));
    0
}
#[allow(unused_variables)]
unsafe extern "C" fn close_file(handle: ULFileHandle) {}
#[allow(unused_variables)]
unsafe extern "C" fn read_from_file(
    handle: ULFileHandle,
    data: *mut c_char,
    length: c_longlong,
) -> c_longlong {
    0
}

pub fn fs() -> ULFileSystem {
    ULFileSystem {
        file_exists: Some(file_exists),
        get_file_size: Some(get_file_size),
        get_file_mime_type: Some(get_file_mime_type),
        open_file: Some(open_file),
        close_file: Some(close_file),
        read_from_file: Some(read_from_file),
    }
}
