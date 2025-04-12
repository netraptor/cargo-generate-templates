// SPDX-License-Identifier: {{license}}

mod error;
mod settings;
mod tasks;
mod util;

use self::error::TaskResult;
use self::tasks::Task;

fn main() -> TaskResult<()> {
    Task::get().run()?;

    Ok(())
}
