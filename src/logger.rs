use crate::conversions::from_ul_string;
use std::os::raw::c_uint;
use std::os::raw::c_void;
use ul_sys::*;
#[allow(unused_variables)]
pub unsafe extern "C" fn logger_callback(
    _user_data: *mut c_void,
    caller: ULView,
    source: ULMessageSource,
    level: ULMessageLevel,
    message: ULString,
    line_number: c_uint,
    column_number: c_uint,
    source_id: ULString,
) {
    /*if let Some((w, _)) = term_size::dimensions() {
        let v: Vec<&str> = UnicodeSegmentation::graphemes(&from_ul_string(message)[..], true).collect();

    } else {*/
        println!(
            "{}:{:?}:{:?}   {}",
            from_ul_string(source_id),
            line_number,
            column_number,
            from_ul_string(message),
        );
    // }
}
