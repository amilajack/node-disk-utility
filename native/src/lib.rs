#[macro_use]
extern crate neon;

mod du;

use neon::prelude::*;

fn dir_size(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = match cx.argument_opt(0) {
        Some(e) => {
            let foo = e.downcast::<JsNumber>().or_throw(&mut cx)?.value();
            Some(foo as usize)
        },
        None => {
            None
        }
    };
    let dir_size = du::dir_size(arg0);
    Ok(cx.string(dir_size))
}

register_module!(mut cx, {
    cx.export_function("dirSize", dir_size)
});
