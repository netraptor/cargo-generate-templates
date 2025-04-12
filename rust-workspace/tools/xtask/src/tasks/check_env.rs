// SPDX-License-Identifier: {{license}}

use std::io::Write;

use crate::{error::TaskResult, util::Cmd};

pub fn run() -> TaskResult<()> {
    get_tool_version("rustc")?;
    get_tool_version("cargo")?;

    println!("\n++ Build environment looks ok! ++");

    Ok(())
}

fn get_tool_version(tool_name: &str) -> TaskResult<()> {
    print!("Checking for {tool_name}...");
    std::io::stdout().flush().ok();

    let check_output = Cmd::new(tool_name, vec!["--version"])
        .msg_on_os_err(format!(
            "Note: Check that '{tool_name}' is installed and in the environment PATH variable"
        ))
        .msg_on_run_err("Note: Double-check the command syntax for getting the version")
        .run_quiet()
        .inspect_err(|_| println!("Failed!"))?;

    println!("{check_output}");
    Ok(())
}
