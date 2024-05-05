#![allow(non_snake_case)]

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::winuser::{MessageBoxA, MB_OK};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::shared::minwindef::HINSTANCE;

#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: u32,
    _lpv_reserved: *mut std::ffi::c_void,
) -> i32 {
    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(hinst_dll);
                show_message();
            }
        }
        _ => {}
    }
    1
}

#[no_mangle]
pub extern "C" fn show_message() {
    unsafe {
        let title = CString::new("Success!").expect("CString::new failed");
        let message = CString::new("This is a message box from a Rust DLL!").expect("CString::new failed");

        MessageBoxA(null_mut(), message.as_ptr(), title.as_ptr(), MB_OK);
    }
}
