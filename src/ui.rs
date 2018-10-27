pub use self::ui_impl::WinHandle;

use std::thread::sleep;
use std::time::Duration;

// Creating these simplifies the wait code for the craft module
pub fn wait_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

pub fn wait_secs(s: u64) {
    sleep(Duration::from_secs(s));
}

pub fn cursor_down(window: WinHandle) {
    log::trace!("Down");
    ui_impl::cursor_down(window);
}

pub fn cursor_up(window: WinHandle) {
    log::trace!("Up");
    ui_impl::cursor_up(window);
}

pub fn cursor_left(window: WinHandle) {
    log::trace!("Left");
    ui_impl::cursor_left(window);
}

pub fn cursor_right(window: WinHandle) {
    log::trace!("Right");
    ui_impl::cursor_right(window);
}

pub fn move_backward(window: WinHandle) {
    log::trace!("<-");
    ui_impl::move_backward(window);
}

pub fn _move_forward(window: WinHandle) {
    log::trace!("->");
    ui_impl::move_forward(window);
}

pub fn confirm(window: WinHandle) {
    log::trace!("Confirm");
    ui_impl::confirm(window);
}

pub fn cancel(window: WinHandle) {
    log::trace!("Cancel");
    ui_impl::cancel(window);
}

pub fn enter(window: WinHandle) {
    log::trace!("Enter");
    ui_impl::enter(window);
}

pub fn escape(window: WinHandle) {
    log::trace!("Escape");
    ui_impl::escape(window);
}

pub fn open_craft_window(window: WinHandle) {
    log::trace!("CraftWindow");
    ui_impl::open_craft_window(window);
}

pub fn send_char(window: WinHandle, c: char) {
    ui_impl::send_char(window, c);
}

pub fn get_window(hwnd: &mut WinHandle) -> bool {
    ui_impl::get_window(hwnd)
}

#[cfg(windows)]
mod ui_impl {
    use failure::Error;
    use std::ffi::CStr;
    use std::ptr::null_mut;
    use std::sync::Once;
    use std::thread::sleep;
    use std::time::Duration;
    use winapi::shared::basetsd::LONG_PTR;
    use winapi::shared::minwindef::{BOOL, UINT};
    pub use winapi::shared::windef::{HWND, HWND__};
    pub use winapi::um::winuser::*;
    pub use winapi::um::winuser::{EnumWindows, GetWindowTextA, PostMessageA};

    pub type WinHandle = HWND;

    // TODO: Configurable keybinds
    const KEY_UP: i32 = VK_NUMPAD8;
    const KEY_DOWN: i32 = VK_NUMPAD2;
    const KEY_LEFT: i32 = VK_NUMPAD4;
    const KEY_RIGHT: i32 = VK_NUMPAD6;
    const KEY_CONFIRM: i32 = VK_NUMPAD0;
    const KEY_FORWARD: i32 = VK_NUMPAD9;
    const KEY_BACKWARD: i32 = VK_NUMPAD7;
    const KEY_CANCEL: i32 = VK_DECIMAL;
    const KEY_ENTER: i32 = VK_RETURN;

    // Common public methods the ui_impl modules export
    pub fn cursor_down(window: HWND) {
        send_key(window, KEY_DOWN);
    }
    pub fn cursor_up(window: HWND) {
        send_key(window, KEY_UP);
    }
    pub fn cursor_left(window: HWND) {
        send_key(window, KEY_LEFT);
    }
    pub fn cursor_right(window: HWND) {
        send_key(window, KEY_RIGHT);
    }
    pub fn move_backward(window: HWND) {
        send_key(window, KEY_BACKWARD)
    }
    pub fn move_forward(window: HWND) {
        send_key(window, KEY_FORWARD);
    }
    pub fn confirm(window: HWND) {
        send_key(window, KEY_CONFIRM);
    }
    pub fn cancel(window: HWND) {
        send_key(window, KEY_CANCEL);
    }
    pub fn enter(window: HWND) {
        send_key(window, KEY_ENTER);
    }
    pub fn escape(window: HWND) {
        send_key(window, VK_ESCAPE);
    }
    pub fn open_craft_window(window: HWND) {
        send_key(window, 'N' as i32);
    }

    pub fn send_key(window: HWND, c: i32) {
        send_msg(window, WM_KEYDOWN, c);
        send_msg(window, WM_KEYUP, c);
        sleep(Duration::from_millis(150));
    }

    pub fn send_char(window: HWND, c: char) {
        send_msg(window, WM_CHAR, c as i32);
        sleep(Duration::from_millis(20));
    }

    // This callback is called for every window the user32 EnumWindows call finds
    // while walking the window list. Use it to find the XIV window by title.
    //
    // To be more foolproof checking process name might be better.
    unsafe extern "system" fn enum_callback(win_hwnd: HWND, arg: LONG_PTR) -> BOOL {
        let mut title: Vec<i8> = vec![0; 256];
        let xiv_hwnd = arg as *mut HWND;

        if GetWindowTextA(win_hwnd, title.as_mut_ptr(), title.len() as i32) > 0 {
            let title = CStr::from_ptr(title.as_ptr()).to_str().unwrap();
            if title.contains("FINAL FANTASY XIV") {
                println!("Found XIV window: {:?}", win_hwnd);
                *xiv_hwnd = win_hwnd;
                return 0;
            }
        }
        1
    }

    // Return the handle of the FFXIV window. The EnumWindow return is inverted because
    // we can live in a better world than one where 0 is success.
    // TODO: Figure out how to return good errors here.
    pub fn get_window(hwnd: &mut HWND) -> bool {
        unsafe {
            match EnumWindows(Some(enum_callback), hwnd as *mut HWND as LONG_PTR) {
                0 => true,
                _ => false,
            }
        }
    }

    // Send a character/key to the XIV window
    fn send_msg(window: HWND, msg: u32, key: i32) {
        unsafe {
            PostMessageA(window, msg as UINT, key as usize, 0);
        }
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
mod ui_impl {
    pub type WinHandle = *mut u64;

    // Common public methods the ui_impl modules export
    pub fn cursor_down(_: WinHandle) {}
    pub fn cursor_up(_: WinHandle) {}
    pub fn cursor_left(_: WinHandle) {}
    pub fn cursor_right(_: WinHandle) {}
    pub fn move_backward(_: WinHandle) {}
    pub fn move_forward(_: WinHandle) {}
    pub fn enter(_: WinHandle) {}
    pub fn confirm(_: WinHandle) {}
    pub fn cancel(_: WinHandle) {}
    pub fn escape(_: WinHandle) {}
    pub fn send_char(_: WinHandle, c: char) {
        //print!("{}", c);
    }
    pub fn open_craft_window(_: WinHandle) {}
    pub fn get_window(_: &mut WinHandle) -> bool {
        true
    }
}
