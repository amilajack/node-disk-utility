extern crate neon;
extern crate diskus;
extern crate humansize;
extern crate num_cpus;

use std::path::PathBuf;
use humansize::{file_size_opts, FileSize};
use diskus::Walk;
use neon::prelude::*;

fn format_result(size: u64) -> String {
    format!(
        "{} ({} bytes)",
        size.file_size(file_size_opts::DECIMAL).unwrap(),
        size
    )
}

fn dir_size(mut cx: FunctionContext) -> JsResult<JsString> {
    // First argument is the path and is required
    let path = cx.argument::<JsString>(0)?.value();
    let path_bufs = vec![PathBuf::from(path)];

    // Second argument is the thread count and is optional
    let thread_count = match cx.argument_opt(1) {
        Some(e) => {
            let threads = e.downcast::<JsNumber>().or_throw(&mut cx)?.value();
            Some(threads as usize)
        },
        None => None
    };

    let num_threads = match thread_count {
        Some(count) => {
            count
        },
        None => {
            3 * num_cpus::get()
        }
    };

    let walk = Walk::new(&path_bufs, num_threads);
    let size_in_bytes = walk.run();
    let dir_size = format_result(size_in_bytes);

    Ok(cx.string(dir_size))
}

register_module!(mut cx, {
    cx.export_function("dirSize", dir_size)
});
