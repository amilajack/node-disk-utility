#[macro_use]
extern crate neon;

mod du;

use neon::prelude::*;

fn dir_size(mut cx: FunctionContext) -> JsResult<JsString> {
    // First argument is the path and is required
    let path = cx.argument::<JsString>(0)?.value();

    // Second argument is the thread count and is optional
    let threads = match cx.argument_opt(1) {
        Some(e) => {
            let threads = e.downcast::<JsNumber>().or_throw(&mut cx)?.value();
            Some(threads as usize)
        },
        None => None
    };

    let dir_size = du::dir_size(threads, path);
    Ok(cx.string(dir_size))
}

register_module!(mut cx, {
    cx.export_function("dirSize", dir_size)
});
