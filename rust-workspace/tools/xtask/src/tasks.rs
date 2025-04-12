// SPDX-License-Identifier: {{license}}

mod check_env;

use bpaf::*;

use crate::error::TaskResult;

#[derive(Debug, Clone)]
pub enum Task {
    CheckBuildEnv {},
}

impl Task {
    pub fn get() -> Self {
        let build_env = construct!(Task::CheckBuildEnv {})
            .to_options()
            .descr("Verify that the necessary build tools are installed")
            .command("check-build-env");

        construct!([build_env])
            .to_options()
            .version(env!("CARGO_PKG_VERSION"))
            .descr(env!("CARGO_PKG_DESCRIPTION"))
            .run()
    }

    pub fn run(&self) -> TaskResult<()> {
        match self {
            Task::CheckBuildEnv {} => check_env::run()?,
        }

        Ok(())
    }
}
