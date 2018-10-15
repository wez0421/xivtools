
use std::thread::sleep;
use std::time::Duration;
pub use self::ui_impl::*;
use crate::xiv_macro;

pub fn find_xiv_window(handle: &mut HWND) {
    platform_find_xiv_window(handle)
}

pub fn toggle_collectable_status(handle: &HWND) {
    send_action(handle, "collectable synthesis", 2);
}

pub fn send_action(window: &HWND, action: &str, wait: u64) {
    println!("\t /ac {}", action);
    send_key(window, UI_ENTER);
    send_string(window, "/ac \"");
    send_string(window, action);
    send_string(window, "\"");
    sleep(Duration::from_millis(250));
    send_key(window, UI_ENTER);
    println!("\twaiting {} seconds", wait);
    sleep(Duration::from_secs(wait));
}

pub fn reset_action_keys(window: &HWND) {
    platform_send_char(window, UI_KEYUP, UI_CONFIRM);
    platform_send_char(window, UI_KEYUP, UI_ENTER);
    platform_send_char(window, UI_KEYUP, UI_CRAFT);
    sleep(Duration::from_millis(250));
}

pub fn craft_item(window: &HWND, actions: &Vec<xiv_macro::MacroEntry>, collectable: bool) {
    // Sending confirm to handle both first and subsequent crafts
    send_key(window, UI_CONFIRM);
       send_key(window, UI_CONFIRM);
    // Wait longer for the craft UI to come up
    sleep(Duration::from_secs(2));
    for entry in actions {
        send_action(window, &entry.action, entry.wait);
    }
        sleep(Duration::from_secs(1));
        if collectable {
            send_key(window, UI_CONFIRM);
            send_key(window, UI_CONFIRM);
            sleep(Duration::from_secs(1));
        }
                    sleep(Duration::from_secs(2));
}

// Clears the UI windows
pub fn reset_ui(window: &HWND) {
    for _ in 0..5 {
        platform_send_char(window, UI_KEYDOWN, UI_ESCAPE);
        platform_send_char(window, UI_KEYUP, UI_ESCAPE);
        sleep(Duration::from_millis(50));
    }
    sleep(Duration::from_millis(250));
}

// Brings up the crafting window and selects the item to craft
pub fn bring_craft_window(window: &HWND, item: &str, offset: u32) {
    send_key(window, UI_CRAFT);
    for _ in 0..8 {
        send_key(window, UI_PREV);
    }
    send_key(window, UI_CONFIRM);
    send_string(window, item);
    send_key(window, UI_ENTER);
    // Longer delay while waiting for search results to come up
    sleep(Duration::from_millis(500));
    for _ in 0..offset {
        send_key(window, UI_DOWN);
        sleep(Duration::from_millis(50));
    }
    send_key(window, UI_CONFIRM);
}

fn send_key(window: &HWND, c: i32) {
    platform_send_char(window, UI_KEYDOWN, c);
    platform_send_char(window, UI_KEYUP, c);
    sleep(Duration::from_millis(250));
}

fn send_string(window: &HWND, s: &str) {
    for c in s.chars() {
        platform_send_char(window, UI_CHAR, c as i32);
    }
    sleep(Duration::from_millis(100));
}

#[cfg(windows)] pub(self) mod ui_impl {
    use std::ffi::CStr;
    use failure::Error;
    use winapi::shared::basetsd::LONG_PTR;
    use winapi::shared::minwindef::{BOOL, UINT};
    pub use winapi::shared::windef::HWND;
    pub use winapi::um::winuser::{
        EnumWindows, GetWindowTextA, PostMessageA
    };

    pub use winapi::um::winuser::{ VK_SPACE, WM_CHAR, WM_KEYDOWN, WM_KEYUP, VK_RETURN, VK_DOWN, VK_UP, VK_PAUSE, VK_END, VK_ESCAPE } ;
    pub const UI_CRAFT: i32 = 0x4e; // N key
    pub const UI_ENTER: i32 = VK_RETURN;
    pub const UI_CONFIRM: i32 = VK_PAUSE;
    pub const UI_PREV: i32 = VK_END;
    pub const UI_DOWN: i32 = VK_DOWN;
    pub const UI_UP: i32 = VK_UP;
    pub const UI_ESCAPE: i32 = VK_ESCAPE;
    pub const UI_CHAR: u32 = WM_CHAR;
    pub const UI_KEYDOWN: u32 = WM_KEYDOWN;
    pub const UI_KEYUP: u32 = WM_KEYUP;

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
    pub fn platform_find_xiv_window(handle: &mut HWND) {
        unsafe { EnumWindows(Some(enum_callback), handle as *mut HWND as LONG_PTR); }
    }

    pub fn platform_send_char(window: &HWND, msg: u32, wparam: i32) -> i32 {
        unsafe { PostMessageA(*window, msg as UINT, wparam as usize, 0) }
    }
}

#[cfg(not(windows))]
pub(self) mod ui_impl {
    use super::*;

    pub const UI_CRAFT: i32 = 'C' as i32;
    pub const UI_ENTER: i32 = 'E' as i32;
    pub const UI_CONFIRM: i32 = 'C' as i32;
    pub const UI_PREV: i32 = 'P' as i32;
    pub const UI_DOWN: i32 = 'D' as i32;
    pub const UI_UP: i32 = 'U' as i32;
    pub const UI_ESCAPE: i32 = 'X' as i32;
    pub const UI_CHAR: u32 = 1;
    pub const UI_KEYDOWN: u32 = 2;
    pub const UI_KEYUP: u32= 3;
    pub struct HWND {}

    pub fn platform_send_char(_: &HWND, msg: u32, wparam: i32) -> i32 {
        let c: char = wparam as u8 as char;
        match msg {
            UI_KEYDOWN => print!(" [{} down]", c),
            UI_KEYUP => println!(" [{} up]", c),
            _ => print!("{}", c),
        }
        1
    }

    pub fn platform_find_xiv_window(_: &mut HWND) {
    }
}