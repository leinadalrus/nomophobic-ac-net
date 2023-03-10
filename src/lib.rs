#![feature(error_in_core)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]

extern crate alloc;
extern crate core;

pub mod bin;

use bin::arc_collection_processor::{ArcWorkerHandlerInput, ArcWorkerHandlerOutput};
use neon::{
    context::Context,
    prelude::{FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::JsString,
};

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

fn setup_worker_handles_exporting(
    mut function_context: FunctionContext,
) -> JsResult<JsString> {
    let working_handle_in =
        Box::<ArcWorkerHandlerInput>::new(ArcWorkerHandlerInput {
            input: String::from("Hello, World!"),
        });
    Ok(function_context.string(working_handle_in.input))
}

pub fn exec(
) -> core::result::Result<(), alloc::boxed::Box<dyn core::error::Error>> {
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

#[neon::main]
pub fn main(mut module_context: ModuleContext) -> NeonResult<()> {
    let working_handle_out =
        Box::<ArcWorkerHandlerOutput>::new(ArcWorkerHandlerOutput {
            output: String::from("Hello, World!"),
        });

    match module_context.try_string(&working_handle_out.output) {
        Ok(/* JsString */ _) => module_context.export_function(
            &working_handle_out.output,
            setup_worker_handles_exporting,
        ),
        Err(_) => {
            panic!("Error(s) has Occurred in Module Context and Result [!]")
        }
    };

    exec(); // TODO(Option): Option => Some | None | _ -> Result

    Ok(())
}
// TODO(Daniel): Shared Immutable := Singleton design pattern with Strategy
// design pattern
