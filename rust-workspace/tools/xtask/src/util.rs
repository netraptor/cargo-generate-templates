// SPDX-License-Identifier: {{license}}

use std::path::{Path, PathBuf};
use std::process::Output;

use error_stack::{ResultExt, report};

use crate::error::{TaskError, TaskResult};

#[allow(unused)]
#[derive(Debug)]
pub struct Cmd {
    program: String,
    args: Vec<String>,
    dir: Option<PathBuf>,
    os_err_msg: Option<String>,
    run_err_msg: Option<String>,
}

#[allow(unused)]
impl Cmd {
    pub fn new<T, U>(program: T, args: U) -> Self
    where
        T: Into<String>,
        U: IntoIterator,
        U::Item: Into<String>,
    {
        let program = program.into();
        let args = args.into_iter().map(Into::into).collect();

        Self {
            program,
            args,
            dir: None,
            os_err_msg: None,
            run_err_msg: None,
        }
    }

    pub fn set_working_dir<T>(self, path: T) -> Self
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();

        Self {
            dir: Some(path),
            ..self
        }
    }

    pub fn msg_on_run_err<T>(self, msg: T) -> Self
    where
        T: Into<String>,
    {
        let msg = msg.into();

        Self {
            run_err_msg: Some(msg),
            ..self
        }
    }

    pub fn msg_on_os_err<T>(self, msg: T) -> Self
    where
        T: Into<String>,
    {
        let msg = msg.into();

        Self {
            os_err_msg: Some(msg),
            ..self
        }
    }

    fn run_impl(&self, quiet: bool) -> TaskResult<Output> {
        let mut run_cmd = duct::cmd(&self.program, &self.args);

        if let Some(path) = self.dir.as_ref() {
            run_cmd = run_cmd.dir(path);
        }

        if quiet {
            run_cmd = run_cmd.stderr_capture().stdout_capture().unchecked();
        } else {
            run_cmd = run_cmd.stderr_to_stdout().unchecked();
        }

        let cmd_result = run_cmd.run().change_context_lazy(|| {
            TaskError::new(format!("OS returned error running command: {}", self))
        });

        let cmd_output = match cmd_result {
            Ok(output) => output,
            Err(e) => {
                if let Some(msg) = &self.os_err_msg {
                    return Err(e).attach_printable(msg.clone());
                } else {
                    return Err(e);
                }
            }
        };

        if !cmd_output.status.success() {
            let context = TaskError::new(format!("Error returned from command: {}", self));

            let error = if quiet {
                let err_msg = TaskError::new(String::from_utf8_lossy(&cmd_output.stderr));
                report!(err_msg).change_context(context)
            } else {
                report!(context)
            };

            if let Some(msg) = &self.run_err_msg {
                return Err(error).attach_printable(msg.clone());
            } else {
                return Err(error);
            }
        }

        Ok(cmd_output)
    }

    pub fn run(self) -> TaskResult<()> {
        let _ = self.run_impl(false)?;
        Ok(())
    }

    pub fn run_quiet(self) -> TaskResult<String> {
        let cmd_output = self.run_impl(true)?;
        let cmd_stdout = String::from_utf8_lossy(&cmd_output.stdout);
        let trimmed_output = cmd_stdout.trim().to_owned();
        Ok(trimmed_output)
    }
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.program)
        } else {
            let arg_string = self
                .args
                .iter()
                .cloned()
                .reduce(|a, b| format!("{a} {b}"))
                .unwrap_or_default();

            write!(f, "{} {}", self.program, arg_string)
        }
    }
}
