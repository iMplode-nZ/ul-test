use crate::conversions::from_ul_string;
use std::os::raw::c_uint;
use std::os::raw::c_void;
use ul_sys::*;
use unicode_segmentation::UnicodeSegmentation;

const LOCATION_LENGTH: usize = 30;
const FIRST_LINE_PADDING: usize = 5;
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
    if let Some((w, _)) = term_size::dimensions() {
        let location_fill = {
            let msg = from_ul_string(message);
            let msg_chars: Vec<&str> = UnicodeSegmentation::graphemes(&msg[..], true).collect();
            let location = format!(
                "{}:{}:{}",
                from_ul_string(source_id),
                line_number,
                column_number
            );
            let mut location_chars: Vec<&str> =
                UnicodeSegmentation::graphemes(&location[..], true).collect();
            let trimmed = if location_chars.len() > LOCATION_LENGTH {
                let pos = location_chars.len() - LOCATION_LENGTH;
                location_chars[pos] = "…";
                &location_chars[pos..]
            } else {
                &location_chars[..]
            };
            format!(
                "{:>width$}",
                trimmed.join(""),
                width = FIRST_LINE_PADDING + LOCATION_LENGTH
            )
        };
    } else {
        println!(
            "{}:{}:{}   {}",
            from_ul_string(source_id),
            line_number,
            column_number,
            from_ul_string(message),
        );
    }
}
