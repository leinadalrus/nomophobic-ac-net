use neon::{
    context::Context,
    prelude::{FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::JsString,
};
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, WorkerLink};

struct ArcWorkerHandler {
    link: WorkerLink<Self>,
}

struct ArcWorkerHandlerInput {
    input: String,
}

struct ArcWorkerHandlerOutput {
    output: String,
}

impl yew_agent::Worker for ArcWorkerHandler {
    type Input = ArcWorkerHandlerInput;
    type Message = ();
    type Output = ArcWorkerHandlerOutput;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self { return Self { link }; }

    fn update(&mut self, message: Self::Message) {}

    fn connected(&mut self, _id: HandlerId) {} // use regex to fetch uri of api

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

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    if let state = std::option::Option<ArcWorkerHandlerInput, std::error::Error>::new() {
        match state {
            Ok(state) => println!("State is Ok!"),
            Err(_) => panic!("An error has occurred in a singleton!")
        }
    }

    Ok(())
}