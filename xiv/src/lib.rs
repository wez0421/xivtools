pub mod ui;
pub use ui::*;

use std::fmt;
#[cfg(windows)]
use {
    std::ffi::CStr,
    winapi::shared::basetsd::LONG_PTR,
    winapi::shared::minwindef::BOOL,
    winapi::shared::windef::HWND,
    winapi::um::winuser::{EnumWindows, GetWindowTextA},
};

// The main handle passed back to library methods. The contents are kept
// private to avoid leaking any winapi dependencies to callers.
pub struct XivHandle {
    #[cfg(windows)]
    hwnd: HWND,
}

impl fmt::Debug for XivHandle {
    #[cfg(windows)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Xivhandle {{ {} }}", self.hwnd as LONG_PTR as u64)
    }
    #[cfg(not(windows))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XivHandle {{ }}")
    }
}

#[cfg(windows)]
pub fn init() -> XivHandle {
    let mut arg = std::ptr::null_mut();
    unsafe {
        // TODO: Figure out Rust error handling rather than just panicking inside a lib
        // method.
        match EnumWindows(Some(enum_callback), &mut arg as *mut HWND as LONG_PTR) {
            0 => XivHandle { hwnd: arg as HWND },
            _ => panic!("Unable to find XIV window! Is Final Fantasy XIV running?:win_hwnd"),
        }
    }
}
#[cfg(not(windows))]
pub fn init() -> XivHandle {
    XivHandle {}
}

// This callback is called for every window the user32 EnumWindows call finds
// while walking the window list. It's used to find the XIV window by title.
//
// To be more foolproof checking process name might be better.
#[cfg(windows)]
unsafe extern "system" fn enum_callback(win_hwnd: HWND, arg: LONG_PTR) -> BOOL {
    let mut title: Vec<i8> = vec![0; 256];
    let xiv_hwnd = arg as *mut HWND;

    if GetWindowTextA(win_hwnd, title.as_mut_ptr(), title.len() as i32) > 0 {
        let title = CStr::from_ptr(title.as_ptr()).to_string_lossy();
        println!("found {}: {:?}, arg {:?}", title, win_hwnd, xiv_hwnd);
        if title.contains("FINAL FANTASY XIV") {
            *xiv_hwnd = win_hwnd;
            return 0;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_window() {
        let h = init();
        ui::clear_window(&h);
    }
}
