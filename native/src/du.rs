// MIT License
//
// Copyright (c) 2018, diskus developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate humansize;
extern crate ignore;
extern crate num_cpus;

use std::collections::HashSet;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

use self::humansize::{file_size_opts, FileSize};
use self::ignore::WalkBuilder;

fn get_size<P: AsRef<Path>>(p: P, num_threads: usize) -> u64 {
    let mut builder = WalkBuilder::new(p);
    builder.hidden(false);
    builder.parents(false);
    builder.ignore(false);
    builder.git_global(false);
    builder.git_ignore(false);
    builder.git_exclude(false);
    builder.follow_links(false);

    builder.threads(num_threads);

    let walker = builder.build_parallel();

    let (tx, rx) = channel();

    let receiver_thread = thread::spawn(move || {
        let mut total = 0;
        let mut ids = HashSet::new();
        for (unique_id, size) in rx {
            if let Some(unique_id) = unique_id {
                // Only count this entry if the ID has not been seen
                if ids.insert(unique_id) {
                    total += size;
                }
            } else {
                total += size;
            }
        }

        total
    });

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    if let Ok(metadata) = entry.metadata() {
                        // If the entry has more than one hard link, generate
                        // a unique ID consisting of device and inode in order
                        // not to count this entry twice.
                        let unique_id = if metadata.is_file() && metadata.nlink() > 1 {
                            Some((metadata.dev(), metadata.ino()))
                        } else {
                            None
                        };

                        let size = metadata.len();

                        tx.send((unique_id, size)).ok();
                    } else {
                        eprintln!(
                            "Could not get metadata: '{}'",
                            entry.path().to_string_lossy()
                        );
                    }
                }
                Err(err) => {
                    eprintln!("I/O error: {}", err);
                }
            }

            return ignore::WalkState::Continue;
        })
    });

    drop(tx);
    receiver_thread.join().unwrap()
}

fn format_result(size: u64) -> String {
    format!(
        "{} ({} bytes)",
        size.file_size(file_size_opts::DECIMAL).unwrap(),
        size
    )
}

pub fn dir_size(thread_count: Option<usize>, dir: String) -> String {
    // Setting the number of threads to 3x the number of cores is a good tradeoff between
    // cold-cache and warm-cache runs. For a cold disk cache, we are limited by disk IO and
    // therefore want the number of threads to be rather large in order for the IO scheduler to
    // plan ahead. On the other hand, the number of threads shouldn't be too high for warm disk
    // caches where we would otherwise pay a higher synchronization overhead.
    let num_threads = match thread_count {
        Some(count) => {
            count
        },
        None => {
            3 * num_cpus::get()
        }
    };

    let size = get_size(dir, num_threads);
    format_result(size)
}