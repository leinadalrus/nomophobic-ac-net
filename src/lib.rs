#![feature(error_in_core)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![allow(unused_must_use)]

extern crate alloc;
extern crate core;

use std::{panic, prelude::v1::*};
use windows::{
    core::*,
    Win32::System::{Com::*, Threading::CreateThreadpoolWork},
};

use neon::{
    context::Context,
    prelude::{FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::JsString,
};
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, WorkerLink};

pub struct WorkerHandler {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerHandlerInput {
    pub input: String,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerHandlerOutput {
    pub output: String,
}

impl yew_agent::Worker for WorkerHandler {
    type Input = WorkerHandlerInput;
    type Message = ();
    type Output = WorkerHandlerOutput;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self { return Self { link }; }

    fn update(&mut self, message: Self::Message) {}

    fn connected(&mut self, _id: HandlerId) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        fn process_ftl_file_through(input: String) -> String {
            // process the file from an internal string (impure) function ...
            // ... into a `&'static str` type for unicode/utf-8 encoding into
            // ftl file.
            let output: String = input;
            return output;
        }

        let output = Self::Output {
            output: process_ftl_file_through(msg.input).to_owned(),
        };

        self.link.respond(id, output);
    }

    fn disconnected(&mut self, _id: HandlerId) {}

    fn destroy(&mut self) {}

    fn name_of_resource() -> &'static str { "index.js" }

    fn resource_path_is_relative() -> bool { false }

    fn is_module() -> bool { false }
}

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
        Box::<WorkerHandlerInput>::new(WorkerHandlerInput {
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
        Box::<WorkerHandlerOutput>::new(WorkerHandlerOutput {
            output: String::from("Hello, World!"),
        });

    match module_context.try_string(&working_handle_out.output) {
        Ok(/* JsString */_) => module_context.export_function(
            &working_handle_out.output,
            setup_worker_handles_exporting,
        ),
        Err(_) => panic!("Error(s) has Occurred in Module Context and Result [!]"),
    };

    exec(); // TODO(Option): Option => Some | None | _ -> Result

    Ok(())
}
// TODO(Daniel): Shared Immutable := Singleton design pattern with Strategy
// design pattern
