// SPDX-License-Identifier: {{license}}

use std::path::PathBuf;

use error_stack::ResultExt;

use crate::error::{TaskError, TaskResult};

#[allow(unused)]
pub fn get_workspace_root() -> TaskResult<PathBuf> {
    cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .map(|c_data| c_data.workspace_root.as_std_path().to_path_buf())
        .change_context_lazy(|| TaskError::new("Failed to read workspace data"))
        .attach_printable("Note: Ensure this is run from within the project workspace.")
}
