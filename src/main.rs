use std::ffi::CStr;

use ul_sys::*;
mod conversions;
mod fs;
mod logger;

fn main() {
    unsafe {
        ulPlatformSetFileSystem(fs::fs());
        let (width, height): (u32, u32) = (1280, 768);

        let settings = ulCreateSettings();
        let config = ulCreateConfig();
        let app = ulCreateApp(settings, config);
        let monitor = ulAppGetMainMonitor(app);
        let window = ulCreateWindow(monitor, width, height, false, 0);

        ulAppSetWindow(app, window);

        let overlay = ulCreateOverlay(window, width, height, 0, 0);
        let view = ulOverlayGetView(overlay);
        ulViewSetAddConsoleMessageCallback(
            view,
            Some(logger::logger_callback),
            std::ptr::null_mut(),
        );
        ulViewLoadURL(
            view,
            ulCreateString(
                CStr::from_bytes_with_nul(b"http://apple.com\0")
                    .unwrap()
                    .as_ptr(),
            ),
        );

        ulAppRun(app);
    }
}
