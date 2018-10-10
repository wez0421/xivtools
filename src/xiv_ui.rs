use std::ffi::CStr;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::{BOOL, UINT};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    EnumWindows, GetWindowTextA, PostMessageA, VK_SPACE, WM_KEYDOWN, WM_KEYUP,
};

unsafe extern "system" fn enum_callback(win_hwnd: HWND, arg: LONG_PTR) -> BOOL {
    let mut title: Vec<i8> = vec![0; 256];
    let xiv_hwnd = arg as *mut HWND;

    if GetWindowTextA(win_hwnd, title.as_mut_ptr(), title.len() as i32) > 0 {
        let title = CStr::from_ptr(title.as_ptr()).to_str().unwrap();
        if title.contains("FINAL FANTASY XIV") {
            println!("enum callback called:{}", title);
            *xiv_hwnd = win_hwnd;
            return 0;
        }
    }
    1
}

fn enum_windows(handle: &mut HWND) -> i32 {
    unsafe { EnumWindows(Some(enum_callback), handle as *mut HWND as LONG_PTR) }
}

fn post_message(handle: HWND, msg: UINT, wparam: i32) -> i32 {
    unsafe { PostMessageA(handle, msg, wparam as usize, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xiv_window_detected() {
        let mut handle: HWND = null_mut();
        assert_eq!(enum_windows(&mut handle), 0);
    }
}