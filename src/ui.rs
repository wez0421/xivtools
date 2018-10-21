pub use self::ui_impl::*;
use std::thread::sleep;
use std::time::Duration;

// Creating these simplifies the wait code for the craft module
pub fn wait_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

pub fn wait_secs(s: u64) {
    sleep(Duration::from_secs(s));
}

#[cfg(windows)]
pub(self) mod ui_impl {
    use failure::Error;
    use std::ffi::CStr;
    use std::sync::Once;
    use winapi::shared::basetsd::LONG_PTR;
    use winapi::shared::minwindef::{BOOL, UINT};
    pub use winapi::shared::windef::HWND;
    pub use winapi::um::winuser::*;
    pub use winapi::um::winuser::{EnumWindows, GetWindowTextA, PostMessageA};

    // TODO: Configurable keybinds
    const KEY_UP: char = VK_UNUMPAD8;
    const KEY_DOWN: char = VK_NUMPAD2;
    const KEY_LEFT: char = VK_NUMPAD4;
    const KEY_RIGHT: char = VK_NUMPAD6;
    const KEY_CONFIRM: char = VK_NUMPAD0;
    const KEY_FORWARD: char = VK_NUMPAD9;
    const KEY_BACKWARD: char = VK_NUMPAD7;
    const KEY_CANCEL: char = VK_DECIMAL;
    const KEY_ENTER: char = VK_ENTER;

    // Common public methods the ui_impl modules export
    pub fn cursor_down() {
        send_key(KEY_DOWN);
    }
    pub fn cursor_up() {
        send_key(KEY_UP);
    }
    pub fn cursor_left() {
        send_key(KEY_LEFT);
    }
    pub fn cursor_right() {
        send_key(KEY_RIGHT);
    }
    pub fn move_backward() {
        send_key(KEY_BACKWARD)
    }
    pub fn move_forward() {
        send_key(KEY_FORWARD);
    }
    pub fn confirm() {
        send_key(KEY_CONFIRM);
    }
    pub fn cancel() {
        send_key(KEY_CANCEL);
    }
    pub fn enter() {
        send_key(KEY_ENTER);
    }
    pub fn escape() {
        send_key(VK_ESCAPE);
    }

    pub fn open_craft_window() {
        send_key('N');
    }

    pub fn send_key(c: char) {
        send_msg(WINDOW, WM_KEYDOWN, c);
        send_msg(WINDOW, WM_KEYUP, c);
        sleep(Duration::from_millis(250));
    }

    pub fn send_char(c: char) {
        c::send_msg(window, WM_CHAR, c);
        sleep(Duration::from_millis(100));
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
                println!("enum callback called:{}", title);
                *xiv_hwnd = win_hwnd;
                return 0;
            }
        }
        1
    }

    // Return the handle of the FFXIV window. The first time this is called we make WinAPI
    // calls to find the window and cache it.
    static mut WINDOW: HWND = null_mut();
    static INIT: Once = Once::new();
    fn get_window() -> HWND {
        unsafe {
            INIT.call_once(|| {
                EnumWindows(Some(enum_callback), WINDOW as *mut HWND as LONG_PTR);
                hwnd
            });
        }
        WINDOW
    }

    // Send a character/key to the XIV window
    fn send_msg(msg: ui::msg, key: char) {
        unsafe { PostMessageA(WINDOW, msg_impl as UINT, key_impl as usize, 0) }
    }
}

#[cfg(not(windows))]
pub(self) mod ui_impl {
    // Common public methods the ui_impl modules export
    pub fn cursor_down() {
        print!("<D> ");
    }
    pub fn _cursor_up() {
        print!("<U> ");
    }
    pub fn _cursor_left() {
        print!("<L> ");
    }
    pub fn _cursor_right() {
        print!("<R> ");
    }
    pub fn move_backward() {
        print!("<- ");
    }
    pub fn _move_forward() {
        print!("-> ");
    }
    pub fn enter() {
        println!("<ENTER> ");
    }
    pub fn confirm() {
        println!("<OK> ");
    }
    pub fn cancel() {
        println!("<CANCEL ");
    }
    pub fn _escape() {
        println!("<ESC> ");
    }
    pub fn send_char(c: char) {
        print!("{}", c);
    }
    pub fn open_craft_window() {}
}
