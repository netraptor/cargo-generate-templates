// SPDX-License-Identifier: {{license}}

use error_stack::Context;

pub type TaskResult<T> = error_stack::Result<T, TaskError>;

#[derive(Debug, Clone)]
pub struct TaskError(String);

impl TaskError {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<String>,
    {
        Self(msg.into())
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Context for TaskError {}
