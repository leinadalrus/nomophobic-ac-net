#![feature(error_in_core)]
#![feature(core_intrinsics)]
#![feature(lang_items)]

extern crate alloc;
extern crate core;

use std::{panic, prelude::v1::*};
use windows::{
    core::*,
    Win32::System::{Com::*, Threading::CreateThreadpoolWork},
};

extern "C" fn system_callback_panic(
    _argc: usize, _argv: *const *const u32,
) -> ! {
    if _argc >= 1 {
        unsafe {
            match char::from_u32(**_argv) {
                Some('1') => assert!(std::process::Command::new(
                    "cd $(ls | grep -r 'entrypoint.sh' ./); ./entrypoint.sh; \
                     ./entrypoint.sh init"
                )
                .spawn()
                .is_ok()),
                Some('2') => assert!(std::process::Command::new(
                    "npm i; npm build; npm run"
                )
                .spawn()
                .is_ok()),
                None => {
                    panic!("echo Error has Occurred in Command Processes...[!]")
                }
                _ => panic!(),
            }
        }
    }

    core::intrinsics::abort();
}

unsafe extern "system" fn callback(
    _: *mut windows::Win32::System::Threading::TP_CALLBACK_INSTANCE,
    _: *mut std::ffi::c_void,
    _: *mut windows::Win32::System::Threading::TP_WORK,
) {
}

fn main() -> core::result::Result<(), alloc::boxed::Box<dyn core::error::Error>>
{
    unsafe {
        let uri = CreateUri(
            w!("https://briochure.app.org"),
            Uri_CREATE_CANONICALIZE,
            0,
        )?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        let work = CreateThreadpoolWork(
            Some(callback),
            Some(std::ptr::null_mut()),
            Some(std::ptr::null()),
        );

        if work.is_null() {
            println!("{:?}", windows::Win32::Foundation::GetLastError());
            println!("{domain} ({port})");
            // system_callback_panic(); //  needs CLI arguments
        }

        for _ in 0..10 {
            windows::Win32::System::Threading::SubmitThreadpoolWork(work);
            println!("{domain} ({port})");
        }

        Ok(())
    }
}
