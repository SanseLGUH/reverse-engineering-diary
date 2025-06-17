use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    let result = unsafe { MessageBeep(0x00000010) };

    match result {
        true =>  println!("working"),
        false => println!("doesn't work") 
    }

    unsafe {
        MessageBoxA(
            std::ptr::null_mut(), 
            None, None, 
            0x00000006 | 0x00004000
        );
    }
}   

type HWND = u64;
#[link(name = "user32")]
unsafe extern "system" {
    // Plays a waveform sound. The waveform sound for each sound type is identified by an entry in the registry.
    fn MessageBeep(uType: u32) -> bool;

    // Displays a modal dialog box that contains a system icon, a set of buttons, and a brief application-specific message, 
    // such as status or error information. The message box returns an integer value that indicates which button the user clicked.
    fn MessageBoxA(
        hwnd: *mut std::ffi::c_void,
        lp_text: Option<*const c_char>,
        lp_caption: Option<*const c_char>,
        u_type: u32,
    ) -> i32;
}