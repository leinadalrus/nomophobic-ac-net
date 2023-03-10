use neon::{
    result::JsResult,
    types::JsString,
};

struct ParameterizedHandleContext {
    input: JsString,
}

trait ContextHandler {
    fn parameterized_handle_context(&self) -> JsResult<'static, JsString>;
}

impl ContextHandler for ParameterizedHandleContext {
    fn parameterized_handle_context(&self) -> JsResult<'static, JsString> {
        let user_input = &self.input;
        match Some(user_input) {
            Some(_) => todo!(),
            None => todo!(),
        };

        todo!()
    }
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    Ok(())
}
