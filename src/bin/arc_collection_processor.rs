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

#[derive(Deserialize, Serialize)]
pub struct ArcWorkerHandlerInput {
    input: String,
}

#[derive(Deserialize, Serialize)]
pub struct ArcWorkerHandlerOutput {
    output: String,
}

impl yew_agent::Worker for ArcWorkerHandler {
    type Input = ArcWorkerHandlerInput;
    type Message = ();
    type Output = ArcWorkerHandlerOutput;
    type Reach = Public<ArcWorkerHandler>;

    fn create(link: WorkerLink<Self>) -> Self { return Self { link }; }

    fn update(&mut self, message: Self::Message) {
        let shared_immutable = ArcWorkerHandlerInput {
            input: Self::name_of_resource().to_owned(),
        };
        if let state = Self::resource_path_is_relative() {
            match state {
                true => println!("resource_path_is_relative == true ? !false : 1"),
                false => panic!("resource_path_is_relative != true ? false : 0"),
                _ => (),
            }

            match shared_immutable {
                ArcWorkerHandlerInput => println!("name_of_resource().to_owned() == true ? !false : 1"),
                _ => (),
            } // regex match state => shared_immutable.input => Result<()>
        }
    }

    fn connected(&mut self, _id: HandlerId) {}

    // use regex to fetch uri of api

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
    Ok(())
}
