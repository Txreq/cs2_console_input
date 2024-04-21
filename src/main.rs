#![allow(dead_code)]
use std::ffi::c_void;

use windows::{
    core::s,
    Win32::{
        Foundation::LPARAM,
        UI::WindowsAndMessaging::{FindWindowA, SendMessageA},
    },
    Win32::{System::DataExchange::COPYDATASTRUCT, UI::WindowsAndMessaging::WM_COPYDATA},
};

fn main() {
    send_command("drop");
}

fn send_command(command: &str) {
    let h_engine = unsafe { FindWindowA(s!("SDL_app"), s!("Counter-Strike 2")) };
    println!("{:?}", h_engine);

    // let buffer = CString::new(command).unwrap();
    let buffer = command;
    let data = COPYDATASTRUCT {
        dwData: 0,
        cbData: (command.len() + 1) as u32,
        lpData: buffer.as_ptr() as *mut c_void,
    };

    unsafe {
        SendMessageA(
            h_engine,
            WM_COPYDATA,
            None,
            LPARAM(&data as *const _ as isize),
        );
    }

    println!("{:#?}", buffer);
    println!("{:#?}", data);
}
