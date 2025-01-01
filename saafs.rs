// SPDX-License-Identifier: GPL-2.0

//! Slack as a Filesystem kernel module

use kernel::prelude::*;

module! {
    type: SlackFS,
    name: "saafs",
    author: "Arthur Beck <averse.abfun@gmail.com>",
    description: "Slack As A FileSystem",
    license: "GPL",
}

struct SlackFS {
    numbers: Vec<i32>,
}

impl kernel::Module for SlackFS {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let mut numbers = Vec::new();
        numbers.push(72, GFP_KERNEL)?;
        numbers.push(108, GFP_KERNEL)?;
        numbers.push(200, GFP_KERNEL)?;

        Ok(SlackFS { numbers })
    }
}

impl Drop for SlackFS {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
